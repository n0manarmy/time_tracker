pub struct TimeUtils {}

impl TimeUtils {

    const TIME_FORMAT: &'static str = "%Y-%m-%d %H%M";

    /// # Usage
    /// provided a label and a path, we build a string to be sent to our log file.
    pub fn get_time_stamp(label: &str) -> String {
        // String that will be output to the file
        let log_out = [&Self::get_time(), " ", &label].concat();
        // call our output program to write the string with our output and the path
        // FileUtils::write_to_log_file(&log_out, &path);
        log_out
    }

    /// # Usage
    /// Builds a string value based on our const TIME_FORMAT
    pub fn get_time() -> String {
        let time_stamp: String = chrono::Local::now().format(Self::TIME_FORMAT).to_string();

        time_stamp
    }
}