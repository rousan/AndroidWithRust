pub const PING: i32 = 10;
pub const PONG: i32 = 11;
pub const SUCCESS: i32 = 12;
pub const ERROR: i32 = 13;
pub const NOOP: i32 = 14;

pub const FILE_WRITE: i32 = 20;

pub fn to_string(what: i32) -> &'static str {
    match what {
        PING => "PING",
        PONG => "PONG",
        SUCCESS => "SUCCESS",
        ERROR => "ERROR",
        NOOP => "NOOP",

        FILE_WRITE => "FILE_WRITE",
        _ => "<UNKNOWN>",
    }
}
