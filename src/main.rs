extern crate chrono;
use std::io::prelude::*;

const LABEL_IN: &'static str = "IN";
const LABEL_OUT: &'static str = "OUT";
const LOG_FILE: &'static str = "./time_tracker.log";

fn main() {

    let args: Vec<String> = std::env::args().collect();
    let path: &std::path::Path = std::path::Path::new(LOG_FILE);

    let usage = format!(
        "Usage: 
            time_tracker in     - marks time in within the time file.
            time_tracker out    - marks time out within the time file.
            time_tracker list   - lists times logged."
    );
    if args.len() < 2 {
        println!("{}", usage);
        std::process::exit(1);
    }

    match args[1].parse::<String>().unwrap().as_str() {
        "in" => log_time(LABEL_IN, &path),
        "out" => log_time(LABEL_OUT, &path),
        "list" => {
            print_logs(&path);
            std::process::exit(1);
        }
        &_ => (),
    }

    print_logs(&path);
}


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

fn log_time(label: &str, path: &std::path::Path) {
    let log_out = [&get_time_stamp(), " ", &label, "\n"].concat();
    match write_to_log_file(&log_out, &path) {
        Ok(ok) => println!("Results {:?}", ok),
        Err(err) => panic!("{}", err),
    }
}

fn get_time_stamp() -> String {
    let format = "%Y-%m-%d %H:%M";
    let time_stamp: String = chrono::Local::now().format(format).to_string();

    time_stamp
}

fn write_to_log_file(line: &str, path: &std::path::Path) -> std::io::Result<()> {
    let mut out_file = std::fs::OpenOptions::new()
        .read(true)
        .create(true)
        .append(true)
        .open(path)?;
    out_file.write_all(line.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
    
    #[test]
    pub fn test_time() {
        println!("{}", chrono::Local::now().format("%Y-%m-%d %H:%M"));
    }

    #[test]
    pub fn test_print_logs() {
        let path: &std::path::Path = std::path::Path::new(LOG_FILE);
        print_logs(&path);
    }
}
