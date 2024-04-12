pub mod error_mod {
    use wasm_bindgen::JsValue;

    #[derive(Debug)]
    pub enum Error {
        ParsingError(String),
        ReferenceError(String),
        TypeError(String),
        ResolveError(String),
        EvaluationError(String),
        DomError(JsValue),
        _InvestigationNeeded(String),
        SerdeWasmBindgenError(serde_wasm_bindgen::Error),
    }
}
