#![cfg_attr(not(feature = "gtk_3_10"), allow(unused_variables, unused_mut))]

mod file_utils;
mod time_utils;
mod gui_construct;

use gio::prelude::*;
use gui_construct::GuiConstruct;

use std::env::args;


fn main() {
    let application = gtk::Application::new(
        Some("com.time.tracker.my"),
        Default::default(),
    )
    .expect("Initialization failed...");

    application.connect_activate(|app| {
        GuiConstruct::build(app);
    });

    application.run(&args().collect::<Vec<_>>());
}