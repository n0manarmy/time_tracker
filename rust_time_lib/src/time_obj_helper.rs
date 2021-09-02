use wasm_bindgen::{convert::{IntoWasmAbi, WasmSlice}, prelude::*};

use crate::time_object::TimeObject;

#[wasm_bindgen]
pub struct TimeObjHelper {
    date: String,
    time_stamp: String,
    time_state: String,
}

pub fn time_object_to_js_value(v: TimeObject) -> JsValue {
    match JsValue::from_serde(&v) {
        Ok(v) => v,
        Err(why) => panic!("{}", why),
    }
}

#[wasm_bindgen]
impl TimeObjHelper {

    #[wasm_bindgen(constructor)]
    pub fn new(date: String, time_stamp: String, time_state: String) -> TimeObjHelper {
        TimeObjHelper {
            date,
            time_stamp,
            time_state,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn date(&self) -> String {
        self.date.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn time_stamp(&self) -> String {
        self.time_stamp.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn time_state(&self) -> String {
        self.time_state.clone()
    }
}