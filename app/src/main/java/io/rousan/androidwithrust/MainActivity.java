package io.rousan.androidwithrust;

import androidx.appcompat.app.AppCompatActivity;
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
                worker.sendMessage("ping", "hello");
            }
        });
    }

    private void initWorker() {
        this.worker = new Worker(this, new Worker.OnMessageListener() {
            @Override
            public void onMessage(String what, String data) {
                Log.d("rust", String.format("From java, what: %s, data: %s", what, data));
            }
        });

        this.worker.start();
    }
}
