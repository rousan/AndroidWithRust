package io.rousan.androidwithrust.services;

import android.app.Service;
import android.content.Intent;
import android.os.Binder;
import android.os.IBinder;

import io.rousan.androidwithrust.bridge.MessageData;
import io.rousan.androidwithrust.bridge.Bridge;
import io.rousan.androidwithrust.utils.Utils;

public class WorkerService extends Service {
    private Bridge bridge;
    private WorkerServiceBinder binder;

    public WorkerService() {
        this.bridge = new Bridge(this, new Bridge.OnMessageListener() {
            @Override
            public void onMessage(int what, MessageData data) {
                Utils.log(String.format("Got a message: what: %s", what + ""));
            }
        });
        this.binder = new WorkerServiceBinder();
    }

    @Override
    public void onCreate() {
        super.onCreate();
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
        Utils.log("Worker Service onBind");
        return this.binder;
    }

    @Override
    public boolean onUnbind(Intent intent) {
        Utils.log("Worker Service onUnbind");
        return true;
    }

    @Override
    public void onRebind(Intent intent) {
        super.onRebind(intent);
        Utils.log("Worker Service onRebind");
    }

    @Override
    public void onDestroy() {
        super.onDestroy();
        Utils.log("Worker Service destroyed");
        this.bridge.shutdown();
    }

    public void sendMessage(int what, MessageData data) {
        this.bridge.sendMessage(what, data);
    }

    public class WorkerServiceBinder extends Binder {
        public WorkerService getService() {
            return WorkerService.this;
        }
    }
}
