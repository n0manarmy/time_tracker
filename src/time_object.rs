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

    pub fn get_hours_mins_diff(t1: &TimeObject, t2: &TimeObject) -> (i32, i32) {
        let mut days: i32 = 0;
        if t1.get_month() > t2.get_month() {
            days = Self::get_days_diff(t2, t1);
        } else if t2.get_month() > t1.get_month() {
            days = Self::get_days_diff(t1, t2);
        }

        let x: (i32, i32) = t1.get_hour_min();
        let y: (i32, i32) = t2.get_hour_min();

        ((x.0 - y.0).abs(), (x.1 - y.1).abs())
    }

    pub fn get_days_diff(x: &TimeObject, y: &TimeObject) -> i32 {
        // (x - y).abs() 

        0
    }

    pub fn get_rounded_mins(min: i32) -> i32 {
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

        assert_eq!(TimeObject::get_days_diff(&t1, &t2), 2);
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
        let mut t2: TimeObject = TimeObject::new();
        
        let in_time: DateTime<Local> = Local.ymd(2020, 12, 8).and_hms(6, 13, 25);
        let out_time: DateTime<Local> = Local.ymd(2020, 12, 8).and_hms(16, 44, 21);

        t1.set_time(TimeState::IN, in_time, in_time);
        assert_eq!(t1.date, in_time);
        assert_eq!(t1.time_stamp, in_time);
        
        t2.set_time(TimeState::OUT, in_time, out_time);
        assert_eq!(t2.date, in_time);
        assert_eq!(t2.time_stamp, out_time);
        
        let time = TimeObject::get_hours_mins_diff(t2, t1);
        dbg!(time);

        let hours = time.0;
        let mins = TimeObject::get_rounded_mins(time.1);
        assert_eq!(hours, 10);
        assert_eq!(mins, 30);
        dbg!(&hours);
        dbg!(&mins);
    }
}