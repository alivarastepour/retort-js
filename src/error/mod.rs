/// provides functionality which is used to handle errors that occurred during any process.
pub mod error_mod {
    use serde_wasm_bindgen::{from_value, Error as SerdeWasmBindgenError};
    use std::fmt::Display;
    use wasm_bindgen::JsValue;
    use web_sys::{console::error_1, Document, Element};

    use crate::dom::dom_mod::{get_app_wrapper, get_document};

    #[derive(Debug)]
    /// An enum which contains different types of errors and their associated data(error objects, strings, etc).
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
                    let msg_result: Result<String, SerdeWasmBindgenError> = from_value(err.clone());
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

    /// returns a string corresponding to `Error` variant.
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

    /// Logs the error to the console using `console.error` function.
    fn error_log(error: &Error) {
        let error_string = error.to_string();
        let js_value_error_string = JsValue::from_str(&error_string);
        error_1(&js_value_error_string);
    }

    /// Displays the error message in the DOM, removing all its content to emphasize severity of error.
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

    /// Exposes error logging and displaying logic publicly.
    pub fn error_handler(error: Error) {
        let wrapper = get_app_wrapper();
        let document = get_document();
        error_log(&error);
        if wrapper.is_ok() && document.is_ok() {
            error_display(&error, wrapper.unwrap(), document.unwrap());
        }
    }

    #[cfg(test)]
    mod tests {
        use serde::{Deserialize, Serialize};
        use wasm_bindgen_test::*;

        use super::*;
        #[test]
        /// tests `Error` variants which their associated info is `String`
        fn test_error_to_string_1() {
            let message = String::from("Some error message.");
            let errors = vec![
                Error::ParsingError(message.clone()),
                Error::EvaluationError(message.clone()),
                Error::ReferenceError(message.clone()),
                Error::ResolveError(message.clone()),
                Error::_InvestigationNeeded(message.clone()),
                Error::TypeError(message.clone()),
            ];
            for error in errors {
                let error_string = error.to_string();
                let error_indicator = get_variant_text(&error);
                assert!(error_string == format!("{error_indicator}: {message}"));
            }
        }

        wasm_bindgen_test_configure!(run_in_browser);

        #[wasm_bindgen_test]
        /// tests `Error` variants which their associated info is `serde_wasm_bindgen::Error`
        fn test_error_to_string_2() {
            #[derive(Serialize, Deserialize, Debug)]
            struct Temp {
                num: i8,
            }

            let value: Result<Temp, SerdeWasmBindgenError> = from_value(JsValue::from_str(""));
            if value.is_err() {
                let error_unwrapped = value.unwrap_err();
                let error_string = &error_unwrapped.to_string();
                let error = Error::SerdeWasmBindgenError(error_unwrapped);
                let error_indicator = get_variant_text(&error);
                assert!(error.to_string() == format!("{error_indicator}: {error_string}"));
            } else {
                assert!(false);
            }
        }

        #[wasm_bindgen_test]
        /// tests `Error` variants which their associated info is `JsValue`
        #[ignore = "https://github.com/alivarastepour/retort-js/issues/36"]
        fn test_error_to_string_3() {
            let document_result = get_document();
            assert!(matches!(document_result, Ok(_)));
            let document = document_result.unwrap();
            let element_result = document.create_element("");
            if element_result.is_err() {
                let error_js_value = element_result.unwrap_err();
                let error = Error::DomError(error_js_value.clone());
                let msg_result = format!("{:?}", error_js_value);
                let error_string = error.to_string();
                let error_indicator = get_variant_text(&error);
                assert!(error_string == format!("{error_indicator}: {msg_result}"));
            } else {
                console_log!("{:?}", element_result.unwrap());
                assert!(false);
            }
        }

        #[wasm_bindgen_test]
        /// Creates a dummy error and checks if the error is properly reflected in DOM. There
        /// is really no clean way of checking if the error is also reflected in the browser's console,
        /// so we'll ignore it.
        fn test_error_display() {
            let document_result = get_document();
            assert!(matches!(document_result, Ok(_)));
            let document = document_result.unwrap();

            let wrapper_result = document.create_element("div");
            assert!(matches!(wrapper_result, Ok(_)));
            let wrapper = wrapper_result.unwrap();

            let body_result = document.body();
            assert!(matches!(body_result, Some(_)));
            let body = body_result.unwrap();

            let attribute_result = wrapper.set_attribute("id", "root");
            let append_result = body.append_child(&wrapper);
            assert!(matches!(attribute_result, Ok(_)) && matches!(append_result, Ok(_)));

            let error = Error::ParsingError("Some error".to_owned());
            error_display(&error, wrapper, document);

            let document_result = get_document();
            assert!(matches!(document_result, Ok(_)));
            let document = document_result.unwrap();

            let root = document.get_element_by_id("root");
            assert!(matches!(root, None));

            let error_wrapper_option_result = document.query_selector("body > div");
            assert!(matches!(error_wrapper_option_result, Ok(_)));
            let error_wrapper_option = error_wrapper_option_result.unwrap();

            assert!(matches!(error_wrapper_option, Some(_)));
            let error_wrapper = error_wrapper_option.unwrap();

            let children_count = error_wrapper.child_element_count();
            assert!(children_count == 1);

            let error_message = error_wrapper.text_content();
            assert!(
                matches!(error_message, Some(value) if value.contains("Some error") && value.contains(&get_variant_text(&error)))
            )
        }
    }
}
