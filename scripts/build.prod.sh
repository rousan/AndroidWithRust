#!/bin/sh

JNI_LIBS_DIR=./app/libs

cargo build --target aarch64-linux-android --release
cargo build --target armv7-linux-androideabi --release
cargo build --target i686-linux-android --release

rm -rf $JNI_LIBS_DIR
mkdir $JNI_LIBS_DIR
mkdir $JNI_LIBS_DIR/arm64-v8a
mkdir $JNI_LIBS_DIR/armeabi-v7a
mkdir $JNI_LIBS_DIR/x86

cp target/aarch64-linux-android/release/librust.so $JNI_LIBS_DIR/arm64-v8a/librust.so
cp target/armv7-linux-androideabi/release/librust.so $JNI_LIBS_DIR/armeabi-v7a/librust.so
cp target/i686-linux-android/release/librust.so $JNI_LIBS_DIR/x86/librust.so