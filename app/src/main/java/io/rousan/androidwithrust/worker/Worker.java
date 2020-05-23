package io.rousan.androidwithrust.worker;

import android.content.Context;
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

    public  void sendMessage(String what, String data) {
        this.sendMessageToWorker(what, data);
    }

    private native void startRustWorker();

    private native void sendMessageToWorker(String what, String data);

    // It will be called from the Rust end to send a message.
    public void sendMessageToJava(final String what, final String data) {
        Handler handler = new Handler(this.context.getMainLooper());
        handler.post(new Runnable() {
            @Override
            public void run() {
                listener.onMessage(what, data);
            }
        });
    }

    public interface OnMessageListener {
        void onMessage(String what, String data);
    }
}
