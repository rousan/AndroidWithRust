package io.rousan.androidwithrust;

import androidx.appcompat.app.AppCompatActivity;
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
                MessageData data = new MessageData();
                data.putString("key", "hello: 象形字 ");

                worker.sendMessage(100, data);
            }
        });
    }

    private void initWorker() {
        this.worker = new Worker(this, new Worker.OnMessageListener() {
            @Override
            public void onMessage(int what, MessageData data) {
                Log.d("rust", String.format("From java, what: %d, data: %s", what, data.getString("key")));
                Log.d("rust", String.format("From java2, what: %d, data: %s", what, data.getString("key2")));
            }
        });

        this.worker.start();
    }
}
