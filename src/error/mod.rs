pub mod error_mod {

    #[derive(Debug)]
    pub enum Error {
        ParsingError(String),
        ReferenceError(String),
        TypeError(String),
        ResolveError(String),
        SerdeWasmBindgenError(serde_wasm_bindgen::Error),
    }
}
