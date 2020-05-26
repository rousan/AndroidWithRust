pub const SERVER_STARTED: i32 = 0;
pub const INIT_DATA: i32 = 1;
pub const SEND_FILE: i32 = 2;
pub const SEND_FILE_PROGRESS: i32 = 3;
pub const SEND_FILE_DONE: i32 = 4;

pub fn to_string(what: i32) -> &'static str {
    match what {
        SERVER_STARTED => "SERVER_STARTED",
        INIT_DATA => "INIT_DATA",
        SEND_FILE => "SEND_FILE",
        SEND_FILE_PROGRESS => "SEND_FILE_PROGRESS",
        SEND_FILE_DONE => "SEND_FILE_DONE",
        _ => "<UNKNOWN>",
    }
}
