package io.rousan.androidwithrust.bridge;

import android.content.Context;
import android.os.Bundle;
import android.os.Handler;

public class Bridge {
    static {
        System.loadLibrary("rust");
    }

    private Context context;
    private OnMessageListener listener;

    public Bridge(Context context, OnMessageListener listener) {
        this.context = context;
        this.listener = listener;
    }

    public void start() {
        this.startNativeBridge();
    }

    public void shutdown() {
        this.shutdownNativeBridge();
    }

    public void sendMessage(int what, MessageData data) {
        this.sendMessageToNativeBridge(what, data.bundle);
    }

    private native void startNativeBridge();

    private native void shutdownNativeBridge();

    private native void sendMessageToNativeBridge(int what, Bundle bundle);

    // It will be called from the Rust end to send a message to Java end.
    private void sendMessageToJava(final int what, final Bundle bundle) {
        Handler handler = new Handler(this.context.getMainLooper());
        handler.post(new Runnable() {
            @Override
            public void run() {
                listener.onMessage(what, new MessageData(bundle));
            }
        });
    }

    public interface OnMessageListener {
        void onMessage(int what, MessageData data);
    }
}
