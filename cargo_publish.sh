#!/usr/bin/env bash

set -euxo pipefail

(cd proc-macro
    cargo +stable publish
)

for attempt in $(seq 10)
do
    if cargo +stable publish; then
        exit 0
    fi
    if [[ "$attempt" -lt 10 ]]; then
        sleep 5
    fi
done

echo "Failed to publish method_name after 10 attempts" >&2
exit 1
