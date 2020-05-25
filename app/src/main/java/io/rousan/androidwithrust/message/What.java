package io.rousan.androidwithrust.message;

public final class What {
    public final static int PING = 10;
    public final static int PONG = 11;
    public final static int SUCCESS = 12;
    public final static int ERROR = 13;
    public final static int NOOP = 14;

    public final static int FILE_WRITE = 20;

    public static String toString(int what) {
        switch (what) {
            case PING: {
                return "PING";
            }
            case PONG: {
                return "PONG";
            }
            case SUCCESS: {
                return "SUCCESS";
            }
            case ERROR: {
                return "ERROR";
            }
            case NOOP: {
                return "NOOP";
            }

            case FILE_WRITE: {
                return "FILE_WRITE";
            }
        }
        return "<UNKNOWN>";
    }
}
