use std::io::prelude::*;

pub struct FileUtils {}

impl FileUtils {

    /// # Usage
    /// takes a string line and the path to our time_tracker.log file and creates a new
    /// file if one does not exist, it appends if the file does exist. If we are 
    /// successful, we write to the file.
    pub fn write_to_log_file(line: &str, path: &std::path::Path) {
        match std::fs::OpenOptions::new().read(true).create(true).append(true).open(path) {
            Ok(mut s) => s.write_all(line.as_bytes()).expect("Error writing to file"), 
            Err(why) => panic!("{}", why),
        }
    }

    /// # Usage
    /// print_logs takes a path input and attempts to open the file. We use a 
    /// match statement and set file with the results from Ok(). We panic if
    /// there is a problem opening the file, dumping the reason for the panic
    /// to the screen.
    /// 
    /// With the file value set, we them read it to a string and then output
    /// to the screen, panicing and printing why if we fail.
    pub fn print_logs(path: &std::path::Path) {
        let mut buffer = String::new();

        let mut file = match std::fs::OpenOptions::new().read(true).open(path) {
            Ok(file) => file,
            Err(why) => panic!("{}", why),
        };
        
        match file.read_to_string(&mut buffer) {
            Ok(_s) => println!("{}", buffer),
            Err(why) => panic!("{}", why),
        }
    }
}