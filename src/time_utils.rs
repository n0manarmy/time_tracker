pub struct TimeUtils {}

impl TimeUtils {

    const TIME_FORMAT_STAMP: &'static str = "%Y-%m-%d %H%M";
    const TIME_FORMAT_SECONDS: &'static str = "%Y-%m-%d %H:%M:%S";

    /// # Usage
    /// provided a label and a path, we build a string to be sent to our log file.
    pub fn get_time_stamp(label: &str) -> String {
        // String that will be output to the file
        [&Self::get_time(Self::TIME_FORMAT_STAMP), " ", &label].concat()
        // call our output program to write the string with our output and the path
        // FileUtils::write_to_log_file(&log_out, &path);
    }

    pub fn get_current_time() -> String {
        Self::get_time(Self::TIME_FORMAT_SECONDS)
    }

    /// # Usage
    /// Builds a string value based on our const TIME_FORMAT
    pub fn get_time(time_format: &str) -> String {
        let time_stamp: String = chrono::Local::now().format(time_format).to_string();

        time_stamp
    }
}