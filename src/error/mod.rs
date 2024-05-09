pub mod error_mod {
    use serde_wasm_bindgen::from_value;
    use std::fmt::Display;
    use wasm_bindgen::JsValue;
    use web_sys::{
        console::{error_1, log_1},
        Document, Element,
    };

    use crate::dom::dom_mod::{get_app_wrapper, get_document};

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

    impl Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let indicator = get_variant_text(&self);
            match &self {
                Error::DomError(err) => {
                    let msg_result: Result<String, serde_wasm_bindgen::Error> =
                        from_value(err.clone());
                    let msg =
                        msg_result.unwrap_or("Could not parse error message properly.".to_owned());
                    let full_message = indicator + ": " + &msg;
                    f.write_str(&full_message)
                }

                Error::SerdeWasmBindgenError(err) => {
                    let msg = err.to_string();
                    let full_message = indicator + ": " + &msg;
                    f.write_str(&full_message)
                }
                Error::ParsingError(err)
                | Error::EvaluationError(err)
                | Error::ReferenceError(err)
                | Error::ResolveError(err)
                | Error::_InvestigationNeeded(err)
                | Error::TypeError(err) => {
                    let full_message = indicator + ": " + err;
                    f.write_str(&full_message)
                }
            }
        }
    }

    fn get_variant_text(error: &Error) -> String {
        match &error {
            Error::DomError(_) => "DOM error".to_owned(),
            Error::ParsingError(_) => "Parsing error".to_owned(),
            Error::EvaluationError(_) => "Evaluation error".to_owned(),
            Error::TypeError(_) => "Type error".to_owned(),
            Error::_InvestigationNeeded(_) => "Unknown error".to_owned(),
            Error::ReferenceError(_) => "Reference error".to_owned(),
            Error::SerdeWasmBindgenError(_) => "Serialization error".to_owned(),
            Error::ResolveError(_) => "Resolve error".to_owned(),
        }
    }

    fn error_log(error: &Error) {
        let error_string = error.to_string();
        let js_value_error_string = JsValue::from_str(&error_string);
        error_1(&js_value_error_string);
    }

    fn error_display(error: &Error, wrapper: Element, document: Document) {
        let error_wrapper_result = document.create_element("div");
        let error_subtitle_result = document.create_element("p");
        if error_wrapper_result.is_err() || error_subtitle_result.is_err() {
            return;
        }

        let error_wrapper = error_wrapper_result.unwrap();
        let error_subtitle = error_subtitle_result.unwrap();
        let error_wrapper_style_result = error_wrapper.set_attribute(
            "style",
            "
            line-height:30px;
            background-color:#570606;
            font-family:sans-serif;
            font-size:1.5rem;
            padding:2rem;
            color:#fff;
            font-weight:700;
            border-radius:8px
            ",
        );
        let error_subtitle_style_result = error_subtitle.set_attribute(
            "style",
            "
            color:#eee;
            font-size:0.8rem;
            font-family:sans-serif
            ",
        );
        if error_wrapper_style_result.is_err() || error_subtitle_style_result.is_err() {
            return;
        }

        error_wrapper.set_text_content(Some(&error.to_string()));
        error_subtitle.set_text_content(Some("See console for more details"));

        let append_result = error_wrapper.append_child(&error_subtitle);
        if append_result.is_err() {
            return;
        }

        let body_result = document.body();
        if body_result.is_none() {
            return;
        }

        let body = body_result.unwrap();
        wrapper.remove();
        let replacement_result = body.append_child(&error_wrapper);
        if replacement_result.is_err() {
            return;
        }
    }

    pub fn error_handler(error: Error) {
        let wrapper = get_app_wrapper();
        let document = get_document();
        error_log(&error);
        if wrapper.is_ok() && document.is_ok() {
            error_display(&error, wrapper.unwrap(), document.unwrap());
        }
    }
}
