package io.rousan.androidwithrust.worker;

import android.os.Bundle;

public class MessageData {
    Bundle bundle;

    public MessageData() {
        this.bundle = new Bundle();
    }

    MessageData(Bundle bundle) {
        this.bundle = bundle;
    }

    public void putString(String key, String value) {
        this.bundle.putString(key, value);
    }

    public String getString(String key) {
        return this.bundle.getString(key);
    }
}
