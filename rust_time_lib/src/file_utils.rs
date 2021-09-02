use std::io::prelude::*;
use std::path::Path;
use std::fs::File;
use std::io::Read;

/// # Usage
/// takes a string line and the path to our time_tracker.log file and creates a new
/// file if one does not exist, it appends if the file does exist. If we are 
/// successful, we write to the file.
pub fn write_to_log_file(line: &str, path: &str) {
    match std::fs::OpenOptions::new().read(true).create(true).append(true).open(path) {
        Ok(mut s) => s.write_all([line, "\n"].concat().as_bytes()).expect("Error writing to file"), 
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

// check our log if it exists
pub fn log_file_exists(path: &str) -> bool {
    Path::new(&path).exists()
}

pub fn read_log_file_to_vec(path: String) -> Vec<String> {
    if log_file_exists(&path) {
        let file = match File::open(&path) {
            Ok(f) => f,
            Err(why) => panic!("{}", why),
        };
        let reader = std::io::BufReader::new(file);
        let mut times: Vec<String> = Vec::new();
        for line in reader.lines() {
            match line {
                Ok(l) => times.push(l),
                Err(why) => panic!("{}", why),
            }
        }
        return times;
    } else {
        panic!("Log file does not exist!");
    }
}

pub fn load_log_file(path: &'static str) -> String {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(why) => panic!("{}", why),
    };
    let mut results = String::new();
    match file.read_to_string(&mut results) {
        Ok(_) => return results,
        Err(why) => panic!("{}", why),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_load_log_file() {
        load_log_file("log_file.json");
    }
}