package io.rousan.androidwithrust.message;

public final class What {
    public final static int INITIATE = 0;
    public final static int COUNTER_VALUE = 1;
    public final static int INCREASE_COUNTER = 2;
    public final static int DECREASE_COUNTER = 3;

    public static String toString(int what) {
        switch (what) {
            case INITIATE: {
                return "INITIATE";
            }
            case COUNTER_VALUE: {
                return "COUNTER_VALUE";
            }
            case INCREASE_COUNTER: {
                return "INCREASE_COUNTER";
            }
            case DECREASE_COUNTER: {
                return "DECREASE_COUNTER";
            }
            default: {
                return "<UNKNOWN>";
            }
        }
    }
}