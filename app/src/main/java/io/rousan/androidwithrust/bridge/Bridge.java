package io.rousan.androidwithrust.bridge;

import android.content.Context;
import android.os.Bundle;
import android.os.Handler;

import java.util.HashMap;

public class Bridge {
    static {
        System.loadLibrary("rust");
    }

    private Context context;
    private HashMap<String, OnEventListener> listeners;

    public Bridge(Context context) {
        this.context = context;
        this.listeners = new HashMap<>();
    }

    public void on(String eventName, OnEventListener listener) {
        this.listeners.put(eventName, listener);
    }

    public void emit(String eventName, EventData data) {
        this.emitEventToRust(eventName, data.bundle);
    }

    public void initiate() {
        this.initiateBridge();
    }

    // This method will be called from the Rust end.
    private void emitEventToAndroid(String eventName, Bundle data) {
        this.dispatchEvent(eventName, new EventData(data));
    }

    private void dispatchEvent(String eventName, final EventData data) {
        final OnEventListener listener = this.listeners.get(eventName);

        if (listener != null) {
            Handler handler = new Handler(this.context.getMainLooper());
            handler.post(new Runnable() {
                @Override
                public void run() {
                    listener.onEvent(data);
                }
            });
        }
    }

    private native void initiateBridge();

    public native void emitEventToRust(String eventName, Bundle data);
}
