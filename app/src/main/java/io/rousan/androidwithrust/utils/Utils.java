package io.rousan.androidwithrust.utils;

import android.util.Log;

public final class Utils {
    public static void log(String msg) {
        Log.d("my-app", String.format("Java: %s", msg));
    }
}
