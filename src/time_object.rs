/**
 * #About
 * TimeObject used to track time in a concise method for calculations.
 * 
 * #Todo
 * * build calculations for changing days. Currently on calcs within same 24 hour period
 */

use chrono::prelude::*;

#[derive(PartialEq)]
pub enum TimeState {
    IN,
    OUT,
    None,
}

pub struct TimeObject {
    pub time_state: TimeState,
    pub date: DateTime<Local>,
    pub time_stamp: DateTime<Local>,
}

impl TimeObject {

    const CLOCK_IN_FORMAT: &'static str = "%H:%M:%S";
    const MONTH: &'static str = "%m";
    const DAY: &'static str = "%d";
    const HOUR: &'static str = "%H";
    const MINUTE: &'static str = "%M";

    pub fn new() -> TimeObject {
        TimeObject {
            time_state: TimeState::None,
            date: Local::now(),
            time_stamp: Local::now(),
        }
    }
    
    pub fn get_month(&self) -> i32 {
        self.time_stamp.format(Self::MONTH).to_string().parse::<i32>().unwrap()
    }

    pub fn get_day(&self) -> i32 {
        self.time_stamp.format(Self::DAY).to_string().parse::<i32>().unwrap()
    }

    pub fn update_time_stamp(&mut self) {
        self.time_stamp = Local::now();
    }

    pub fn get_current_time(&mut self) -> String {
        self.time_stamp.format(&Self::CLOCK_IN_FORMAT).to_string()
    }

    pub fn get_hour_min(&self) -> (i32, i32) {
        
        let hour = self.time_stamp.format(Self::HOUR).to_string();
        let minute = self.time_stamp.format(Self::MINUTE).to_string();
        let hour = hour.parse::<i32>().unwrap();
        let minute = minute.parse::<i32>().unwrap();

        (hour, minute)
    }

    pub fn clock_in(&mut self) {
        self.time_state = TimeState::IN;
        self.time_stamp = Local::now();
    }

    pub fn clock_out(&mut self) {
        self.time_state = TimeState::OUT;
        self.time_stamp = Local::now();
    }

    pub fn set_time(&mut self, time_state: TimeState, date: DateTime<Local>, time_stamp: DateTime<Local>) {
        self.time_state = time_state;
        self.date = date;
        self.time_stamp = time_stamp;
    }

    pub fn get_rounded_mins(min: i64) -> i64 {
        15 * (min / 15)
    }
}

#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    pub fn test_get_days_diff() {
        let mut t1: TimeObject = TimeObject::new();
        let mut t2: TimeObject = TimeObject::new();

        t1.set_time(TimeState::IN, 
            Local.ymd(2020, 12, 8).and_hms(6, 13, 25), 
            Local.ymd(2020, 12, 8).and_hms(6, 13, 25));

        t2.set_time(TimeState::OUT, 
            Local.ymd(2020, 12, 8).and_hms(6, 13, 25), 
            Local.ymd(2020, 12, 10).and_hms(6, 13, 25));
        
        let min = t1.time_stamp.signed_duration_since(t2.time_stamp).num_minutes().abs();
        assert_eq!(min, (48 * 60));
    
    }

    #[test]
    pub fn test_time_object() {
        let mut time_object: TimeObject = TimeObject::new();
        assert_ne!(time_object.date, Local::now());
        assert_ne!(time_object.time_stamp, Local::now());
        dbg!(&time_object.date);
        dbg!(&time_object.time_stamp);
        
        &time_object.update_time_stamp();
        dbg!(&time_object.time_stamp);
        dbg!(&time_object.get_current_time());
        dbg!(&time_object.get_hour_min());
    }

    #[test]
    pub fn test_set_time() {
        let mut t1: TimeObject = TimeObject::new();
        let mut t2: TimeObject = TimeObject::new();
        
        let in_time: DateTime<Local> = Local.ymd(2020, 12, 8).and_hms(6, 13, 25);
        t1.set_time(TimeState::IN, in_time, in_time);
        
        assert_eq!(t1.date, in_time);
        assert_eq!(t1.time_stamp, in_time);
    }

    #[test]
    pub fn test_find_min_15() {
        assert_eq!(TimeObject::get_rounded_mins(5), 0);
        assert_eq!(TimeObject::get_rounded_mins(17), 15);
        assert_eq!(TimeObject::get_rounded_mins(31), 30);
        assert_eq!(TimeObject::get_rounded_mins(59), 45);

        assert_eq!(TimeObject::get_rounded_mins(0), 0);
        assert_eq!(TimeObject::get_rounded_mins(15), 15);
        assert_eq!(TimeObject::get_rounded_mins(30), 30);
        assert_eq!(TimeObject::get_rounded_mins(45), 45);
    }

    #[test]
    pub fn test_calculate_time() {
        let mut t1: TimeObject = TimeObject::new();
        let in_time: DateTime<Local> = Local.ymd(2020, 12, 8).and_hms(6, 13, 25);
        t1.set_time(TimeState::IN, in_time, in_time);
        assert_eq!(t1.date, in_time);
        assert_eq!(t1.time_stamp, in_time);

        let mut t2: TimeObject = TimeObject::new();        
        let out_time: DateTime<Local> = Local.ymd(2020, 12, 8).and_hms(16, 44, 21);
        t2.set_time(TimeState::OUT, in_time, out_time);
        assert_eq!(t2.date, in_time);
        assert_eq!(t2.time_stamp, out_time);
        
        let mins = t1.time_stamp.signed_duration_since(t2.time_stamp).num_minutes().abs();
        dbg!(mins);

        let hours = mins / 60;
        dbg!(hours);
        let rounded_mins = TimeObject::get_rounded_mins(mins - (hours * 60));
        assert_eq!(hours, 10);
        assert_eq!(rounded_mins, 30);
    }
}