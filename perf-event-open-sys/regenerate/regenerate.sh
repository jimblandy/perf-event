#!/usr/bin/env bash

set -eu

install_root=$1
arch=$2

script_dir=$(cd $(dirname $0); pwd)
src_dir=$(cd $script_dir/../src; pwd)
bindings=$src_dir/bindings_${arch}.rs
if ! [ -f "$bindings" ]; then
    echo "Unexpected architecture name: $arch" >&2
    echo "There should be an existing bindings file '$bindings'" >&2
    echo "If you're adding bindings for a new architecture, just say:" >&2
    echo "touch $bindings" >&2
    echo "and try again. You'll need to adjust src/lib.rs too." >&2
    exit 1
fi

if ! [ -d "$install_root/usr/include" ]; then
    echo "Not a populated install root: $install_root" >&2
    echo "Try running 'fetch-kernel-headers-fedora.sh $arch'." >&2
    exit 1
fi

wrapper_h=$script_dir/wrapper.h
if [ ! -f "$wrapper_h" ]; then
    echo "no wrapper header '$wrapper_h'" >&2
    exit 1
fi

bindings_header=$script_dir/bindings_header.rs

(
    cat "$bindings_header"
    bindgen                                     \
        --impl-debug                            \
        --with-derive-default                   \
        --no-prepend-enum-name                  \
        "$wrapper_h"                            \
        --                                      \
        -nostdinc                       \
        -isystem "$install_root/usr/include"
) > new-bindings.rs~

mv new-bindings.rs~ $bindings
