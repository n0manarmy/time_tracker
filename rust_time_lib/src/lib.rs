mod utils;
mod time_object;
mod time_utils;
mod times;
mod file_utils;
mod label_values;
mod time_obj_helper;

extern crate web_sys;
extern crate js_sys;

use time_obj_helper::TimeObjHelper;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn get_current_time() -> String {
    time_utils::TimeUtils::get_current_time()
}

#[wasm_bindgen]
pub fn clock_in_out(state: String) -> TimeObjHelper {
    web_sys::console::log_1(&format!("rust: clock_in_out state {}", state).into());
    // label_values::get_in_label_value()
    match state.as_ref() {
        "IN" => {
            let t = time_object::TimeObject::new().clock_in();
            time_obj_helper::TimeObjHelper::new(t.get_time_stamp_date_string(), t.get_current_time(), t.time_state.to_string())
        },
        "OUT" => {
            let t = time_object::TimeObject::new().clock_out();
            time_obj_helper::TimeObjHelper::new(t.get_time_stamp_date_string(), t.get_current_time(), t.time_state.to_string())
        },
        &_ => panic!("found nothing in the match"),
    }
    
}

#[wasm_bindgen]
pub fn get_previous_logs() -> Box<[TimeObjHelper]> {
    load_previous_logs().into_boxed_slice()
}

pub fn load_previous_logs() -> Vec<TimeObjHelper> {
    let log_file = file_utils::FileUtils::load_log_file("log_file.json");
    let log_file = file_utils::FileUtils::read_log_file_to_vec(&log_file);
    let log_file = time_object::TimeObject::build_time_object_vec(log_file);

    let log_file = log_file.into_iter().map(|log| TimeObjHelper::new(
                                                    log.get_time_stamp_date_string(),
                                                    log.get_current_time(),
                                                    log.time_state.to_string())).collect();

    log_file
}

// #[wasm_bindgen]
// pub fn load_existing_times() -> (String, String, String) {
//     ExportedTupleStruct("one".into(), "two".into(), "three".into())
// }