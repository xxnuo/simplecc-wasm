use simplecc::Dict;
use wasm_bindgen::prelude::*;

mod dicts;

#[wasm_bindgen]
pub fn simplecc(text: &str, name: &str) -> Result<String, JsValue> {
    let fname = name.split(".").nth(0);
    let dict: &Dict = match fname {
        None => return Err(JsValue::from_str("dict name is not supported")),
        Some(fname) => match fname {
            "s2t" => &*dicts::S2T,
            "t2s" => &*dicts::T2S,
            "s2tw" => &*dicts::S2TW,
            "s2hk" => &*dicts::S2HK,
            "hk2s" => &*dicts::HK2S,
            "s2twp" => &*dicts::S2TWP,
            "tw2sp" => &*dicts::TW2SP,
            _ => return Err(JsValue::from_str("dict name is not supported")),
        },
    };
    Ok(dict.replace_all(text))
}
