use wasm_bindgen::prelude::*;
use serde_bin_prot::{error::Error, from_reader, integers::integer, to_writer};
use std::io::Read;
use serde::Serialize;

fn deserialize_test<R: Read>(read: R, test: &Test) -> Result<Test, Error> {
    match test {
        Test::Bool(_) => Ok(Test::Bool(from_reader(read)?)),

    }
}

#[derive(Serialize, Debug)]
#[serde(untagged)] // ensures serializing a test enum just serializes the internal data
enum Test {
    Bool(bool),

}

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    let b = vec![0x01u8];
    let b = deserialize_test(b.as_slice(), &Test::Bool(true)).unwrap();
    alert(&format!("Hello, {}! and from serde-bin-prot: {:?}", name, b));
}
