package io.rousan.androidwithrust.bridge;

import android.os.Bundle;

public class EventData {
    Bundle bundle;

    public EventData() {
        this.bundle = new Bundle();
    }

    EventData(Bundle bundle) {
        this.bundle =  bundle;
    }

    public void putString(String key, String value) {
        this.bundle.putString(key, value);
    }

    public String getString(String key) {
        return this.bundle.getString(key);
    }
}
