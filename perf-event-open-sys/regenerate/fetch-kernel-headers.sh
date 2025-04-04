#!/usr/bin/env bash
#
# Fetch and unpack kernel headers using Fedora's dnf and rpm.
#
# Usage: sh fetch-kernel-headers.sh aarch64
#
# to fetch Fedora Core 41 and 64-bit ARM headers.

set -eu

arch=$1

script_dir=$(cd $(dirname $0); pwd)
src_dir=$(cd $script_dir/../src; pwd)
bindings=${src_dir}/bindings_${arch}.rs
if ! [ -f "$bindings" ]; then
    echo "Unexpected architecture name: $arch" >&2
    echo "There should be an existing bindings file '$bindings'" >&2
    echo "If you're adding bindings for a new architecture, just say:" >&2
    echo "touch $bindings" >&2
    echo "and try again. You'll need to adjust src/lib.rs too." >&2
    exit 1
fi

downloaded_rpms=$script_dir/downloaded-fedora-rpms
installroot=$script_dir/install-root-fedora-$arch

rm -rf "$downloaded_rpms"
sudo rm -rf "$installroot"

dnf download                                    \
    --destdir="$downloaded_rpms"                \
    --forcearch="$arch"                         \
    kernel-headers

rpm=$(echo "$downloaded_rpms/kernel-headers-"*".${arch}.rpm")
if [ ! -f "$rpm" ]; then
    echo "'dnf download' didn't seem to fetch file '$rpm'" >&2
    exit 1
fi

sudo rpm -ivh                                   \
     --root="$installroot"                      \
     --ignorearch                               \
     "$rpm"
