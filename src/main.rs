#![cfg_attr(not(feature = "gtk_3_10"), allow(unused_variables, unused_mut))]
extern crate serde;

mod file_utils;
mod time_utils;
mod manual_gui_construct;
mod message;
mod time_object;
mod my_menu_bar;

use gio::prelude::*;
use manual_gui_construct::GuiConstruct;
use time_object::TimeObject;
use file_utils::FileUtils;

use std::env::args;

fn main() {

    let application = gtk::Application::new(
        Some("com.my.time.tracker"),
        Default::default(),
    )
    .expect("Initialization failed...");

    application.connect_activate(|app| {
        //init data store for times
        let log_file: &'static str = "log_file.json";
        let times: Vec<TimeObject> = if FileUtils::log_file_exists(&log_file) {
            TimeObject::build_time_object_vec(FileUtils::read_log_file_to_vec(&log_file))
        } else {
            Vec::new()
        };
        // dbg!(times);
        GuiConstruct::build(app, times, log_file);
    });

    application.run(&args().collect::<Vec<_>>());
}