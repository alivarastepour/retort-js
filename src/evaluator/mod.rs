pub mod evaluator_mod {
    use wasm_bindgen::prelude::wasm_bindgen;
    use web_sys::js_sys::Function;

    #[wasm_bindgen(js_namespace=window)]
    extern "C" {
        fn Function(arg1: String, arg2: String, function_string: String) -> Function;
    }

    fn get_state_props_evaluator(function_string: String) -> Function {
        Function("state".to_owned(), "props".to_owned(), function_string)
        // let b = SomeThing {
        //     exercise: "Running".to_owned(),
        // };
        // let a = to_value(&b).unwrap();
        // let res = function.call1(&JsValue::undefined(), &a);
        // if res.is_err() {
        //     log_1(&JsValue::from_str("err!"));
        //     log_1(&res.unwrap_err());
        // } else {
        //     log_1(&JsValue::from_str("no err!!!"));
        //     log_1(&res.unwrap());
        // }
    }

    fn evaluate_expression(expression: String) {
        let evaluator = get_state_props_evaluator(expression);
        // expression.replace_range(range, replace_with)
    }

    // what expression are commonly used within the context of jsx?
    // 1- callback registered using on`Event` attribute -> need to be explicitly imported in presenter
    // 2- attributes and props that are evaluated using a call to a function -> need to be explicitly imported in the presenter
    // 3- passing states and props as they are -> ez
    // 4- using operators(nullish coalescing, ternary operator, etc) to evaluate the value of an attribute or prop. -> new Function syntax
    // 5- constant values defined higher in the scope -> not gonna happen
    // 6- primitive data types like string, number and boolean -> they'll be treated like expressions: new Function syntax
    // 7- using operators to render jsx content conditionally
    // 8- using map to render a list of data

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
