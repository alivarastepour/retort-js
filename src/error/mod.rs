pub mod error_mod {
    use wasm_bindgen::JsValue;

    #[derive(Debug)]
    pub enum Error {
        ParsingError(String),
        ReferenceError(String),
        TypeError(String),
        ResolveError(String),
        DomError(JsValue),
        SerdeWasmBindgenError(serde_wasm_bindgen::Error),
    }
}
