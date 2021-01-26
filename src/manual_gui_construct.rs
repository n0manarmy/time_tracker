#![cfg_attr(not(feature = "gtk_3_10"), allow(unused_variables, unused_mut))]

use gio::prelude::*;
use gtk::prelude::*;

use crate::file_utils::FileUtils;
use crate::message::Message;
use crate::time_utils::TimeUtils;
use crate::time_object::TimeObject;
use crate::my_menu_bar::MyMenuBar;

use std::{thread, time};
use std::rc::Rc;
use glib::{clone, Continue};
use gtk::{
    Orientation,
    ApplicationWindow, TreeView
};

pub struct GuiConstruct {}

impl GuiConstruct {

    pub fn init(times: Vec<TimeObject>, log_file: &'static str) -> gtk::Box {
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

        // Spacers
        let spacer = gtk::Frame::new(None);

        // Buttons
        let button_time_in = gtk::Button::new_with_label(time_in_button_text);
        let button_time_out = gtk::Button::new_with_label(time_out_button_text);

        // storage for time in/time out
        let col_type: [glib::Type; 3] = [
            glib::Type::String,
            glib::Type::String,
            glib::Type::String,
        ];

        let list_store: gtk::ListStore = gtk::ListStore::new(&col_type);
        let scrolled_window = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
        scrolled_window.set_policy(gtk::PolicyType::Never, gtk::PolicyType::Automatic);
        let col_indicies: [u32; 3] = [0, 1, 2];
        
        let tree_view = Self::build_table(&list_store, times, &col_indicies);
        
        scrolled_window.add(&tree_view);
    
        // Button functions and callbacks
        button_time_in.connect_clicked(clone!(@weak list_store => move |_| {
            // let time_stamp = TimeUtils::get_time_stamp("IN");
            let t: TimeObject = TimeObject::new().clock_in();
    
            let v: [&dyn ToValue; 3] = [
                &t.get_time_stamp_date_string(), 
                &t.get_current_time(),
                &t.time_state.to_string(),
            ];
            list_store.set(&list_store.append(), &col_indicies, &v);
            FileUtils::write_to_log_file(&t.to_serde_str(), &log_file);
        }));
    
        button_time_out.connect_clicked(clone!(@weak list_store => move |_| {
            // let time_stamp = TimeUtils::get_time_stamp("OUT");
    
            // let values: [&dyn ToValue; 1] = [&time_stamp];
            // list_store.set(&list_store.append(), &[0], &values);
            // FileUtils::write_to_log_file(&time_stamp, &LOG_FILE);
            let t: TimeObject = TimeObject::new().clock_out();
    
            let v: [&dyn ToValue; 3] = [
                &t.get_time_stamp_date_string(), 
                &t.get_current_time(),
                &t.time_state.to_string(),
            ];
            list_store.set(&list_store.append(), &col_indicies, &v);
            FileUtils::write_to_log_file(&t.to_serde_str(), &log_file);
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

        completed_frame.pack_start(&left_frame, false, true, 5);
        completed_frame.pack_start(&right_frame, true, true, 5);

        completed_frame
    }

    pub fn build_table(list_store: &gtk::ListStore, times: Vec<TimeObject>, col_indicies: &[u32]) -> gtk::TreeView {
        let model = Rc::new(list_store.clone());
        let tree_view = TreeView::new_with_model(&*model);

        Self::add_columns(&tree_view);
        tree_view.set_vexpand(true);

        for t in times {

            let v: [&dyn ToValue; 3] = [
                &t.get_time_stamp_date_string(), 
                &t.get_current_time(),
                &t.time_state.to_string(),
            ];
            list_store.set(&list_store.append(), &col_indicies, &v);
        }

        tree_view
    }

    pub fn add_columns(tree_view: &gtk::TreeView) {
        {
            let renderer = gtk::CellRendererText::new();
            let column = gtk::TreeViewColumn::new();
            renderer.set_alignment(0.5, 1.0);
            column.pack_start(&renderer, true);
            column.set_title("Date");
            column.set_alignment(0.5);
            column.add_attribute(&renderer, "text", 0);
            tree_view.append_column(&column);
        }

        {
            let renderer = gtk::CellRendererText::new();
            let column = gtk::TreeViewColumn::new();
            renderer.set_alignment(0.5, 1.0);
            column.pack_start(&renderer, true);
            column.set_title("Time Stamp");
            column.set_alignment(0.5);
            column.add_attribute(&renderer, "text", 1);
            tree_view.append_column(&column);
        }

        {
            let renderer = gtk::CellRendererText::new();
            let column = gtk::TreeViewColumn::new();
            renderer.set_alignment(0.5, 1.0);
            column.pack_start(&renderer, true);
            column.set_title("Time State");
            column.set_alignment(0.5);
            column.add_attribute(&renderer, "text", 2);
            tree_view.append_column(&column);
        }

    }

    pub fn build(application: &gtk::Application, times: Vec<TimeObject>, log_file: &'static str) {
        

        let window: ApplicationWindow = ApplicationWindow::new(application);
        window.set_default_size(600,400);
        window.set_application(Some(application));
        // window.set_position(gtk::WindowPosition::None);
        window.set_title("Time Tracker");

        let completed_frame = gtk::Box::new(Orientation::Vertical, 0);
        
        // Packing with add defaults justification based on received containers
        completed_frame.add(&MyMenuBar::init());
        completed_frame.add(&Self::init(times, log_file));
        
        window.add(&completed_frame);
    
        window.show_all();
    }
}