mod utils;
mod time_object;
mod time_utils;
mod times;
mod file_utils;
mod label_values;
mod time_obj_helper;

extern crate web_sys;

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

// #[wasm_bindgen]
// pub fn load_existing_times() -> (String, String, String) {
//     ExportedTupleStruct("one".into(), "two".into(), "three".into())
// }