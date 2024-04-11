pub mod dom_mod {
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

    fn construct_dom(current_root: &VirtualNode, parent: &Element, document: &Document) {
        let node_type = &current_root.node_type;
        match node_type {
            NodeType::Component(component) => {
                self::construct_dom(&component.vdom, parent, document);
            }
            NodeType::Tag(tag_name) => {
                let new_element_result = document.create_element(&tag_name);
                if let Result::Err(err) = new_element_result {
                    panic!("")
                }
                let new_element = new_element_result.unwrap();
                let attributes = &current_root.attributes;
                for (key, value) in attributes {
                    // TEMP SOLUTION
                    let value = value.replace("{", "").replace("}", "");
                    let value = &value[1..value.len() - 1];
                    let res = new_element.set_attribute(key, &value);
                }
                let res = parent.append_child(&new_element);
                let children = &current_root.children;
                for child in children {
                    self::construct_dom(child, &new_element, &document);
                }
            }
            NodeType::Text(text) => {
                let text_element = Text::new_with_data(text);
                if let Result::Err(err) = text_element {
                    panic!("")
                }
                let t = text_element.unwrap();
                parent.append_child(&t);
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

        construct_dom(current_root, &parent, &document);

        Ok(())
    }
}
