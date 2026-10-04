#[allow(unused_imports)]
use {
    ::core::ops::Not as _,
    ::proc_macro::{TokenStream, *},
};

#[proc_macro_attribute]
pub fn named(params: TokenStream, input: TokenStream) -> TokenStream {
    named_impl(params, input).unwrap_or_else(|err| {
        let err = Some(TokenTree::from(Literal::string(err)));
        quote!(::core::compile_error! { #err })
    })
}

fn named_impl(params: TokenStream, input: TokenStream) -> Result<TokenStream, &'static str> {
    // parse::Nothing for `params`.
    if params.into_iter().next().is_some() {
        return Err("unexpected attribute arguments");
    }

    let item_kind = input.clone().into_iter().find_map(|tt| match tt {
        TokenTree::Ident(ident) if ident.to_string() == "fn" => Some("fn"),
        TokenTree::Ident(ident) if ident.to_string() == "impl" => Some("impl"),
        _ => None,
    });
    if item_kind == Some("impl") {
        return named_impl_block(input);
    }
    if item_kind != Some("fn") {
        return Err("expected a `fn` or `impl`");
    }

    let mut tts = input.into_iter().peekable();

    let mut input = Vec::<TokenTree>::new();

    // `#` `[…]` attributes:
    while matches!(tts.peek(), Some(TokenTree::Punct(p)) if p.as_char() == '#') {
        input.extend(tts.by_ref().take(2));
    }

    // rest but scan the tt right after `fn`.
    let fname = loop {
        let tt = tts.next().ok_or("expected a `fn`")?;
        if matches!(tt, TokenTree::Ident(ref ident) if ident.to_string() == "fn") {
            input.push(tt);
            let fname = tts
                .peek()
                .ok_or("expected a function name after `fn`")?
                .to_string();
            input.extend(tts);
            break TokenTree::from(Literal::string(&fname));
        }
        input.push(tt);
    };

    let g = match input.last_mut() {
        Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Brace => g,
        _ => return Err("expected a `fn`"),
    };
    let g_span = g.span();
    let mut replacement = named_body(g, Some(fname));
    replacement.set_span(g_span);
    *g = replacement;
    Ok(input.into_iter().collect())
}

fn named_impl_block(input: TokenStream) -> Result<TokenStream, &'static str> {
    let mut input = input.into_iter().collect::<Vec<_>>();
    let impl_index = input
        .iter()
        .position(|tt| matches!(tt, TokenTree::Ident(ident) if ident.to_string() == "impl"))
        .ok_or("expected an `impl`")?;
    let body_index = (impl_index + 1..input.len())
        .rev()
        .find(|&index| {
            matches!(&input[index], TokenTree::Group(group) if group.delimiter() == Delimiter::Brace)
        })
        .ok_or("expected an `impl` body")?;
    let type_name = impl_type_name(&input[impl_index + 1..body_index])?;
    let body = match &mut input[body_index] {
        TokenTree::Group(group) => group,
        _ => unreachable!(),
    };
    let body_span = body.span();
    let mut methods = body.stream().into_iter().collect::<Vec<_>>();
    let mut index = 0;

    while index < methods.len() {
        if !matches!(&methods[index], TokenTree::Ident(ident) if ident.to_string() == "fn") {
            index += 1;
            continue;
        }

        let mut name_index = index + 1;
        if matches!(methods.get(name_index), Some(TokenTree::Punct(punct)) if punct.as_char() == '<')
        {
            name_index = skip_generics(&methods, name_index)?;
        }
        let method_name = match methods.get(name_index) {
            Some(TokenTree::Ident(ident)) => ident.to_string(),
            _ => {
                index += 1;
                continue;
            }
        };
        let method_body_index = (name_index + 1..methods.len()).find(|&candidate| {
            matches!(&methods[candidate], TokenTree::Group(group) if group.delimiter() == Delimiter::Brace)
        });
        let Some(method_body_index) = method_body_index else {
            index += 1;
            continue;
        };

        let full_name = TokenTree::from(Literal::string(&format!("{type_name}::{method_name}")));
        let method_body = match &mut methods[method_body_index] {
            TokenTree::Group(group) => group,
            _ => unreachable!(),
        };
        let method_span = method_body.span();
        let mut replacement = named_body(method_body, Some(full_name));
        replacement.set_span(method_span);
        methods[method_body_index] = TokenTree::Group(replacement);
        index = method_body_index + 1;
    }

    let mut replacement = Group::new(
        Delimiter::Brace,
        methods.into_iter().collect::<TokenStream>(),
    );
    replacement.set_span(body_span);
    *body = replacement;
    Ok(input.into_iter().collect())
}

