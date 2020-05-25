package io.rousan.androidwithrust.services;

import android.app.Service;
import android.content.Intent;
import android.os.IBinder;
import android.util.Log;

import io.rousan.androidwithrust.bridge.MessageData;
import io.rousan.androidwithrust.bridge.Bridge;
import io.rousan.androidwithrust.utils.Utils;

public class WorkerService extends Service {
    private Bridge bridge;

    public WorkerService() {
        this.bridge = new Bridge(this, new Bridge.OnMessageListener() {
            @Override
            public void onMessage(int what, MessageData data) {
                Utils.log(String.format("Got a message: what: %s", what + ""));
            }
        });
    }

    @Override
    public void onCreate() {
        Utils.log("Worker Service created");
        this.bridge.start();
    }

    @Override
    public int onStartCommand(Intent intent, int flags, int startId) {
        Utils.log("Worker Service onStartCommand");
        return START_NOT_STICKY;
    }

    @Override
    public IBinder onBind(Intent intent) {
        // TODO: Return the communication channel to the service.
        throw new UnsupportedOperationException("Not yet implemented");
    }

    @Override
    public void onDestroy() {
        Utils.log("Worker Service destroyed");
        this.bridge.shutdown();
    }
}
