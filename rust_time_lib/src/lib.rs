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
    // web_sys::console::log_1(&format!("rust: clock_in_out state {}", state).into());
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
pub fn get_previous_logs() -> Vec<JsValue> {
    unsafe {
        web_sys::console::log_1(&format!("get_previous_logs").into());
    }    
    load_previous_logs()
}

pub fn load_previous_logs() -> Vec<JsValue> {
    let log_file: String = match file_utils::load_log_file("/home/user/workspace/time_tracker/log_file.json") {
        Ok(v) => v,
        Err(why) => panic!("{}", why),
    };
    let log_file = log_file.lines().map(|l| l.to_string()).collect();
    
    let log_file = time_object::TimeObject::build_time_object_vec(log_file);

    let log_file = log_file.into_iter().map(|log| time_obj_helper::time_object_to_js_value(log)).collect();

    log_file
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    pub fn test_load_previous_logs() {
        let log_file = load_previous_logs();
        dbg!(&log_file);
        // for l in log_file {
        //     println!("{:?}", l);
        // }
    }
}