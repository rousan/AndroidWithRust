package io.rousan.androidwithrust.activities;

import androidx.appcompat.app.AppCompatActivity;
import io.rousan.androidwithrust.R;
import io.rousan.androidwithrust.worker.MessageData;
import io.rousan.androidwithrust.worker.Worker;

import android.os.Bundle;
import android.util.Log;
import android.view.View;

public class MainActivity extends AppCompatActivity {
    private Worker worker;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);
        initWorker();

        this.findViewById(R.id.btn).setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View v) {
                worker.sendMessage(MessageWhat.PING, MessageData.empty());
            }
        });
    }

    private void initWorker() {
        this.worker = new Worker(this, new Worker.OnMessageListener() {
            @Override
            public void onMessage(int what, MessageData data) {
                MainActivity.this.onMessage(what, data);
            }
        });

        this.worker.start();
    }

    private void onMessage(int what, MessageData data) {
        Log.d("my-app", String.format("Java: Got a message: what: %s", MessageWhat.toString(what)));

        switch (what) {
            case MessageWhat.PONG: {
                Log.d("my-app", "Java: Got pong reply from Rust end");
            }
        }
    }
}
