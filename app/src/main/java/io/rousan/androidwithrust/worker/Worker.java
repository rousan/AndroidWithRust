package io.rousan.androidwithrust.worker;

import android.content.Context;
import android.os.Bundle;
import android.os.Handler;

public class Worker {
    static {
        System.loadLibrary("rust");
    }

    private Context context;
    private OnMessageListener listener;

    public Worker(Context context, OnMessageListener listener) {
        this.context = context;
        this.listener = listener;
    }

    public void start() {
        this.startRustWorker();
    }

    public void sendMessage(int what, MessageData data) {
        this.sendMessageToWorker(what, data.bundle);
    }

    private native void startRustWorker();

    private native void sendMessageToWorker(int what, Bundle bundle);

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
