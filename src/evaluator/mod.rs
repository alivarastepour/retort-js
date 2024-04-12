pub mod evaluator_mod {
    use std::fmt::format;

    use serde_wasm_bindgen::{from_value, to_value};
    use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};
    use web_sys::js_sys::Function;

    use crate::{component::component_mod::Component, error::error_mod::Error};

    #[derive(Debug)]
    pub enum TextVariant {
        Boolean,
        String,
        Number,
        Expression,
    }

    #[derive(Debug)]
    pub struct TextInfo {
        pub variant: TextVariant,
        pub value: String,
    }

    #[wasm_bindgen(js_namespace=window)]
    extern "C" {
        fn Function(arg1: String, arg2: String, function_string: String) -> Function;
    }

    fn get_state_props_evaluator(function_string: String) -> Function {
        let function_string = "const state = JSON.parse(state_);const props = JSON.parse(props_);"
            .to_owned()
            + "return "
            + &function_string;
        Function("state_".to_owned(), "props_".to_owned(), function_string)
    }

    pub fn evaluate_expression(
        expression: String,
        current_component: &Component,
    ) -> Result<String, Error> {
        let evaluator = get_state_props_evaluator(expression.to_owned());
        let expression_evaluation_result = evaluator.call2(
            &JsValue::undefined(),
            &JsValue::from_str(&current_component.state()),
            &JsValue::from_str(&current_component.props()),
        );
        if expression_evaluation_result.is_err() {
            let msg: String =
                from_value(expression_evaluation_result.unwrap_err()).unwrap_or("ERR".to_owned());
            return Err(Error::EvaluationError(format!(
                "Failed to evaluate the following expression: {expression}: {msg}"
            )));
        }
        let evaluated_expression = expression_evaluation_result.unwrap();
        let evaluated_expression_string_result;
        if evaluated_expression.is_string() {
            evaluated_expression_string_result = from_value(evaluated_expression).unwrap();
        } else if evaluated_expression.as_f64().is_some() {
            let evaluated_expression_f64_result = evaluated_expression.as_f64().unwrap();
            evaluated_expression_string_result = evaluated_expression_f64_result.to_string();
        } else if evaluated_expression.as_bool().is_some() {
            let evaluated_expression_bool_result = evaluated_expression.as_bool().unwrap();
            evaluated_expression_string_result = evaluated_expression_bool_result.to_string();
        } else {
            return Err(Error::EvaluationError(
                "The following text value didn't have any of the supported types: {text}"
                    .to_owned(),
            ));
        }

        Ok(evaluated_expression_string_result)
    }

    pub fn get_attribute_text_variant(text: String) -> Result<TextInfo, Error> {
        let text_trimmed = text.trim();
        if text_trimmed.starts_with("{") && text_trimmed.ends_with("}") {
            let inside_bracket = &text_trimmed[1..text_trimmed.len() - 1];
            if inside_bracket.parse::<i64>().is_ok() {
                return Ok(TextInfo {
                    value: inside_bracket.to_owned(),
                    variant: TextVariant::Number,
                });
            } else if inside_bracket.parse::<bool>().is_ok() {
                return Ok(TextInfo {
                    value: inside_bracket.to_owned(),
                    variant: TextVariant::Boolean,
                });
            } else if (inside_bracket.starts_with("\"") && inside_bracket.ends_with("\""))
                || (inside_bracket.starts_with("'") && inside_bracket.ends_with("'"))
            {
                return Ok(TextInfo {
                    value: inside_bracket.to_owned(),
                    variant: TextVariant::String,
                });
            } else {
                return Ok(TextInfo {
                    value: inside_bracket.to_owned(),
                    variant: TextVariant::Expression,
                });
            }
        }
        return Err(Error::ParsingError(format!(
            "The following text value didn't have any of the supported types: {text}"
        )));
    }

    // tokenizer/parser
    //                                    what expression are commonly used within the context of jsx?
    // supported/not supported            1- callback registered using on`Event` attribute -> need to be explicitly imported in presenter
    // supported/not supported            2- attributes and props that are evaluated using a call to a function -> need to be explicitly imported in the presenter
    // supported/not supported            3- passing states and props as they are -> ez
    // supported/not supported            4- using operators(nullish coalescing, ternary operator, etc) to evaluate the value of an attribute or prop. -> new Function syntax
    // won't be considered                5- constant values defined higher in the scope -> not gonna happen
    // supported/not supported            6- primitive data types like string, number and boolean -> they'll be treated like expressions: new Function syntax
    // supported/not supported            7- using operators to render jsx content conditionally
    // supported/not supported            8- using map to render a list of data

    // handling 1:
    // although this can be easily replaced with the addEventListener functionality, we can't ignore the
    // fact that we assign a lot of event handlers using the on`Event` attribute. So we can either ignore this,
    // or make user import used event listeners explicitly in the presenter.

    // handling 2:
    // we can handle this similar to 1, but with a few extra considerations; these functions must have
    // access to state and props of a component through context, or parameter, which raises the second
    // concern; this results in a need for some subscription method, because a change in state or props
    // may cause the return value of these functions change.

    // handling 3:
    // this covers the scenarios where a text node has some references to state or prop values. This
    // can be handled using the `new Function` syntax but handling the conversion between {} and ${}
    // should be taken care of.

    // handling 4:
    // this also looks like a case for the `new Function` evaluation.

    // handling 5:
    // that's the neat part, we DON'T. Just use literals or expressions.

    // handling 6:
    // I guess our current method(we are currently removing the `{`, `}`, starting `"` and ending `"` from
    // attribute values when they have it) covers the case for strings, but additional checks may be added
    // for other primitive types as well.

    // handling 7 and 8:
    // in react, a tag may contain both expression and literal text at the same time as children.
    // handling this requires changes be made to tokenizer and parser modules, to distinguish between
    // literal text and js expressions like conditional rendering and rendering lists using map.
    // or we can ignore this for the time being and assume that each tag has either literal text or
    // expressions and not both of them at the same time.
}
