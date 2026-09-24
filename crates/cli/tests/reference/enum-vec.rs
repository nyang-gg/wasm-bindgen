use wasm_bindgen::prelude::*;

// Locks TypeScript/JSDoc for Vec of C-style enums (issue #4480).
// Previously these described as anonymous JsValue arrays (`any[]`).

#[wasm_bindgen]
pub enum Color {
    Green,
    Yellow,
    Red,
}

#[wasm_bindgen]
pub fn enum_vec_echo(values: Vec<Color>) -> Vec<Color> {
    values
}

#[wasm_bindgen]
pub fn option_enum_vec_echo(values: Option<Vec<Color>>) -> Option<Vec<Color>> {
    values
}

#[wasm_bindgen(js_name = "RenamedErr")]
pub enum RustErr {
    One,
    Two,
}

#[wasm_bindgen]
pub fn renamed_err_vec_echo(values: Vec<RustErr>) -> Vec<RustErr> {
    values
}

#[wasm_bindgen(js_namespace = ns)]
pub enum NsErr {
    A,
    B,
}

#[wasm_bindgen]
pub fn ns_err_vec_echo(values: Vec<NsErr>) -> Vec<NsErr> {
    values
}

#[wasm_bindgen(private)]
pub enum HiddenErr {
    X,
    Y,
}

#[wasm_bindgen]
pub fn hidden_err_vec_echo(values: Vec<HiddenErr>) -> Vec<HiddenErr> {
    values
}

// String enums in Vec (nyang-gg fork): previously failed to compile, since only
// C-style enums had the vector impls. Elements cross as JS strings.
#[wasm_bindgen]
pub enum Anim {
    Walk = "walk",
    Idle = "idle",
}

#[wasm_bindgen]
pub fn string_enum_vec_echo(values: Vec<Anim>) -> Vec<Anim> {
    values
}

#[wasm_bindgen]
pub fn option_string_enum_vec_echo(values: Option<Vec<Anim>>) -> Option<Vec<Anim>> {
    values
}