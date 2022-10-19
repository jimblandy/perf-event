#!/usr/bin/env bash

set -eu

cd $(dirname $0)

arch=$(uname -m)
bindings="src/bindings_${arch}.rs"
if ! [ -f "$bindings" ]; then
    echo "Unexpected architecture name: $arch" >&2
    echo "There should be an existing bindings file '$bindings'" >&2
    echo "If you're adding bindings for a new architecture, just say:" >&2
    echo "touch $bindings" >&2
    echo "and try again. You'll need to adjust src/lib.rs too." >&2
    exit 1
fi

(
    cat src/bindings_header.rs
    bindgen                                     \
        --impl-debug                            \
        --with-derive-default                   \
        --no-prepend-enum-name                  \
        wrapper.h
) > new-bindings.rs~

mv new-bindings.rs~ $bindings
