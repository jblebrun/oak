#!/bin/bash

set -o xtrace
set -o errexit
set -o nounset
set -o pipefail

mkdir --parent target

# build the orchestrator binary
cargo build --package=oak_containers_orchestrator --release --target=x86_64-unknown-linux-musl -Z unstable-options --out-dir=./target
cargo build --package=oak_containers_syslogd --release -Z unstable-options --out-dir=./target

# We need to patch the binary to set the interpreter to the correct location, but we can't do it in-place, as that would
# confuse cargo. Therefore we copy the binary to a new location and patch that.
cp ./target/oak_containers_syslogd ./target/oak_containers_syslogd_patched

# When built under nix the interpreter points to some Nix-specific location that doesn't exist on a regular Linux host, therefore
# we need to manually patch the binary to set it back to the normal regular location.
patchelf --set-interpreter /lib64/ld-linux-x86-64.so.2 ./target/oak_containers_syslogd_patched

# Fix the file permissions that will be loaded into the system image, as Git doesn't track them.
# Unfortunately we can't do it in Dockerfile (with `COPY --chown`), as that requires BuildKit.
chmod --recursive a+rX files/

bazel build runtime_bundle
