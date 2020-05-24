pub const PING: i32 = 0;
pub const PONG: i32 = 1;
pub const NOOP: i32 = -1;

pub fn to_string(what: i32) -> &'static str {
    match what {
        PING => "PING",
        PONG => "PONG",
        NOOP => "NOOP",
        _ => "<UNKNOWN>",
    }
}
