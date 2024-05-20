mod js_evaluator;
mod util;
pub mod evaluator_mod {
    use super::js_evaluator::js_evaluator::get_state_props_evaluator;
    use super::util::evaluator_util::*;
    use serde_wasm_bindgen::{from_value, to_value};
    use wasm_bindgen::JsValue;

    use crate::{component::component_mod::Component, error::error_mod::Error};

    #[derive(Debug)]
    pub enum AttributeTextVariant {
        Boolean,
        String,
        Number,
        Expression,
    }

    #[derive(Debug)]
    pub struct TextInfo<T> {
        pub variant: T,
        pub value: String,
    }

    /// Given a `JsValue` which is the result of evaluating some expression via the `window.Function`
    /// constructor, converts it to a String. `Err` variant is returned when `evaluated_expression`
    /// can't be converted to `string`, `number`, `boolean`, `undefined` or `null`. The `default` parameter is used in the error
    /// message to inform caller of the expression with problem.
    fn fill_evaluated_expression_string_result(
        evaluated_expression: JsValue,
        default: String,
    ) -> Result<String, Error> {
        let result;
        if evaluated_expression.is_string() {
            result = from_value(evaluated_expression).unwrap();
        } else if evaluated_expression.as_f64().is_some() {
            let converted = evaluated_expression.as_f64().unwrap();
            result = converted.to_string();
        } else if evaluated_expression.as_bool().is_some() {
            let converted = evaluated_expression.as_bool().unwrap();
            result = converted.to_string();
        } else if evaluated_expression.is_undefined() {
            result = String::from("undefined");
        } else if evaluated_expression.is_null() {
            result = String::from("null");
        } else {
            return Err(Error::EvaluationError(format!(
                "The following text value didn't have any of the supported(number, boolean, string) types: {default}"
            )));
        }
        Ok(result)
    }

    /// Given a JS expression and context of the component which it was used in, returns a raw String
    /// which is the evaluated result of the expression. In case of error, an `Err` variant is returned
    /// which contains the reason.
    fn evaluate_expression(
        expression: String,
        current_component: &Component,
    ) -> Result<String, Error> {
        let evaluator = get_state_props_evaluator(expression.to_owned());
        let converted_state_result = to_value(current_component.get_state());
        if converted_state_result.is_err() {
            return Err(Error::SerdeWasmBindgenError(
                converted_state_result.unwrap_err(),
            ));
        }
        let converted_prop_result = to_value(current_component.get_props());
        if converted_prop_result.is_err() {
            return Err(Error::SerdeWasmBindgenError(
                converted_prop_result.unwrap_err(),
            ));
        }
        let converted_prop = converted_prop_result.unwrap();
        let converted_state = converted_state_result.unwrap();
        let expression_evaluation_result = evaluator.call2(
            &JsValue::undefined(), // not value for `this` is provided to the evaluator.
            &converted_state,
            &converted_prop,
        );
        if expression_evaluation_result.is_err() {
            let msg: Result<String, serde_wasm_bindgen::Error> =
                from_value(expression_evaluation_result.unwrap_err());
            if msg.is_err() {
                return Err(Error::SerdeWasmBindgenError(msg.unwrap_err()));
            }
            let msg = msg.unwrap();
            return Err(Error::EvaluationError(format!(
                "Failed to evaluate the following expression: {expression}: {msg}"
            )));
        }
        let evaluated_expression = expression_evaluation_result.unwrap();
        let evaluated_expression_string_result =
            fill_evaluated_expression_string_result(evaluated_expression, expression);
        if evaluated_expression_string_result.is_err() {
            return Err(evaluated_expression_string_result.unwrap_err());
        }
        Ok(evaluated_expression_string_result.unwrap())
    }

