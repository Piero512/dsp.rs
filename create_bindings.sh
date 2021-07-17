#!/bin/bash

echo "Generating C header dsp_rs.h"
cargo test --features c-headers -- generate_headers --nocapture

if [ -z $(cargo lipo --help) ]
then
    echo "cargo lipo not found, please install it by running 'cargo install cargo-lipo'"
    echo "Skipping iOS lib because cargo-lipo wasn't found"
else
    echo "Building universal iOS library"
    cargo lipo
fi
if [ -z $(cargo ndk) ]
then
    echo "cargo ndk not found. Please install it by running 'cargo install cargo-ndk'"
else
    echo "Building cargo for Android NDK"
    cargo ndk -t armeabi-v7a -t arm64-v8a -t x86_64 -t x86 -o ./jniLibs build --release
fi

