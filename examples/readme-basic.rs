use ::method_name::named;

#[named]
fn my_super_duper_function() {
    dbg!(method_name!());
}

fn main() {
    my_super_duper_function();
}
