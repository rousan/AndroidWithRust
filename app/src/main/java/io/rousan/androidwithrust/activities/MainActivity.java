package io.rousan.androidwithrust.activities;

import androidx.appcompat.app.AppCompatActivity;
import io.rousan.androidwithrust.R;
import io.rousan.androidwithrust.bridge.MessageData;
import io.rousan.androidwithrust.services.WorkerService;
import io.rousan.androidwithrust.utils.Utils;

import android.content.ComponentName;
import android.content.Context;
import android.content.Intent;
import android.content.ServiceConnection;
import android.os.Bundle;
import android.os.IBinder;
import android.view.View;

public class MainActivity extends AppCompatActivity {
    private WorkerService workerService;
    private boolean isBoundWithService;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);
        startService(new Intent(this, WorkerService.class));
    }

    @Override
    protected void onStart() {
        super.onStart();
        Intent intent = new Intent(this, WorkerService.class);
        bindService(intent, connection, Context.BIND_AUTO_CREATE);
    }

    @Override
    protected void onStop() {
        super.onStop();
        unbindService(connection);
    }

    public void onChooseButtonClick(View v) {
        Utils.log("Choose clicked");
        if (isBoundWithService) {
            workerService.sendMessage(900, MessageData.empty());
        }
    }

    public void onSendButtonClick(View v) {
        Utils.log("Send clicked");
        startService(new Intent(this, WorkerService.class));
    }

    private ServiceConnection connection = new ServiceConnection() {
        @Override
        public void onServiceConnected(ComponentName name, IBinder service) {
            Utils.log("Connected with service");
            workerService = ((WorkerService.WorkerServiceBinder)service).getService();
            isBoundWithService = true;
        }

        @Override
        public void onServiceDisconnected(ComponentName name) {
            Utils.log("Disconnected with service");
            workerService = null;
            isBoundWithService = false;
        }
    };
}
