package io.rousan.androidwithrust.activities;

import androidx.annotation.Nullable;
import androidx.appcompat.app.AppCompatActivity;
import io.rousan.androidwithrust.R;
import io.rousan.androidwithrust.bridge.MessageData;
import io.rousan.androidwithrust.message.What;
import io.rousan.androidwithrust.services.WorkerService;
import io.rousan.androidwithrust.utils.Utils;

import android.content.ComponentName;
import android.content.Context;
import android.content.DialogInterface;
import android.content.Intent;
import android.content.ServiceConnection;
import android.os.Bundle;
import android.os.Environment;
import android.os.Handler;
import android.os.IBinder;
import android.os.Message;
import android.os.Messenger;
import android.os.RemoteException;
import android.view.View;
import android.widget.TextView;

import com.obsez.android.lib.filechooser.ChooserDialog;

import java.io.File;

public class MainActivity extends AppCompatActivity {
    public static final int WHAT_SET_REPLY_MESSENGER = Integer.MAX_VALUE;
    public static final int CHOOSE_FILE_REQUEST_CODE = 100;

    private boolean isBoundWithService;
    private Messenger sendMessenger;
    private Messenger receiveMessenger;

    private String chose_file_path;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);

        startService(new Intent(this, WorkerService.class));
        bindService(new Intent(this, WorkerService.class), serviceConnection, Context.BIND_AUTO_CREATE);

        TextView tv = (TextView) findViewById(R.id.tv_ip);
        tv.setText(String.format("IP: %s", Utils.getIPAddress(true)));
    }

    @Override
    protected void onDestroy() {
        super.onDestroy();
        unbindService(serviceConnection);
    }

    @Override
    protected void onActivityResult(int requestCode, int resultCode, @Nullable Intent data) {
        super.onActivityResult(requestCode, resultCode, data);

        if (requestCode == CHOOSE_FILE_REQUEST_CODE) {
            String path = data.getData().getPath();
            Utils.log("Choosen file: " + path);
            this.chose_file_path = path;
        }
    }

    public void onChooseButtonClick(View v) {
        Utils.log("Choose clicked");

        Intent intent = new Intent(Intent.ACTION_GET_CONTENT);
        intent.setType("file/*");
        startActivityForResult(intent, CHOOSE_FILE_REQUEST_CODE);
    }

    public void onSendButtonClick(View v) {
        Utils.log("Send clicked");

        MessageData data = new MessageData();
        data.putString("ip", "192.168.0.109");
        data.putString("path", chose_file_path);

        sendMessage(What.SEND_FILE, data);
    }

    public void onBridgeMessage(int what, MessageData data) {
        Utils.log(String.format("Got a message: what: %s", What.toString(what)));
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
}