fn impl_type_name(tokens: &[TokenTree]) -> Result<String, &'static str> {
    let self_type = if let Some(for_index) = tokens
        .iter()
        .position(|tt| matches!(tt, TokenTree::Ident(ident) if ident.to_string() == "for"))
    {
        &tokens[for_index + 1..]
    } else {
        let impl_generics_end = if matches!(tokens.first(), Some(TokenTree::Punct(punct)) if punct.as_char() == '<')
        {
            skip_generics(tokens, 0)?
        } else {
            0
        };
        &tokens[impl_generics_end..]
    };

    let mut type_name = None;
    for token in self_type {
        match token {
            TokenTree::Ident(ident) if ident.to_string() == "where" => break,
            TokenTree::Punct(punct) if punct.as_char() == '<' => break,
            TokenTree::Ident(ident) => type_name = Some(ident.to_string()),
            _ => {}
        }
    }
    type_name.ok_or("could not determine the `impl` type")
}

fn skip_generics(tokens: &[TokenTree], start: usize) -> Result<usize, &'static str> {
    let mut depth = 0;
    for (index, token) in tokens.iter().enumerate().skip(start) {
        if let TokenTree::Punct(punct) = token {
            match punct.as_char() {
                '<' => depth += 1,
                '>' => {
                    depth -= 1;
                    if depth == 0 {
                        return Ok(index + 1);
                    }
                }
                _ => {}
            }
        }
    }
    Err("unclosed generic parameter list")
}

fn named_body(group: &Group, method_name: Option<TokenTree>) -> Group {
    let mut body = quote!(
        macro_rules! method_name {() => (
            #method_name
        )}
    );
    body.extend(group.stream());
    Group::new(group.delimiter(), body)
}

/// Mini `quote!` implementation,
/// can only interpolate `impl IntoIterator<Item = TokenTree>`.
macro_rules! quote_ {
    (
        @$q:tt
        { $($code:tt)* } $($rest:tt)*
    ) => (
        $q.push(
            TokenTree::Group(Group::new(
                Delimiter::Brace,
                quote!($($code)*)
            ))
        );
        quote!(@$q $($rest)*);
    );

    (
        @$q:tt
        [ $($code:tt)* ]
        $($rest:tt)*
    ) => (
        $q.push(
            TokenTree::Group(Group::new(
                Delimiter::Bracket,
                quote!($($code)*)
            ))
        );
        quote!(@$q $($rest)*);
    );

    (
        @$q:tt
        ( $($code:tt)* )
        $($rest:tt)*
    ) => (
        $q.push(
            TokenTree::Group(Group::new(
                Delimiter::Parenthesis,
                quote!($($code)*)
            ))
        );
        quote!(@$q $($rest)*);
    );

    (
        @$q:tt
        #$var:ident
        $($rest:tt)*
    ) => (
        $q.extend($var);
        quote!(@$q $($rest)*);
    );

    (
        @$q:tt
        $tt:tt $($rest:tt)*
    ) => (
        $q.extend(
            stringify!($tt)
                .parse::<TokenStream>()
                .unwrap()
        );
        quote!(@$q $($rest)*);
    );

    (
        @$q:tt
        /* nothign left */
    ) => ();

    (
        $($code:tt)*
    ) => ({
        let mut _q = Vec::<TokenTree>::new();
        quote!(@_q $($code)*);
        _q.into_iter().collect::<TokenStream>()
    });
}
use quote_ as quote;
