#!/usr/bin/env bash

clang_version=$1

if [ -z "$clang_version" ]; then
    echo "  usage: ./bindgen.sh <clang_version>"
    echo "example: ./bindgen.sh 5.0.0"
    echo "Check your current version with \`clang -v\`."
    exit 1
fi

set -euxo pipefail

bindgen wrapper.h \
    -o src/ogc.rs \
    --rust-target nightly \
    --use-core \
    --distrust-clang-mangling \
    --no-layout-tests \
    --ctypes-prefix "::libc" \
    --no-prepend-enum-name \
    --generate "functions,types,vars" \
    --blacklist-type "u(8|16|32|64|128)" \
    --blacklist-type "i(8|16|32|64|128)" \
    --blacklist-type "f(32|64)" \
    -- \
    --target=powerpc-none-eabi \
    --sysroot=$DEVKITPPC/powerpc-eabi \
    -isystem$DEVKITPPC/powerpc-eabi/include \
    -isystem/usr/lib/clang/$clang_version/include \
    -I$DEVKITPRO/libogc/include \
    -mfloat-abi=hard \
    -march=powerpc \
    -nostdinc \
    -Wno-macro-redefined \
    -Wno-incompatible-library-redeclaration \
    -DHW_RVL
