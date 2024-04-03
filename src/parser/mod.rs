pub mod parser_mod {
    use web_sys::Node;

    // use crate::component::component_mod::Component;
    use crate::component::component_mod::Component;
    use crate::file_util::file_util_mod::ParsedFile;
    use crate::tokenizer::tokenizer_mod::{tokenizer, CurrentState, TokenizerState};
    use std::collections::HashMap;
    use std::fmt::{format, Display};

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

                    // stack.push(new_node);
                    // stack_size += 1;
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
                TokenizerState::Component => {}
                _ => {}
            }
        }
        return vdom;
    }
}
