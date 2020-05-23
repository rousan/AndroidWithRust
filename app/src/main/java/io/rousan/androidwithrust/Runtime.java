package io.rousan.androidwithrust;

import android.os.Bundle;
import android.util.Log;

public final class Runtime {
    static {
        System.loadLibrary("rust");
    }

    public native void start();

    public native void sendMessage(int id, String data);

    public native void sendMessage2(int id, Bundle data);

    public void handleMessage(int id, String data) {
        Log.d("rust", String.format("From java, id: %d, data: %s", id, data));
    }

    public void handleMessage2(int id, Bundle data) {
        Log.d("rust", String.format("From java, id: %d, data: %s", id, data.getString("abcd")));
    }
}