    /// Given a String which contains a mix of JS expressions and strings plus the context of the component
    /// which it was used in, returns a raw String which is the evaluated result of the expression.
    /// In case of error, an `Err` variant is returned which contains the reason.
    fn evaluate_expression_and_string(
        string_with_expression: String,
        current_component: &Component,
    ) -> Result<String, Error> {
        let mut result: String = String::new();
        let mut current_expression: String = String::new();
        let string_with_expression_chars: Vec<char> = string_with_expression.chars().collect();
        let mut expression_stack = Vec::new();
        for chr in string_with_expression_chars {
            if chr == '{' {
                expression_stack.push('{');
            } else if chr == '}' {
                let head = expression_stack.pop();
                if head.is_none() {
                    return Err(Error::ParsingError(format!("There was an error while parsing the following expression: {string_with_expression}. You have probably messed up some curly brackets.")));
                }
                if expression_stack.is_empty() {
                    let evaluated_expression_result =
                        evaluate_expression(current_expression, current_component);
                    if evaluated_expression_result.is_err() {
                        return Err(evaluated_expression_result.unwrap_err());
                    }
                    let evaluated_expression = evaluated_expression_result.unwrap();
                    result += &evaluated_expression;
                    current_expression = "".to_owned();
                } else {
                    return Err(Error::_InvestigationNeeded(
                        "Observe: when does this happen?".to_owned(),
                    ));
                }
            } else {
                if expression_stack.is_empty() {
                    result += &chr.to_string();
                } else {
                    current_expression += &chr.to_string();
                }
            }
        }
        Ok(result)
    }

    /// determines how should a value in attribute be treated. Returns an `Ok` variant which contains
    /// the value and its type; or `Err` variant with explanation if `text` does not follow the defined
    /// attributes's value pattern.
    fn get_attribute_text_variant(text: String) -> Result<TextInfo<AttributeTextVariant>, Error> {
        let text_trimmed = text.trim();
        if is_a_valid_attribute_value(text_trimmed) {
            let inside_bracket = &text_trimmed[1..text_trimmed.len() - 1];
            let variant;
            let value;
            if attribute_value_is_number(inside_bracket) {
                variant = AttributeTextVariant::Number;
                value = inside_bracket;
            } else if attribute_value_is_bool(inside_bracket) {
                variant = AttributeTextVariant::Boolean;
                value = inside_bracket;
            } else if attribute_value_is_wrapped_in_quotes(inside_bracket) {
                let inside_quotes = &inside_bracket[1..inside_bracket.len() - 1];
                let inside_quotes_has_valid_expression =
                    has_valid_expression_inside(inside_quotes.to_owned());
                if inside_quotes_has_valid_expression {
                    variant = AttributeTextVariant::Expression;
                    value = inside_quotes;
                } else {
                    variant = AttributeTextVariant::String;
                    value = inside_quotes;
                }
            } else {
                variant = AttributeTextVariant::Expression;
                value = text_trimmed;
            }
            let text_info = TextInfo {
                value: value.to_string(),
                variant,
            };
            return Ok(text_info);
        }
        return Err(Error::ParsingError(format!(
            "The following text value didn't have any of the supported types: {text}"
        )));
    }

    /// Evaluates the given attribute value in the context of provided component.
    pub fn evaluate_attribute_value_to_raw_string(
        value: String,
        current_component: &Component,
    ) -> Result<String, Error> {
        let attribute_value_variant_result = get_attribute_text_variant(value.to_owned());
        if attribute_value_variant_result.is_err() {
            return Err(attribute_value_variant_result.unwrap_err());
        }
        let TextInfo { value, variant } = attribute_value_variant_result.unwrap();
        let attr_value;
        match variant {
            AttributeTextVariant::Expression => {
                let attr_value_result = evaluate_expression_and_string(value, current_component);
                if attr_value_result.is_err() {
                    return Err(attr_value_result.unwrap_err());
                }
                attr_value = attr_value_result.unwrap();
            }
            _ => {
                attr_value = value;
            }
        }

        Ok(attr_value)
    }

    /// Evaluates the given text value in the context of provided component.
    pub fn evaluate_text_value_to_raw_string(
        text: &String,
        current_component: &Component,
    ) -> Result<String, Error> {
        let has_valid_exp = has_valid_expression_inside(text.to_owned());
        if has_valid_exp {
            return evaluate_expression_and_string(text.to_owned(), current_component);
        } else {
            return Ok(text.to_owned());
        }
    }

