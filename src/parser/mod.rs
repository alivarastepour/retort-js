pub mod parser_mod {

    // use crate::component::component_mod::Component;
    use crate::component::component_mod::Component;
    use crate::file_util::file_util_mod::ParsedFile;
    use crate::tokenizer::tokenizer_mod::{tokenizer, CurrentState, TokenizerState};
    use serde::{Deserialize, Serialize};
    use serde_wasm_bindgen::from_value;
    use std::collections::HashMap;
    use std::future::IntoFuture;
    use wasm_bindgen_futures::JsFuture;
    use web_sys::console::{log_0, log_1};
    use web_sys::js_sys::Promise;

    use std::fmt::Display;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(module = "/module_resolver/module_resolver.js")]
    extern "C" {
        fn module_resolver(path: &str) -> Promise;
    }

    #[wasm_bindgen]
    pub async fn call_module_resolver(path: &str) -> Result<Component, serde_wasm_bindgen::Error> {
        // Call the JavaScript function
        log_1(&JsValue::from_str("here1"));
        let promise = module_resolver(path);
        log_1(&JsValue::from_str("here2"));
        let future = JsFuture::from(promise);
        log_1(&JsValue::from_str("here3"));
        let result = future.await.unwrap();
        log_1(&JsValue::from_str("here4"));
        let result: Result<Component, serde_wasm_bindgen::Error> = from_value(result);
        log_1(&JsValue::from_str("here5"));
        return result;
    }
    pub enum NodeType {
        Component(Component), //component object
        Tag(String),          // tag name
        Text(String),         // text content
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

    // TODOS:
    // 1- empty strings are being tokenized as Texts, not cool.

    pub fn parse_vdom_from_string(parsed_file: ParsedFile) -> Vec<VirtualNode> {
        let ParsedFile { imports, markup } = parsed_file;
        let mut get_next_token = tokenizer(markup);
        let mut stack: Vec<VirtualNode> = Vec::new();
        let mut stack_size: usize = 0;
        let mut vdom: Vec<VirtualNode> = Vec::new();
        loop {
            let CurrentState { state, token } = get_next_token();
            match state {
                TokenizerState::Finalized => {
                    break;
                }
                TokenizerState::Error(err) => {
                    panic!("[ERROR] {err}")
                }
                TokenizerState::ClosingAngleBracket => {}
                TokenizerState::OpenAngleBracket => {}
                TokenizerState::TagNameClose => {
                    let completed_node = stack.pop().unwrap();

                    // vdom.push(completed_node);
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
                        panic!(
                            "{}",
                            format!(
                            "An import statement for {token} was supposed to exist, but it didn't."
                        )
                        )
                    }
                    let component_path = component_path.unwrap();
                    log_1(&JsValue::from_str("here??"));
                    let component = call_module_resolver(&component_path);
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
                    // vdom.push(completed_node);
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
        return vdom;
    }
}
