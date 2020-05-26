pub const INITIATE: i32 = 0;
pub const COUNTER_VALUE: i32 = 1;
pub const INCREASE_COUNTER: i32 = 2;
pub const DECREASE_COUNTER: i32 = 3;

pub fn to_string(what: i32) -> &'static str {
    match what {
        INITIATE => "INITIATE",
        COUNTER_VALUE => "COUNTER_VALUE",
        INCREASE_COUNTER => "INCREASE_COUNTER",
        DECREASE_COUNTER => "DECREASE_COUNTER",
        _ => "<UNKNOWN>",
    }
}
