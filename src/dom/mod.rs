pub mod dom_mod {
    use std::collections::HashMap;

    use serde_wasm_bindgen::to_value;
    use wasm_bindgen::JsValue;
    use web_sys::{console::log_1, window, Document, Element, Text, Window};

    use crate::{
        component::component_mod::Component,
        error::error_mod::Error,
        evaluator::evaluator_mod::evaluate_value_to_raw_string,
        parser::parser_mod::{NodeType, VirtualNode},
    };

    const APP_WRAPPER_ID: &str = "root";

    /// Returns an `Ok` variant if window object was found successfully; an `Err` variant otherwise.
    fn get_window() -> Result<Window, Error> {
        let window_option = window();
        if window_option.is_none() {
            return Err(Error::ReferenceError(
                "Could not find the window object.".to_owned(),
            ));
        }
        let window = window_option.unwrap();
        Ok(window)
    }

    /// Returns an `Ok` variant if document object was found successfully;
    /// an `Err` variant otherwise.
    pub fn get_document() -> Result<Document, Error> {
        let window_result = self::get_window();
        if window_result.is_err() {
            return Err(window_result.unwrap_err());
        }
        let window = window_result.unwrap();

        let document_option = window.document();
        if document_option.is_none() {
            return Err(Error::ReferenceError(
                "Could not find the document object.".to_owned(),
            ));
        }
        let document = document_option.unwrap();
        Ok(document)
    }

    /// Returns an `Ok` variant if root node with id of `self::APP_WRAPPER_ID` was found successfully;
    /// an `Err` variant otherwise.
    pub fn get_app_wrapper() -> Result<Element, Error> {
        let document_result = self::get_document();
        if document_result.is_err() {
            return Err(Error::ReferenceError(
                "Could not find the document object.".to_owned(),
            ));
        }
        let document = document_result.unwrap();
        let app_wrapper_option = document.get_element_by_id(self::APP_WRAPPER_ID);
        if app_wrapper_option.is_none() {
            return Err(Error::ReferenceError(format!("Could not find the root element. Make sure you have a root element with id of `{APP_WRAPPER_ID}`.")));
        }
        let app_wrapper = app_wrapper_option.unwrap();
        Ok(app_wrapper)
    }

    /// Adds attributes to the provided element. Note that attribute values are evaluated. Returns an
    /// `Err` if an error occurs during evaluation or setting attributes.
    fn add_attributes(
        current_component: &Component,
        attributes: &HashMap<String, String>,
        element: &Element,
    ) -> Result<(), Error> {
        for (key, value) in attributes {
            let attr_value_result =
                evaluate_value_to_raw_string(value.to_owned(), current_component);
            if attr_value_result.is_err() {
                return Err(attr_value_result.unwrap_err());
            }
            let attr_value = attr_value_result.unwrap();
            let set_attribute_result = element.set_attribute(key, &attr_value);
            if set_attribute_result.is_err() {
                return Err(Error::DomError(set_attribute_result.unwrap_err()));
            }
        }
        Ok(())
    }

    /// Recursively calls the `self::construct_dom` function on each child of the current virtual node.
    /// Returns an `Err` variant if an `Err` variant is returned from any of the calls to `self::construct_dom`.
    fn add_children(
        children: &Vec<VirtualNode>,
        current_component: &Component,
        element: &Element,
        document: &Document,
    ) -> Result<(), Error> {
        for child in children {
            let node_type = &child.node_type;
            let construct_result;
            match node_type {
                NodeType::Component(component) => {
                    construct_result =
                        self::construct_dom(child, &component, &element, &document, true);
                }
                _ => {
                    construct_result =
                        self::construct_dom(child, current_component, &element, &document, false);
                }
            }
            if construct_result.is_err() {
                return Err(construct_result.unwrap_err());
            }
        }
        Ok(())
    }

    /// Constructs a tag element from the given virtual node and appends it to the provided parent.
    /// Returns an `Err` variant which explains what went wrong, `Ok` otherwise.
    fn construct_tag(
        current_root: &VirtualNode,
        current_component: &Component,
        parent: &Element,
        document: &Document,
        tag_name: String,
    ) -> Result<(), Error> {
        let new_element_result = document.create_element(&tag_name);
        if new_element_result.is_err() {
            return Err(Error::DomError(new_element_result.unwrap_err()));
        }
        let new_element = new_element_result.unwrap();

        let attributes = &current_root.attributes;
        let add_attributes_result = add_attributes(current_component, attributes, &new_element);
        if add_attributes_result.is_err() {
            return Err(add_attributes_result.unwrap_err());
        }

        let append_child_result = parent.append_child(&new_element);
        if append_child_result.is_err() {
            return Err(Error::DomError(append_child_result.unwrap_err()));
        }

        let children = &current_root.children;
        return add_children(children, current_component, &new_element, document);
    }

    /// Crates a text node and appends it to the provided parent.
    /// Returns an `Err` variant which explains what went wrong, `Ok` otherwise.
    fn construct_text(
        text: &String,
        parent: &Element,
        current_component: &Component,
    ) -> Result<(), Error> {
        // let text_value_result = evaluate_value_to_raw_string(text.to_owned(), current_component);
        // if text_value_result.is_err() {
        //     return Err(text_value_result.unwrap_err());
        // }
        // let text = text_value_result.unwrap();
        let text_element_result = Text::new_with_data(&text);
        if text_element_result.is_err() {
            return Err(Error::DomError(text_element_result.unwrap_err()));
        }
        let text_element = text_element_result.unwrap();

        let append_text_result = parent.append_child(&text_element);
        if append_text_result.is_err() {
            return Err(Error::DomError(append_text_result.unwrap_err()));
        }
        Ok(())
    }

    /// Constructs DOM using the provided virtual node and component as a root. Returns `Ok` variant
    /// if no errors are encountered while building DOM; an `Err` variant otherwise, explaining what
    /// went wrong.
    fn construct_dom(
        current_root: &VirtualNode,
        current_component: &Component,
        parent: &Element,
        document: &Document,
        caller_is_component_node: bool,
    ) -> Result<(), Error> {
        let node_type = &current_root.node_type;
        let res;
        match node_type {
            NodeType::Component(component) => {
                res = construct_dom(&component.vdom, component, parent, document, true);
                // return res;
            }
            NodeType::Tag(tag_name) => {
                res = construct_tag(
                    current_root,
                    current_component,
                    parent,
                    document,
                    tag_name.to_owned(),
                );
            }
            NodeType::Text(text) => {
                res = construct_text(text, parent, current_component);
            }
        }
        // if caller_is_component_node {
        //     let mount_res = current_component.call_component_did_mount();
        //     if mount_res.is_err() {
        //         let msg = mount_res.unwrap_err().as_string().unwrap_or("".to_owned());
        //         return Err(Error::_InvestigationNeeded(format!(
        //             "Call to component did mount resulted in error: {msg}"
        //         )));
        //     }
        // }
        return res;
    }

    /// Encapsulates the logic of preparing arguments for `self::construct_dom` function
    pub fn construct_dom_wrapper(root_component: &Component) -> Result<(), Error> {
        let document_result = get_document();
        if document_result.is_err() {
            return Err(document_result.unwrap_err());
        }

        let parent_result = get_app_wrapper();
        if parent_result.is_err() {
            return Err(parent_result.unwrap_err());
        }

        let document = document_result.unwrap();
        let parent = parent_result.unwrap();

        let construct_dom_result = construct_dom(
            &root_component.vdom,
            root_component,
            &parent,
            &document,
            true,
        );
        if construct_dom_result.is_err() {
            let msg = construct_dom_result.unwrap_err();
            match msg {
                Error::DomError(err) => log_1(&err),
                Error::EvaluationError(err) => {
                    let a = to_value(&err).unwrap();
                    log_1(&a);
                }
                Error::TypeError(err) => {
                    let a = to_value(&err).unwrap();
                    log_1(&a);
                }
                Error::ResolveError(err) => {
                    let a = to_value(&err).unwrap();
                    log_1(&a);
                }
                Error::ParsingError(err) => {
                    let a = to_value(&err).unwrap();
                    log_1(&a);
                }
                _ => {
                    log_1(&JsValue::from_str("err"));
                }
            }
        }
        Ok(())
    }
}

// <div>hello world</div>
// <div>hello {state.value}</div>
// <div>{state.value > 4 ? <span>xx</span> : <p>yx</p>}</div>
// <div>{state.value > 4 ? <span>xx</span> : <p>{state.value > 2 ? <span>aa</span> : <p>b</p>}</p>}</div>
// <div>{state.value ?? state.x ?? state.y ?? 0}</div>
// <div>{state.value ? "hi" : "Bye"}</div>