    // fn get_tag_content_variant(text: String) -> Result<(), Error> {
    //     let collected_expression_and_string_result = extract_expression_and_string(text);
    //     if collected_expression_and_string_result.is_err() {
    //         return Err(collected_expression_and_string_result.unwrap_err());
    //     }
    //     let mut result: Vec<DeterminedTagContentTextVariant> = Vec::new();
    //     let collected_expression_and_string = collected_expression_and_string_result.unwrap();
    //     for value in collected_expression_and_string {
    //         match value {
    //             NonDeterminedTagContentTextVariant::Expression(exp) => {}
    //             NonDeterminedTagContentTextVariant::Value(val) => {
    //                 result.push(DeterminedTagContentTextVariant::Value(val))
    //             }
    //         }
    //     }
    //     Ok(())
    // }

    // fn extract_expression_and_string(
    //     string_with_expression: String,
    // ) -> Result<Vec<NonDeterminedTagContentTextVariant>, Error> {
    //     let string_with_expression_chars: Vec<char> = string_with_expression.chars().collect();
    //     let mut result: Vec<NonDeterminedTagContentTextVariant> = Vec::new();
    //     let mut expression_stack = Vec::new();
    //     let mut current_expression: String = String::new();
    //     let mut current_string: String = String::new();
    //     for chr in string_with_expression_chars {
    //         if chr == '{' {
    //             expression_stack.push('{');
    //             if !current_string.is_empty() {
    //                 result.push(NonDeterminedTagContentTextVariant::Value(
    //                     current_string.clone(),
    //                 ));
    //             }
    //             current_string.clear();
    //             current_expression += "{";
    //         } else if chr == '}' {
    //             let head = expression_stack.pop();
    //             if head.is_none() {
    //                 return Err(Error::ParsingError(format!("There was an error while parsing the following expression: {string_with_expression}. You have probably messed up some curly brackets.")));
    //             }
    //             current_expression += &chr.to_string();
    //             if expression_stack.is_empty() {
    //                 result.push(NonDeterminedTagContentTextVariant::Expression(
    //                     current_expression.clone(),
    //                 ));
    //                 current_expression.clear();
    //             }
    //         } else {
    //             if expression_stack.is_empty() {
    //                 current_string += &chr.to_string();
    //             } else {
    //                 current_expression += &chr.to_string();
    //             }
    //         }
    //     }
    //     if !current_string.is_empty() {
    //         result.push(NonDeterminedTagContentTextVariant::Value(
    //             current_string.clone(),
    //         ));
    //     }
    //     Ok(result)
    // }

    // #[derive(Debug, Clone)]
    // pub enum NonDeterminedTagContentTextVariant {
    //     Value(String),
    //     Expression(String),
    // }
    // pub enum DeterminedTagContentTextVariant {
    //     Value(String),
    //     ExpressionNoTag(String),
    //     ExpressionWithTag(String),
    // }

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

    // how should we handle values inside tags?
    // rn, they are all treated as Text, but they most obviously shouldn't.
    // 1- given a Text variant, we need to determine how it should be treated, just like we did
    //    with attributes.
    // 2- why not the same functionality? because they differ. attributes are wrapped inside
    //    curly brackets plus quotation marks, but Text inside tags is not.
    // 3- so we should first determine the type; it can be one of the following:
    //    - only value: <div>hello world</div>
    //    - value with expression(no tag): <div>hello {state.value}</div>
    //    - expression(no tag): <div>{state.value ? "hi" : "Bye"}</div>
    //    - expression(with tag): <div>{state.value > 4 ? <span>xx</span> : <p>yx</p>}</div>
    //    - nested tag expressions: <div>{state.value > 4 ? <span>xx</span> : <p>{state.value > 2 ? <span>aa</span> : <p>b</p>}</p>}</div>
    // 4- what are complications? we can't directly evaluate expressions that contain tags.
    // 5- so overall, we need to cover the following scenarios: only-value, value-expression-no-tag and
    //    value-expression-tag
}
