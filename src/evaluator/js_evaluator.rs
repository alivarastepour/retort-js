/// Contains required logic to evaluate an expression with the Javascript's `new Function` syntax.
pub mod js_evaluator {
    use wasm_bindgen::prelude::wasm_bindgen;
    use web_sys::js_sys::Function;

    const USE_STRICT: &str = "\"use strict\";";
    const STATE_PARAMETER: &str = "state_";
    const PROPS_PARAMETER: &str = "props_";

    /// In JSON strings which contain arrays, `stringify` method is called twice
    /// when converting. Since we need to call the `parse` as many times as we have called the `stringify`,
    /// we must check whether the type of state is `object` or not after the first call to `parse`.
    const CLOSURE: &str =
        "let state=JSON.parse(state_);if(typeof state === 'string'){state=JSON.parse(state)}";
    const RETURN: &str = "return ";

    /// This block interfaces `window.Function` constructor to the rust environment.
    #[wasm_bindgen(js_namespace=window)]
    extern "C" {
        fn Function(arg1: String, arg2: String, function_string: String) -> Function;
    }

    /// Evaluates the result of `function_string` in a JS context using the `window.Function`
    /// constructor. Current component's state and props are the only values in the created
    /// anonymous function's closure.
    pub fn get_state_props_evaluator(function_string: String) -> Function {
        let function_body = USE_STRICT.to_owned() + CLOSURE + RETURN + &function_string;
        Function(
            STATE_PARAMETER.to_owned(),
            PROPS_PARAMETER.to_owned(),
            function_body,
        )
    }

    #[cfg(test)]
    mod tests {
        use serde_wasm_bindgen::to_value;
        use wasm_bindgen::JsValue;
        use wasm_bindgen_test::*;

        use super::*;

        wasm_bindgen_test_configure!(run_in_browser);

        #[wasm_bindgen_test]
        /// tests correctness of the function which is returned from `get_state_props_evaluator` using
        /// `state` parameter.
        fn test_get_state_props_evaluator() {
            let expression = String::from("`${state.name} is ${state.age} years old.`");
            let evaluator = get_state_props_evaluator(expression);

            let temp_js_value_result = to_value("{\"age\":12, \"name\":\"ali\"}");
            assert!(matches!(temp_js_value_result, Ok(_)));

            let temp_js_value = temp_js_value_result.unwrap();
            let resolve_result =
                evaluator.call2(&JsValue::undefined(), &temp_js_value, &temp_js_value);
            console_log!("{:?}", resolve_result);

            assert!(matches!(resolve_result, Ok(_)));

            let result = resolve_result.unwrap();
            assert!(matches!(result.as_string(), Some(val) if val == "ali is 12 years old."))
        }
    }
}
