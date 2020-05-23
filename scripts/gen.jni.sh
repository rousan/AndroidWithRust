#!/bin/sh

JNI_OUTPUT_DIR=app/src/main/jni
JAVA_PKG_PATH=io/rousan/androidwithrust
ANDROID_SDK_PATH=/Users/rousan/Library/Android/sdk

javac -h "$JNI_OUTPUT_DIR" -classpath "$ANDROID_SDK_PATH/platforms/android-28/android.jar:app/build/intermediates/javac/debug/classes" "app/src/main/java/$JAVA_PKG_PATH/worker/Worker.java"
rm -rf app/src/main/java/$JAVA_PKG_PATH/worker/*.class