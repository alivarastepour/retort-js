pub mod parser_mod {

    use crate::component::component_mod::Component;
    use crate::error::error_mod::Error;
    use crate::presenter::presenter_mod::ParsedPresenter;
    use crate::tokenizer::tokenizer_mod::{tokenizer, CurrentState, TokenizerState};
    use serde_wasm_bindgen::from_value;
    use std::collections::HashMap;
    use wasm_bindgen_futures::JsFuture;
    use web_sys::js_sys::{Function, Promise};

    use std::fmt::Display;
    use wasm_bindgen::prelude::*;

    pub enum NodeType {
        Component(Component), //component object
        Tag(String),          // tag name
        Text(String),         // text content
    }

    #[wasm_bindgen(module = "/module_resolver/module_resolver.js")]
    extern "C" {
        fn module_resolver(path: &str) -> Promise;
    }

    pub async fn call_module_resolver(path: &str) -> Result<Component, Error> {
        let path = path.replace("\"", "").replace(";", "");
        let promise = module_resolver(&path);
        let future = JsFuture::from(promise);
        let result = future.await;
        if let Result::Err(err) = &result {
            let msg = err.as_string();
            if let Option::None = msg {
                let msg = "Expected an error message to be string, but wasn't.".to_owned();
                return Err(Error::TypeError(msg));
            }
            let msg = msg.unwrap();
            return Err(Error::ResolveError(msg));
        }
        let result = result.unwrap();
        let component_result: Result<Component, serde_wasm_bindgen::Error> = from_value(result);
        if let Result::Err(err) = component_result {
            return Err(Error::SerdeWasmBindgenError(err));
        }
        let component = component_result.unwrap();
        Ok(component)
    }

    impl Display for NodeType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Component(component) => f.write_str("component"),
                Self::Tag(tag) => f.write_str(&format!("tag: {tag}")),
                Self::Text(text) => f.write_str(&format!("text: {text}")),
            }
        }
    }

    pub struct VirtualNode {
        pub node_type: NodeType,
        pub attributes: HashMap<String, String>,
        pub children: Vec<VirtualNode>,
    }

    impl Clone for VirtualNode {
        fn clone(&self) -> Self {
            Self {
                attributes: self.attributes.clone(),
                children: self.children.clone(),
                node_type: NodeType::Tag("".to_owned()),
            }
        }
    }

    fn get_parser_return_value(stack: Vec<VirtualNode>) -> Result<VirtualNode, Error> {
        if stack.len() == 1 {
            let top = stack.get(0).unwrap();
            return Ok(top.clone());
        }
        let msg = "Presenter of each component must be wrapped inside one and only one wrapper."
            .to_owned();
        return Err(Error::ParsingError(msg));
    }

    pub async fn parse_vdom_from_string(
        parsed_file: ParsedPresenter,
    ) -> Result<VirtualNode, Error> {
        let ParsedPresenter { imports, markup } = parsed_file;
        let mut get_next_token = tokenizer(markup);
        let mut stack: Vec<VirtualNode> = Vec::new();
        let mut stack_size: usize = 0;
        let mut vdom: Vec<VirtualNode> = Vec::new();
        loop {
            let CurrentState { state, token } = get_next_token();
            match state {
                TokenizerState::Finalized => {
                    return get_parser_return_value(stack);
                }
                TokenizerState::Error(err) => {
                    let err = format!("[ERROR] {err}");
                    return Err(Error::ParsingError(err));
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
                    let tag = NodeType::Text(token);
                    let new_node = VirtualNode {
                        node_type: tag,
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
                        let msg =
                            "An import statement for {token} was supposed to exist, but it didn't."
                                .to_owned();
                        return Err(Error::ReferenceError(msg));
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
                    })
                } // note that all Components are assumed to be self-closing at this point. The other variant is not handled
                TokenizerState::Props => {
                    let owner_node = stack.get_mut(stack_size - 1).unwrap();
                    let attrs = &mut owner_node.attributes;
                    let mut key_value_split = token.split('=');
                    let key = key_value_split.next().unwrap().to_owned();
                    let value = key_value_split.next().unwrap().to_owned();
                    attrs.insert(key, value);
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
                TokenizerState::CloseAngleBracket => {}
                _ => {}
            }
        }
        return Err(Error::ParsingError("()".to_owned()));
    }
}
