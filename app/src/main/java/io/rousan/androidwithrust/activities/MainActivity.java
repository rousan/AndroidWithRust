package io.rousan.androidwithrust.activities;

import androidx.appcompat.app.AppCompatActivity;
import io.rousan.androidwithrust.R;
import io.rousan.androidwithrust.services.WorkerService;

import android.content.Intent;
import android.os.Bundle;
import android.view.View;

public class MainActivity extends AppCompatActivity {
    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);

        this.findViewById(R.id.btn_send).setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View v) {
                Intent intent = new Intent(MainActivity.this, WorkerService.class);
                startService(intent);
            }
        });

        this.findViewById(R.id.btn_choose_file).setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View v) {
                Intent intent = new Intent(MainActivity.this, WorkerService.class);
                stopService(intent);
            }
        });

    }
}
