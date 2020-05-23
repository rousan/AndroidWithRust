#!/bin/sh

JNI_OUTPUT_DIR=app/src/main/jni
JAVA_PKG_PATH=io/rousan/androidwithrust
ANDROID_SDK_PATH=/Users/rousan/Library/Android/sdk

javac -h "$JNI_OUTPUT_DIR" "app/src/main/java/$JAVA_PKG_PATH/Runtime.java" -classpath "$ANDROID_SDK_PATH/platforms/android-28/android.jar"