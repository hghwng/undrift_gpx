#![feature(proc_macro, wasm_custom_section, wasm_import_module)]
extern crate converter;
extern crate quick_xml;
extern crate undrift_gps;
extern crate wasm_bindgen;

use converter::*;
use quick_xml::{Reader, Writer};
use undrift_gps::*;
use wasm_bindgen::prelude::*;

const FUNCS: &[(&str, ConvertFn)] = &[
    ("bd-gcj", bd_to_gcj),
    ("bd-wgs", bd_to_wgs),
    ("gcj-bd", gcj_to_bd),
    ("gcj-wgs", gcj_to_wgs),
    ("wgs-bd", wgs_to_bd),
    ("wgs-gcj", wgs_to_gcj),
];

#[wasm_bindgen]
pub struct Result {
    data: Vec<u8>,
    err: String,
}

#[wasm_bindgen]
impl Result {
    pub fn get_data(&self) -> Vec<u8> {
        self.data.clone()
    }
    pub fn get_err(&self) -> String {
        self.err.clone()
    }
}

#[wasm_bindgen]
pub extern "C" fn convert(input: &[u8], func_name: String) -> Result {
    const INDENT_CHAR: u8 = b'\t';
    const INDENT_LEVEL: usize = 1;

    let reader = Reader::from_reader(input);
    let buf = Vec::new();
    let mut writer = Writer::new_with_indent(buf, INDENT_CHAR, INDENT_LEVEL);

    let func = match FUNCS.iter().find(|(name, _)| *name == func_name) {
        Some(func) => func.1,
        None => {
            return Result {
                data: Vec::new(),
                err: format!("Unknown conversion of {}", func_name),
            }
        }
    };

    match gpx_transform(reader, &mut writer, func) {
        Err(e) => Result {
            data: Vec::new(),
            err: format!("{}", e),
        },
        Ok(_) => Result {
            data: writer.into_inner(),
            err: "".to_string(),
        },
    }
}
