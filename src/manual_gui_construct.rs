#![cfg_attr(not(feature = "gtk_3_10"), allow(unused_variables, unused_mut))]

use gio::prelude::*;
use gtk::prelude::*;

use crate::file_utils::FileUtils;
use crate::message::Message;
use crate::time_utils::TimeUtils;
use crate::time_object::TimeObject;

use std::{thread, time};
use std::rc::Rc;
use glib::{clone, Continue};
use gtk::{
    Orientation,
    ApplicationWindow, ListStore, TreeView
};

const LOG_FILE: &'static str = "log_file.rson";


pub struct GuiConstruct {}

impl GuiConstruct {

    pub fn build_frames(previous_logs: String) -> gtk::Box {
        let curent_time_label_text = "Current Time";
        let time_in_button_text = "Time In";
        let time_out_button_text = "Time Out";

        // Storage frames
        let left_frame = gtk::Box::new(Orientation::Vertical, 5);
        let right_frame = gtk::Box::new(Orientation::Vertical, 5);
        let completed_frame = gtk::Box::new(Orientation::Horizontal, 5);

        //Time label and updater services
        let current_time_label = gtk::Label::new(Some(curent_time_label_text));
        let current_time = gtk::Label::new(Some(&TimeUtils::get_current_time()));
        let (msg_sender, msg_receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

        // storage for time in/time out
        let list_store: gtk::ListStore = Self::create_model();
        let model = Rc::new(list_store.clone());
        let renderer = gtk::CellRendererText::new();
        let column = gtk::TreeViewColumn::new();

        // tree view to store the new model
        let tree_view = TreeView::new_with_model(&*model);
        // reference counter based on the data model
        let col_indicies: [u32; 1] = [0];
        let values: [&dyn ToValue; 1] = [&previous_logs];
        let scrolled_window = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);

        // Spacers
        let spacer = gtk::Frame::new(None);

        // Buttons
        let button_time_in = gtk::Button::new_with_label(time_in_button_text);
        let button_time_out = gtk::Button::new_with_label(time_out_button_text);

        list_store.set(&list_store.append(), &col_indicies, &values);
        tree_view.set_vexpand(true);
        
        // scrolled window holds the tree view
        scrolled_window.add(&tree_view);
        column.pack_start(&renderer, true);
        column.set_title("Log");
        column.set_alignment(0.5);
        column.add_attribute(&renderer, "text", 0);
        tree_view.append_column(&column);
    
        // Button functions and callbacks
        button_time_in.connect_clicked(clone!(@weak list_store => move |_| {
            let time_stamp = TimeUtils::get_time_stamp("IN");
    
            let values: [&dyn ToValue; 1] = [&time_stamp];
            list_store.set(&list_store.append(), &[0], &values);
            FileUtils::write_to_log_file(&time_stamp, &LOG_FILE);
        }));
    
        button_time_out.connect_clicked(clone!(@weak list_store => move |_| {
            let time_stamp = TimeUtils::get_time_stamp("OUT");
    
            let values: [&dyn ToValue; 1] = [&time_stamp];
            list_store.set(&list_store.append(), &[0], &values);
            FileUtils::write_to_log_file(&time_stamp, &LOG_FILE);
        }));


        // ----> Clock threads
        // Thread manager for current time
        let current_time_clone = current_time.clone();

        // update enum time value
        thread::spawn(move || {
            loop {
                thread::sleep(time::Duration::from_secs(1));
                let _ = msg_sender.send(Message::UpdateLabel(TimeUtils::get_current_time()));
            }
        });
        
        // set gtk label with time value
        msg_receiver.attach(None, move |msg| {
            match msg {
                Message::UpdateLabel(text) => current_time_clone.set_text(text.as_str()),
            }
            Continue(true)
        });
        // ----<

        //build frames
        left_frame.pack_start(&current_time_label, false, false, 0);
        left_frame.pack_start(&current_time, false, false, 0);
        left_frame.pack_start(&spacer, false, false, 10);
        left_frame.pack_start(&button_time_in, false, false, 0);
        left_frame.pack_start(&button_time_out, false, false, 0);

        right_frame.pack_start(&scrolled_window, true, true, 20);

        completed_frame.pack_start(&left_frame, false, false, 5);
        completed_frame.pack_start(&right_frame, true, true, 5);

        completed_frame
    }

    pub fn build(application: &gtk::Application) {
        
        //init data store for times
        let previous_logs = if FileUtils::log_file_exists(LOG_FILE) {
            FileUtils::load_log_file(LOG_FILE)
        } else {
            String::new()
        };
        
    
        // used to manage the time clock display
        // let (msg_sender, msg_receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
    
        // loads the xml created in glade
        // let glade_src = include_str!("gui/gui.glade");
        // let builder = Builder::new_from_string(glade_src);
    
        // loads the window from glade that contains the objects
        // let window: ApplicationWindow = builder.get_object("root").expect("Couldn't get window");
        let window: ApplicationWindow = ApplicationWindow::new(application);
        window.set_default_size(600,400);
        window.set_application(Some(application));
        window.set_title("Time Tracker");

        window.add(&Self::build_frames(previous_logs));
    
        window.show_all();
    }
    
    fn create_model() -> ListStore {
    
        let col_type: [glib::Type; 1] = [
            glib::Type::String,
        ];
    
        ListStore::new(&col_type)
    }
}