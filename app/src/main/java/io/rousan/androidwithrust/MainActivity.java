package io.rousan.androidwithrust;

import androidx.appcompat.app.AppCompatActivity;
import io.rousan.androidwithrust.bridge.Bridge;
import io.rousan.androidwithrust.bridge.EventData;
import io.rousan.androidwithrust.bridge.OnEventListener;

import android.os.Bundle;
import android.util.Log;
import android.view.View;

public class MainActivity extends AppCompatActivity {
    private Bridge bridge;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);
        initialize_bridge();

        this.findViewById(R.id.btn).setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View v) {
                EventData data = new EventData();
                data.putString("msg", "Hello");

                MainActivity.this.bridge.emit("ping", data);
            }
        });
    }

    private void initialize_bridge() {
        Bridge bridge = new Bridge(this);

        bridge.on("ping", new OnEventListener() {
            @Override
            public void onEvent(EventData data) {
                Log.d("rust", String.format("Java: Fired ping event with message: %s", data.getString("msg")));
            }
        });

        this.bridge = bridge;

        this.bridge.initiate();
    }
}
