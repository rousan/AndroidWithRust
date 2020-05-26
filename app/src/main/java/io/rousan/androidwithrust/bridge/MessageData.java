package io.rousan.androidwithrust.bridge;

import android.os.Bundle;

public class MessageData {
    public static MessageData empty() {
        return new MessageData();
    }

    Bundle bundle;

    public MessageData() {
        this.bundle = new Bundle();
    }

    public MessageData(Bundle bundle) {
        this.bundle = bundle;
    }

    public void putString(String key, String value) {
        this.bundle.putString(key, value);
    }

    public String getString(String key) {
        return this.bundle.getString(key);
    }

    public Bundle getData() {
        return bundle;
    }
}
