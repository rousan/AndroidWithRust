package io.rousan.androidwithrust.utils;

import android.util.Log;

public final class Utils {
    public static void log(Object msg) {
        Log.d("my-app", String.format("Java: %s", msg));
    }
}
