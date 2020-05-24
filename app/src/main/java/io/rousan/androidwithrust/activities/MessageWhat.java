package io.rousan.androidwithrust.activities;

public final class MessageWhat {
    public final static int PING = 0;
    public final static int PONG =  1;

    public static String toString(int what) {
        switch (what) {
            case PING: {
                return "PING";
            }
            case PONG: {
                return "PONG";
            }
        }
        return "<UNKNOWN>";
    }
}
