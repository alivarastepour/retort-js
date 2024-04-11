pub mod dom_mod {
    use std::collections::HashMap;

    use web_sys::{window, Document, Element, Text, Window};

    use crate::{
        error::error_mod::Error,
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
        attributes: &HashMap<String, String>,
        element: &Element,
    ) -> Result<(), Error> {
        for (key, value) in attributes {
            let set_attribute_result = element.set_attribute(key, value);
            if set_attribute_result.is_err() {
                return Err(Error::DomError(set_attribute_result.unwrap_err()));
            }
        }
        Ok(())
    }

    fn add_children(
        children: &Vec<VirtualNode>,
        element: &Element,
        document: &Document,
    ) -> Result<(), Error> {
        for child in children {
            let construct_result = self::construct_dom(child, &element, &document);
            if construct_result.is_err() {
                return Err(construct_result.unwrap_err());
            }
        }
        Ok(())
    }

    fn construct_tag(
        current_root: &VirtualNode,
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
        let add_attributes_result = add_attributes(attributes, &new_element);
        if add_attributes_result.is_err() {
            return Err(add_attributes_result.unwrap_err());
        }

        let append_child_result = parent.append_child(&new_element);
        let append_child_result_is_ok = append_child_result.is_ok();
        if !append_child_result_is_ok {
            return Err(Error::DomError(append_child_result.unwrap_err()));
        }

        let children = &current_root.children;
        return add_children(children, &new_element, document);
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
        parent: &Element,
        document: &Document,
    ) -> Result<(), Error> {
        let node_type = &current_root.node_type;
        match node_type {
            NodeType::Component(component) => {
                return construct_dom(&component.vdom, parent, document);
            }
            NodeType::Tag(tag_name) => {
                return construct_tag(current_root, parent, document, tag_name.to_owned());
            }
            NodeType::Text(text) => {
                return construct_text(text, parent);
            }
        }
    }

    pub fn construct_dom_wrapper(current_root: &VirtualNode) -> Result<(), Error> {
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

        let construct_dom_result = construct_dom(current_root, &parent, &document);
        Ok(())
    }
}
