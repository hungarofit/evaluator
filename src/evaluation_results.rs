extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use super::classification::Classification;

#[wasm_bindgen(inline_js = r#"
export function createObject() { return {}; }
export function setProperty(obj, key, value) { obj[key] = value; }
"#)]
extern "C" {
    fn createObject() -> JsValue;
    fn setProperty(obj: &JsValue, key: &str, value: &JsValue);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
pub struct HungarofitEvaluation {
    pub classification: Classification,
    pub total_score: f32,
    pub aerob_score: f32,
    pub jump_score: f32,
    pub pushup_score: f32,
    pub situp_score: f32,
    pub torso_score: f32,
    pub throwdouble_score: f32,
    pub throwsingle_score: f32,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
impl HungarofitEvaluation {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen(js_name = toJSON))]
    pub fn to_json(&self) -> JsValue {
        let obj = createObject();
        setProperty(&obj, "classification", &JsValue::from(self.classification as u32));
        setProperty(&obj, "total_score", &JsValue::from(self.total_score));
        setProperty(&obj, "aerob_score", &JsValue::from(self.aerob_score));
        setProperty(&obj, "jump_score", &JsValue::from(self.jump_score));
        setProperty(&obj, "pushup_score", &JsValue::from(self.pushup_score));
        setProperty(&obj, "situp_score", &JsValue::from(self.situp_score));
        setProperty(&obj, "torso_score", &JsValue::from(self.torso_score));
        setProperty(&obj, "throwdouble_score", &JsValue::from(self.throwdouble_score));
        setProperty(&obj, "throwsingle_score", &JsValue::from(self.throwsingle_score));
        obj
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
pub struct HungarofitMiniEvaluation {
    pub classification: Classification,
    pub total_score: f32,
    pub aerob_score: f32,
    pub jump_score: f32,
    pub pushup_score: f32,
    pub situp_score: f32,
    pub torso_score: f32,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
impl HungarofitMiniEvaluation {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen(js_name = toJSON))]
    pub fn to_json(&self) -> JsValue {
        let obj = createObject();
        setProperty(&obj, "classification", &JsValue::from(self.classification as u32));
        setProperty(&obj, "total_score", &JsValue::from(self.total_score));
        setProperty(&obj, "aerob_score", &JsValue::from(self.aerob_score));
        setProperty(&obj, "jump_score", &JsValue::from(self.jump_score));
        setProperty(&obj, "pushup_score", &JsValue::from(self.pushup_score));
        setProperty(&obj, "situp_score", &JsValue::from(self.situp_score));
        setProperty(&obj, "torso_score", &JsValue::from(self.torso_score));
        obj
    }
}
