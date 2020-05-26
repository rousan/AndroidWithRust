package io.rousan.androidwithrust.activities;

import android.content.ComponentName;
import android.content.Context;
import android.content.Intent;
import android.content.ServiceConnection;
import android.os.Bundle;
import android.os.Handler;
import android.os.IBinder;
import android.os.Message;
import android.os.Messenger;
import android.os.RemoteException;
import android.view.View;
import android.widget.TextView;

import androidx.appcompat.app.AppCompatActivity;
import io.rousan.androidwithrust.R;
import io.rousan.androidwithrust.bridge.MessageData;
import io.rousan.androidwithrust.message.What;
import io.rousan.androidwithrust.services.WorkerService;
import io.rousan.androidwithrust.utils.Utils;

public class MainActivity extends AppCompatActivity {
    public static final int WHAT_SET_REPLY_MESSENGER = Integer.MAX_VALUE;

    private boolean isBoundWithService;
    private Messenger sendMessenger;
    private Messenger receiveMessenger;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);

        startService(new Intent(this, WorkerService.class));
        bindService(new Intent(this, WorkerService.class), serviceConnection, Context.BIND_AUTO_CREATE);
    }

    public void onBridgeMessage(int what, MessageData data) {
        Utils.log(String.format("Got a message: what: %s", What.toString(what)));

        switch (what) {
            case What.COUNTER_VALUE: {
                final String counter_value = data.getString("value");
                runOnUiThread(new Runnable() {
                    @Override
                    public void run() {
                        TextView tv = (TextView)findViewById(R.id.tv_counter_val);
                        tv.setText(counter_value);
                    }
                });
            }
        }
    }

    public void sendMessage(int what, MessageData data) {
        if (isBoundWithService) {
            Message msg = Message.obtain(null, what);
            msg.setData(data.getData());

            try {
                sendMessenger.send(msg);
            } catch (RemoteException exp) {
                Utils.log(exp);
            }
        }
    }

    public void onIncreaseButtonClick(View v) {
        sendMessage(What.INCREASE_COUNTER, MessageData.empty());
    }

    public void onDecreaseButtonClick(View v) {
        sendMessage(What.DECREASE_COUNTER, MessageData.empty());
    }

    private ServiceConnection serviceConnection = new ServiceConnection() {
        @Override
        public void onServiceConnected(ComponentName name, IBinder service) {
            Utils.log("Connected with service");

            sendMessenger = new Messenger(service);
            receiveMessenger = new Messenger(new IncomingHandler(MainActivity.this));
            isBoundWithService = true;

            try {
                Message msg = Message.obtain(null, WHAT_SET_REPLY_MESSENGER);
                msg.replyTo = receiveMessenger;
                sendMessenger.send(msg);
            } catch (RemoteException exp) {
                Utils.log(exp);
            }

            sendMessage(What.INITIATE, MessageData.empty());
        }

        @Override
        public void onServiceDisconnected(ComponentName name) {
            Utils.log("Disconnected with service");

            sendMessenger = null;
            receiveMessenger = null;
            isBoundWithService = false;
        }
    };

    public static class IncomingHandler extends Handler {
        private MainActivity mainActivity;

        IncomingHandler(MainActivity mainActivity) {
            this.mainActivity = mainActivity;
        }

        @Override
        public void handleMessage(Message msg) {
            mainActivity.onBridgeMessage(msg.what, new MessageData(msg.getData()));
        }
    }

    @Override
    protected void onDestroy() {
        super.onDestroy();
        unbindService(serviceConnection);
    }
}
