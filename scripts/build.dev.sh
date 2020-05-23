#!/bin/sh

JNI_LIBS_DIR=./app/libs

# Build only for x86 target for development purpose.
cargo build --target i686-linux-android

rm -rf $JNI_LIBS_DIR
mkdir $JNI_LIBS_DIR
mkdir $JNI_LIBS_DIR/x86

cp target/i686-linux-android/debug/librust.so $JNI_LIBS_DIR/x86/librust.so