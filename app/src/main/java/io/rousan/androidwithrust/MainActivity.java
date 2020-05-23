package io.rousan.androidwithrust;

import androidx.appcompat.app.AppCompatActivity;

import android.os.Bundle;
import android.view.View;

public class MainActivity extends AppCompatActivity {
    private Runtime runtime;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);

        this.runtime = new Runtime();
        this.runtime.start();

        this.findViewById(R.id.btn).setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View v) {
                // MainActivity.this.runtime.sendMessage(1, "Hi");
                Bundle data = new Bundle();
                data.putString("abcd", "hey bundle from java");
                MainActivity.this.runtime.sendMessage2(3, data);
            }
        });
    }
}
