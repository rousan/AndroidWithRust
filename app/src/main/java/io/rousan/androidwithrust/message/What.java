package io.rousan.androidwithrust.message;

public final class What {
    public final static int SERVER_STARTED = 0;
    public final static int INIT_DATA = 1;
    public final static int SEND_FILE = 2;
    public final static int SEND_FILE_PROGRESS = 3;
    public final static int SEND_FILE_DONE = 4;

    public static String toString(int what) {
        switch (what) {
            case SERVER_STARTED: {
                return "SERVER_STARTED";
            }
            case INIT_DATA: {
                return "INIT_DATA";
            }
            case SEND_FILE: {
                return "SEND_FILE";
            }
            case SEND_FILE_PROGRESS: {
                return "SEND_FILE_PROGRESS";
            }
            case SEND_FILE_DONE: {
                return "SEND_FILE_DONE";
            }
            default: {
                return "<UNKNOWN>";
            }
        }
    }
}