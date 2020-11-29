#![cfg_attr(not(feature = "gtk_3_10"), allow(unused_variables, unused_mut))]

use gio::prelude::*;
use gtk::prelude::*;

use crate::file_utils::FileUtils;
use crate::message::Message;
use crate::time_utils::TimeUtils;

use std::{thread, time};
use std::rc::Rc;
use glib::{clone, Continue};
use gtk::{
    ApplicationWindow, Builder, Button, Label, ListStore, TreeView, ScrolledWindow
};

const LOG_FILE: &'static str = "log_file.log";


pub struct GuiConstruct {}

impl GuiConstruct {
    pub fn build(application: &gtk::Application) {
        
        //init data store for times
        let previous_logs = if FileUtils::log_file_exists(LOG_FILE) {
            FileUtils::load_log_file(LOG_FILE)
        } else {
            String::new()
        };
    
        // used to manage the time clock display
        let (msg_sender, msg_receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
    
        // loads the xml created in glade
        let glade_src = include_str!("gui/gui.glade");
        let builder = Builder::new_from_string(glade_src);
    
        // loads the window from glade that contains the objects
        let window: ApplicationWindow = builder.get_object("root").expect("Couldn't get window");
        window.set_default_size(600,400);
        window.set_application(Some(application));
    
        // Buttons
        let button_time_in: Button = builder.get_object("button.time.in").expect("couldn't get time in button");
        let button_time_out: Button = builder.get_object("button.time.out").expect("couldn't get time in button");
        // let button_time_list: Button = builder.get_object("button.time.list").expect("couldn't get time in button");
        
        // Labels
        let label_last_clock_in: Label = builder.get_object("label.current.time").expect("couldn't get label for last clock in");
        let current_time: Label = builder.get_object("current.time").expect("couldn't get label for current time");
        current_time.set_text(&TimeUtils::get_current_time());
    
        // Views
        let scrolled_window: ScrolledWindow = builder.get_object("scrolled.window").expect("Couldn't load scrolled window");
    
        // reference counter based on the data model
        let list_store: gtk::ListStore = Self::create_model();
        let col_indicies: [u32; 1] = [0];
        let values: [&dyn ToValue; 1] = [&previous_logs];
    
        list_store.set(&list_store.append(), &col_indicies, &values);
    
        let model = Rc::new(list_store.clone());
    
        // tree view to store the new model
        let tree_view = TreeView::new_with_model(&*model);
        tree_view.set_vexpand(true);
    
        // scrolled window holds the tree view
        scrolled_window.add(&tree_view);
        Self::add_columns(&model, &tree_view);    
    
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
    
        window.show_all();
    }
    
    fn create_model() -> ListStore {
    
        let col_type: [glib::Type; 1] = [
            glib::Type::String,
        ];
    
        ListStore::new(&col_type)
    }
    
    fn add_columns(model: &Rc<ListStore>, tree_view: &TreeView) {
        {
            let renderer = gtk::CellRendererText::new();
            let column = gtk::TreeViewColumn::new();
            column.pack_start(&renderer, true);
            column.set_title("Log");
            column.set_alignment(0.5);
            column.add_attribute(&renderer, "text", 0);
            tree_view.append_column(&column);
        }
    }
}