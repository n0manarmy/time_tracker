extern crate chrono;
use std::io::prelude::*;

const LABEL_IN: &'static str = "IN";
const LABEL_OUT: &'static str = "OUT";
const LOG_FILE: &'static str = "./time_tracker.log";
const TIME_FORMAT: &'static str = "%Y-%m-%d %H%M";


/// # About
/// time_tracker is a simple log time in and log time out application. 
/// The program takes command line arguments and executes either an 
/// IN time input or and OUT time input into file time_tracker.log.
/// If time_tracker.log does not exist, then the program creates the
/// file.
/// 
/// For this current version we are calling the libraries directly
/// instead of importing them through the use import.
fn main() {

    // Collect our command line arguments
    let args: Vec<String> = std::env::args().collect();
    // Set our output if no commands are provided
    let usage = format!(
        "Usage: 
            time_tracker in     - marks time in within the time file.
            time_tracker out    - marks time out within the time file.
            time_tracker list   - lists times logged."
    );

    // if our argument input is less than 2 (first is the app name, second
    // is any input by the user), we print the usage statement from above.
    // we continue.
    if args.len() < 2 {
        println!("{}", usage);
    } else {
        // we set the path for our const LOG_FILE
        let path: &std::path::Path = std::path::Path::new(LOG_FILE);
        // we parse the input by the user, if it matches below, we proceed
        // else we return nothing then exit.
        match args[1].parse::<String>().unwrap().as_str() {
            "in" => log_time(LABEL_IN, &path),
            "out" => log_time(LABEL_OUT, &path),
            "list" => {
                print_logs(&path);
                std::process::exit(1);
            }
            &_ => std::process::exit(-1),
        }
        print_logs(&path);
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
fn print_logs(path: &std::path::Path) {
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

/// # Usage
/// provided a label and a path, we build a string to be sent to our log file.
fn log_time(label: &str, path: &std::path::Path) {
    // String that will be output to the file
    let log_out = [&get_time_stamp(), " ", &label, "\n"].concat();
    // call our output program to write the string with our output and the path
    write_to_log_file(&log_out, &path);
}

/// # Usage
/// Builds a string value based on our const TIME_FORMAT
fn get_time_stamp() -> String {
    let time_stamp: String = chrono::Local::now().format(TIME_FORMAT).to_string();

    time_stamp
}

/// # Usage
/// takes a string line and the path to our time_tracker.log file and creates a new
/// file if one does not exist, it appends if the file does exist. If we are 
/// successful, we write to the file.
fn write_to_log_file(line: &str, path: &std::path::Path) {
    match std::fs::OpenOptions::new().read(true).create(true).append(true).open(path) {
        Ok(mut s) => s.write_all(line.as_bytes()).expect("Error writing to file"), 
        Err(why) => panic!("{}", why),
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    
    #[test]
    pub fn test_time() {
        println!("{}", chrono::Local::now().format("%Y-%m-%d %H%M"));
    }

    #[test]
    pub fn test_print_logs() {
        let path: &std::path::Path = std::path::Path::new(LOG_FILE);
        print_logs(&path);
    }
}
