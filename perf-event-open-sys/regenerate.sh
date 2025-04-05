#!/usr/bin/env bash

set -eu

cd "$(dirname "$0")"

# Adding a new architecture
# =========================
# In order to add bindings for a new architecture you will need the arch name
# as used in
# - the linux architecture name,
# - the rustc target triple, and,
# - the clang target triple.
#
# For some architectures (e.g. x86_64) this will all be the same. For others,
# (e.g. RISC-V) they will all be different.
#
# Here's how to find each one:
# - For rustc, look at the supported target triples in the documentation here:
#   https://doc.rust-lang.org/nightly/rustc/platform-support.html
# - Linux has its own architecture naming scheme. See the page below for a
#   command which generates a list:
#   https://docs.kernel.org/kbuild/headers_install.html
# - For Clang, there doesn't seem to be one place that lists all of the
#   supported architectures. It may be necessary to do some research here.
#   A small list is available at:
#   https://clang.llvm.org/docs/CrossCompilation.html#target-triple
#
# Once you have these, add a new gen_bindings command that passes in the arch
# names.
#
# As an example, the command for riscv64 would look like this:
#
#    gen_bindings riscv64gc riscv riscv64
#
# Updating the linux kernel version
# =================================
# The full kernel version used is specified right here:
version=6.2.10
#
# In order to generate bindings for a different kernel change the version and
# rerun the script. See https://kernel.org/ to find available kernel versions.
# This script will take care of downloading the new kernel tarball, extracting
# it, and then using that kernel to generate the bindings.

series="v$(echo $version | cut -d . -f 1).x"

scriptdir="$PWD"
targetdir="$(cargo metadata --format-version 1 | jq -r .target_directory)"
target="$targetdir/linux"

mkdir -p "$target"

if ! [ -f "$target/linux-$version.tar.xz" ]; then
    wget "https://cdn.kernel.org/pub/linux/kernel/$series/linux-$version.tar.xz" \
        -O "$target/linux-$version.tar.xz"
fi

if ! [ -d "$target/linux-$version" ]; then
    tar xf "$target/linux-$version.tar.xz" -C "$target"
fi

function gen_bindings {
    arch="$1"
    linux_arch="${2:-$arch}"
    clang_arch="${3:-$arch}"

    bindings="$target/$arch/bindings.rs"

    echo "Generating $arch bindings"

    rm -rf "${target:?}/$arch"
    mkdir -p "$target/$arch"
    cd "$target/linux-$version"
    make headers_install ARCH="$linux_arch" INSTALL_HDR_PATH="$target/$arch" > /dev/null
    cd "$scriptdir"

    CLANG_ARGS=(
        -target "$clang_arch-unknown-linux-gnu"
        -nostdlibinc
        -isystem "$target/$arch/include"
    )

    # This ensures we get errors from clang instead of bindgen panicking with
    # no useful error message.
    #
    # We don't actually use the output from here though.
    clang "${CLANG_ARGS[@]}"                    \
        -E wrapper.h                            \
        -o "$target/$arch/wrapper.i"

    bindgen                                     \
        --impl-debug                            \
        --with-derive-default                   \
        --no-prepend-enum-name                  \
        --output "$bindings"                    \
        wrapper.h                               \
        --                                      \
        "${CLANG_ARGS[@]}"

    cat src/bindings_header.rs      \
        "$bindings"                 \
        > "src/bindings_$arch.rs"
}

echo "$version." > src/version

gen_bindings x86_64
gen_bindings aarch64 arm64
gen_bindings risc64gc riscv riscv64
