pub mod dom_mod {
    use std::collections::HashMap;

    use serde_wasm_bindgen::to_value;
    use wasm_bindgen::JsValue;
    use web_sys::{
        console::{log_1, time, time_end},
        window, Document, Element, Text, Window,
    };

    use crate::{
        component::component_mod::Component,
        error::error_mod::Error,
        evaluator::evaluator_mod::{
            evaluate_expression, evaluate_expression_and_string, get_attribute_text_variant,
            TextInfo, TextVariant,
        },
        parser::parser_mod::{NodeType, VirtualNode},
        util::util_mod::{option_has_value, result_is_ok},
    };

    const APP_WRAPPER_ID: &str = "root";

    fn get_window() -> Result<Window, Error> {
        let window_option = window();
        let window_has_value = option_has_value(&window_option);
        if !window_has_value {
            let msg = "Could not find the window object.".to_owned();
            return Err(Error::ReferenceError(msg));
        }
        let window = window_option.unwrap();
        Ok(window)
    }

    pub fn get_document() -> Result<Document, Error> {
        let window_result = self::get_window();
        let window_is_ok = result_is_ok(&window_result);
        if !window_is_ok {
            return Err(window_result.unwrap_err());
        }
        let window = window_result.unwrap();

        let document_option = window.document();
        let document_has_value = option_has_value(&document_option);
        if !document_has_value {
            let msg = "Could not find the document object.".to_owned();
            return Err(Error::ReferenceError(msg));
        }
        let document = document_option.unwrap();

        Ok(document)
    }

    pub fn get_app_wrapper() -> Result<Element, Error> {
        let document_result = self::get_document();
        let document_is_ok = result_is_ok(&document_result);
        if !document_is_ok {
            let msg = "Could not find the document object.".to_owned();
            return Err(Error::ReferenceError(msg));
        }
        let document = document_result.unwrap();
        let app_wrapper_option = document.get_element_by_id(self::APP_WRAPPER_ID);
        let app_wrapper_has_value = option_has_value(&app_wrapper_option);
        if !app_wrapper_has_value {
            let msg = format!("Could not find the root element. Make sure you have a root element with id of `{APP_WRAPPER_ID}`.");
            return Err(Error::ReferenceError(msg));
        }
        let app_wrapper = app_wrapper_option.unwrap();
        Ok(app_wrapper)
    }

    fn add_attributes(
        current_component: &Component,
        attributes: &HashMap<String, String>,
        element: &Element,
    ) -> Result<(), Error> {
        for (key, value) in attributes {
            let attribute_value_variant_result = get_attribute_text_variant(value.to_owned());
            if attribute_value_variant_result.is_err() {
                return Err(attribute_value_variant_result.unwrap_err());
            }
            let TextInfo { value, variant } = attribute_value_variant_result.unwrap();
            let attr_value;
            match variant {
                TextVariant::Expression => {
                    let attr_value_result = evaluate_expression(value, current_component);
                    if attr_value_result.is_err() {
                        return Err(attr_value_result.unwrap_err());
                    }
                    attr_value = attr_value_result.unwrap();
                }
                TextVariant::ExpressionAndString => {
                    let attr_value_result =
                        evaluate_expression_and_string(value, current_component);
                    if attr_value_result.is_err() {
                        return Err(attr_value_result.unwrap_err());
                    }
                    attr_value = attr_value_result.unwrap();
                }
                _ => {
                    attr_value = value;
                }
            }

            let set_attribute_result = element.set_attribute(key, &attr_value);
            if set_attribute_result.is_err() {
                return Err(Error::DomError(set_attribute_result.unwrap_err()));
            }
        }
        Ok(())
    }

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
                    construct_result = self::construct_dom(child, &component, &element, &document);
                }
                _ => {
                    construct_result =
                        self::construct_dom(child, current_component, &element, &document);
                }
            }
            if construct_result.is_err() {
                return Err(construct_result.unwrap_err());
            }
        }
        Ok(())
    }

    fn construct_tag(
        current_root: &VirtualNode,
        current_component: &Component,
        parent: &Element,
        document: &Document,
        tag_name: String,
    ) -> Result<(), Error> {
        let new_element_result = document.create_element(&tag_name);
        let new_element_result_is_ok = result_is_ok(&new_element_result);
        if !new_element_result_is_ok {
            return Err(Error::DomError(new_element_result.unwrap_err()));
        }
        let new_element = new_element_result.unwrap();

        let attributes = &current_root.attributes;
        let add_attributes_result = add_attributes(current_component, attributes, &new_element);
        if add_attributes_result.is_err() {
            return Err(add_attributes_result.unwrap_err());
        }

        let append_child_result = parent.append_child(&new_element);
        let append_child_result_is_ok = append_child_result.is_ok();
        if !append_child_result_is_ok {
            return Err(Error::DomError(append_child_result.unwrap_err()));
        }

        let children = &current_root.children;
        return add_children(children, current_component, &new_element, document);
    }

    fn construct_text(text: &String, parent: &Element) -> Result<(), Error> {
        let text_element_result = Text::new_with_data(text);
        let text_element_result_is_ok = text_element_result.is_ok();
        if !text_element_result_is_ok {
            return Err(Error::DomError(text_element_result.unwrap_err()));
        }
        let text_element = text_element_result.unwrap();
        let append_text_result = parent.append_child(&text_element);
        let append_text_result_is_ok = append_text_result.is_ok();
        if !append_text_result_is_ok {
            return Err(Error::DomError(append_text_result.unwrap_err()));
        }
        Ok(())
    }

    fn construct_dom(
        current_root: &VirtualNode,
        current_component: &Component,
        parent: &Element,
        document: &Document,
    ) -> Result<(), Error> {
        let node_type = &current_root.node_type;
        match node_type {
            NodeType::Component(component) => {
                return construct_dom(&component.vdom, component, parent, document);
            }
            NodeType::Tag(tag_name) => {
                return construct_tag(
                    current_root,
                    current_component,
                    parent,
                    document,
                    tag_name.to_owned(),
                );
            }
            NodeType::Text(text) => {
                return construct_text(text, parent);
            }
        }
    }

    pub fn construct_dom_wrapper(root_component: &Component) -> Result<(), Error> {
        let document_result = get_document();
        let document_is_ok = result_is_ok(&document_result);
        if !document_is_ok {
            return Err(document_result.unwrap_err());
        }

        let parent_result = get_app_wrapper();
        let parent_is_ok = result_is_ok(&parent_result);
        if !parent_is_ok {
            return Err(parent_result.unwrap_err());
        }

        let document = document_result.unwrap();
        let parent = parent_result.unwrap();

        let construct_dom_result =
            construct_dom(&root_component.vdom, root_component, &parent, &document);
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
