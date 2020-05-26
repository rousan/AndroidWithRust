package io.rousan.androidwithrust.services;

import android.app.Service;
import android.content.Intent;
import android.os.Handler;
import android.os.IBinder;
import android.os.Message;
import android.os.Messenger;
import android.os.RemoteException;

import java.io.IOException;

import io.rousan.androidwithrust.activities.MainActivity;
import io.rousan.androidwithrust.bridge.MessageData;
import io.rousan.androidwithrust.bridge.Bridge;
import io.rousan.androidwithrust.message.What;
import io.rousan.androidwithrust.utils.Utils;

public class WorkerService extends Service {
    private Bridge bridge;
    private Messenger receiveMessenger;
    private Messenger sendMessenger;

    public WorkerService() {
        this.bridge = new Bridge(this, new Bridge.OnMessageListener() {
            @Override
            public void onMessage(int what, MessageData data) {
                if (what == What.SERVER_STARTED) {
                    try {
                        MessageData init_data = new MessageData();
                        init_data.putString("output_path", getExternalFilesDir(null).getCanonicalPath());
                        sendMessage(What.INIT_DATA, init_data);
                        return;
                    } catch (Exception exp) {
                        Utils.log(exp);
                    }
                }

                if (sendMessenger != null) {
                    Message message = Message.obtain(null, what);
                    message.setData(data.getData());

                    try {
                        sendMessenger.send(message);
                    } catch (RemoteException e) {
                        e.printStackTrace();
                    }
                }
            }
        });
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
        receiveMessenger = new Messenger(new IncomingHandler(this));
        return receiveMessenger.getBinder();
    }

    @Override
    public boolean onUnbind(Intent intent) {
        Utils.log("Worker Service onUnbind");
        sendMessenger = null;
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

    public static class IncomingHandler extends Handler {
        private WorkerService workerService;

        IncomingHandler(WorkerService workerService) {
            this.workerService = workerService;
        }

        @Override
        public void handleMessage(Message msg) {
            if (msg.what == MainActivity.WHAT_SET_REPLY_MESSENGER) {
                workerService.sendMessenger = msg.replyTo;
                return;
            }

            workerService.bridge.sendMessage(msg.what, new MessageData(msg.getData()));
        }
    }
}
