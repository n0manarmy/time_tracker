pub struct TimeObject {
    pub date: String,
    pub time_stamp: String,
}

impl TimeObject {

    pub fn new() -> TimeObject {
        TimeObject {
            date: "".to_string(),
            time_stamp: "".to_string(),
        }
    }
}