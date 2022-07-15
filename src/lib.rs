mod utils;

use wasm_bindgen::prelude::*;
use similar::{ChangeTag, TextDiff};
use serde::{Serialize, Deserialize};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[derive(Serialize, Deserialize)]
pub struct LineDiff {
    pub op: String,
    pub text: String
}

#[wasm_bindgen]
pub fn send_array_to_js(source: &str, target: &str) -> JsValue {
    let mut list: Vec<LineDiff> = Vec::new();
    // let line = LineDiff {
    //     op: String::from("+"),
    //     text: String::from("ttt")
    // };
    

    let diff = TextDiff::from_lines(
        source,
        target
    );
    for change in diff.iter_all_changes() {
        let sign = match change.tag() {
            ChangeTag::Delete => "-",
            ChangeTag::Insert => "+",
            ChangeTag::Equal => " ",
        };
        let line = LineDiff {
            op: sign.to_string(),
            text: change.value().to_string()
        };
        list.push(line);
    }
    JsValue::from_serde(&list).unwrap()
}

#[wasm_bindgen]
pub fn compare_from_words(source: &str, target: &str) -> JsValue {
    let mut list: Vec<LineDiff> = Vec::new();
    let diff = TextDiff::from_words(
        source,
        target
    );
    for change in diff.iter_all_changes() {
        let sign = match change.tag() {
            ChangeTag::Delete => "-",
            ChangeTag::Insert => "+",
            ChangeTag::Equal => " ",
        };
        let line = LineDiff {
            op: sign.to_string(),
            text: change.value().to_string()
        };
        list.push(line);
    }
    JsValue::from_serde(&list).unwrap()
}

#[wasm_bindgen]
pub fn compare_from_chars(source: &str, target: &str) -> JsValue {
    let mut list: Vec<LineDiff> = Vec::new();
    let diff = TextDiff::from_chars(
        source,
        target
    );
    for change in diff.iter_all_changes() {
        let sign = match change.tag() {
            ChangeTag::Delete => "-",
            ChangeTag::Insert => "+",
            ChangeTag::Equal => " ",
        };
        let line = LineDiff {
            op: sign.to_string(),
            text: change.value().to_string()
        };
        list.push(line);
    }
    JsValue::from_serde(&list).unwrap()
}