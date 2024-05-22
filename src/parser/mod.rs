pub mod parser_mod {

    use crate::component::component_mod::Component;
    use crate::const_util::const_util_mod::ATTRIBUTE_KEY_VALUE_SEPARATOR;
    use crate::error::error_mod::Error as CustomError;
    use crate::evaluator::evaluator_mod::{
        evaluate_attribute_value_to_raw_string, evaluate_text_value_to_raw_string,
    };
    use crate::presenter::presenter_mod::ParsedPresenter;
    use crate::tokenizer::tokenizer_mod::{tokenizer, CurrentState, TokenizerState};
    use serde::{Deserialize, Serialize};
    use serde_wasm_bindgen::from_value;
    use std::collections::HashMap;
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_futures::JsFuture;
    use web_sys::js_sys::Promise;

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub enum NodeType {
        Component(Component), //component object
        Tag(String),          // tag name
        Text(String),         // text content
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct VirtualNode {
        pub node_type: NodeType,
        pub attributes: HashMap<String, String>,
        pub children: Vec<VirtualNode>,
    }

    // This path should be kept in sync with where the specified file actually resides.
    #[wasm_bindgen(module = "/module_resolver/module_resolver.js")]
    extern "C" {
        fn module_resolver(path: &str) -> Promise;
    }

    /// A wrapper function that calls the `module_resolver` function which is defined in Javascript.
    /// The resolved value of a call to `module_resolver` is supposedly a `Component` object;
    /// if it was, an `Ok` variant is returned which contains the `Component` object, `Err` otherwise.
    /// Note that except for the root component, other modules which contain exported `Component` objects
    /// rely on being called from here; other than that, they won't be executed at all.
    pub async fn call_module_resolver(path: &str) -> Result<Component, CustomError> {
        let promise = module_resolver(&path);
        let future = JsFuture::from(promise);
        let result = future.await;
        if let Result::Err(err) = &result {
            let msg = err.as_string();
            if let Option::None = msg {
                let msg = "Expected an error message to be string, but wasn't.".to_owned();
                return Err(CustomError::TypeError(msg));
            }
            let msg = msg.unwrap();
            return Err(CustomError::ResolveError(msg));
        }
        let result = result.unwrap();
        let component_result: Result<Component, serde_wasm_bindgen::Error> = from_value(result);
        if let Result::Err(err) = component_result {
            return Err(CustomError::SerdeWasmBindgenError(err));
        }
        let component = component_result.unwrap();
        Ok(component)
    }

    /// The final stack which contains the info of VDOM, should only have one item in the end; which
    /// in this case, an `Ok` variant containing a VirtualNode is returned. Other than that an `Err`
    /// variant is returned explaining the reason.
    fn get_parser_return_value(stack: Vec<VirtualNode>) -> Result<VirtualNode, CustomError> {
        if stack.len() == 1 {
            let top = stack.get(0).unwrap();
            return Ok(top.clone());
        }
        let msg = "Presenter of each component must be wrapped inside one and only one wrapper."
            .to_owned();
        return Err(CustomError::ParsingError(msg));
    }

    /// Given an object of type `ParsedPresenter`, constructs a vdom using the `tokenizer` module.
    /// If an error is encountered, an `Err` variant is returned explaining why; `Ok` otherwise,
    /// which contains a `VirtualNode` object.
    pub async fn parse_vdom_from_string(
        parsed_file: &ParsedPresenter,
        component: &mut Component,
    ) -> Result<VirtualNode, CustomError> {
        let ParsedPresenter { imports, markup } = parsed_file;
        let mut get_next_token = tokenizer(markup.to_owned());
        let mut stack: Vec<VirtualNode> = Vec::new();
        let mut stack_size: usize = 0;
        let mut vdom: Vec<VirtualNode> = Vec::new();
        loop {
            let next_token_result = get_next_token();
            if next_token_result.is_err() {
                return Err(next_token_result.unwrap_err());
            }
            let CurrentState { state, token } = next_token_result.unwrap();
            match state {
                TokenizerState::Finalized => {
                    return get_parser_return_value(stack);
                }
                TokenizerState::TagNameClose => {
                    let completed_node = stack.pop().unwrap();
                    stack_size -= 1;
                    if stack_size != 0 {
                        let parent_node = stack.get_mut(stack_size - 1).unwrap();
                        let children = &mut parent_node.children;
                        children.push(completed_node);
                    } else {
                        vdom.push(completed_node);
                        break;
                    }
                }
                TokenizerState::Text => {
                    let evaluated_text_result =
                        evaluate_text_value_to_raw_string(&token, &component);
                    if evaluated_text_result.is_err() {
                        return Err(evaluated_text_result.unwrap_err());
                    }
                    let text = evaluated_text_result.unwrap();
                    let text = NodeType::Text(text);
                    let new_node = VirtualNode {
                        node_type: text,
                        attributes: HashMap::new(),
                        children: Vec::new(),
                    };
                    if stack_size != 0 {
                        let parent_node = stack.get_mut(stack_size - 1).unwrap();
                        let children = &mut parent_node.children;
                        children.push(new_node);
                    } else {
                        vdom.push(new_node);
                        break;
                    }
                }
                TokenizerState::TagNameOpen => {
                    let tag = NodeType::Tag(token);
                    let new_node = VirtualNode {
                        node_type: tag,
                        attributes: HashMap::new(),
                        children: Vec::new(),
                    };
                    stack.push(new_node);
                    stack_size += 1;
                }
                TokenizerState::Component => {
                    let component_path = imports.get(&token);
                    if let Option::None = component_path {
                        let msg = format!(
                            "An import statement for `{token}` was supposed to exist, but it didn't."
                        );
                        return Err(CustomError::ReferenceError(msg));
                    }
                    let component_path = component_path.unwrap();
                    let component = call_module_resolver(&component_path).await;
                    if let Result::Err(err) = component {
                        return Err(err);
                    }
                    let component = component.unwrap();
                    stack.push(VirtualNode {
                        attributes: HashMap::new(),
                        children: Vec::new(),
                        node_type: NodeType::Component(component),
                    });
                    stack_size += 1;
                } // note that all Components are assumed to be self-closing at this point. The other variant is not handled
                TokenizerState::Props => {
                    let owner_node = stack.get_mut(stack_size - 1).unwrap();
                    let attrs = &mut owner_node.attributes;
                    let key_value_split = token.split_once(ATTRIBUTE_KEY_VALUE_SEPARATOR).unwrap();
                    let attr_value_result = evaluate_attribute_value_to_raw_string(
                        key_value_split.1.to_owned(),
                        &component,
                    );
                    if attr_value_result.is_err() {
                        return Err(attr_value_result.unwrap_err());
                    }
                    let attr_value = attr_value_result.unwrap();
                    attrs.insert(key_value_split.0.to_owned(), attr_value);
                }
                TokenizerState::SelfClosingAngleBracket => {
                    let completed_node = stack.pop().unwrap();
                    stack_size -= 1;
                    if stack_size != 0 {
                        let parent_node = stack.get_mut(stack_size - 1).unwrap();
                        let children = &mut parent_node.children;
                        children.push(completed_node);
                    } else {
                        vdom.push(completed_node);
                        break;
                    }
                }
                _ => {}
            }
        }
        return get_parser_return_value(vdom);
    }
}
