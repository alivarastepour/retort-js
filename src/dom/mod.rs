pub mod dom_mod {
    use std::collections::HashMap;

    use serde_wasm_bindgen::to_value;
    use wasm_bindgen::JsValue;
    use web_sys::{console::log_1, window, Document, Element, Text, Window};

    use crate::{
        component::component_mod::{effects_runner, Component, Effects},
        const_util::const_util_mod::{
            is_input_true_literal, APP_WRAPPER_ID, RENDER_ELSE_ATTRIBUTE_NAME,
            RENDER_ELSE_IF_ATTRIBUTE_NAME, RENDER_IF_ATTRIBUTE_NAME,
        },
        error::error_mod::Error,
        parser::parser_mod::{NodeType, VirtualNode},
    };

    // future reference: the logic for determining whether a node should render or not has been repeated
    // 3 times in this module. IDK how to generalize it.

    /// Represents the valid states for `render-*` attributes in each scope. `NotReached` indicates
    /// that no `render-if` attribute has been reached yet, or an `if-else` expression has already
    /// finished(thus it has no effect on the next `if-else` expressions). `True` indicates that one
    /// of the earlier expressions have evaluated to `true`, thus other `else-if`s should not be added.
    /// `False` indicates that there have been some clauses, but non of them have evaluated to true thus
    /// far.
    #[derive(Debug)]
    enum IfExprState {
        NotReached,
        True,
        False,
    }

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
        let app_wrapper_option = document.get_element_by_id(APP_WRAPPER_ID);
        if app_wrapper_option.is_none() {
            return Err(Error::ReferenceError(format!("Could not find the root element. Make sure you have a root element with id of `{APP_WRAPPER_ID}`.")));
        }
        let app_wrapper = app_wrapper_option.unwrap();
        Ok(app_wrapper)
    }

    /// Adds attributes to the provided element. Note that attribute values are evaluated. Returns an
    /// `Err` if an error occurs during evaluation or setting attributes.
    fn add_attributes(
        attributes: &HashMap<String, String>,
        element: &Element,
    ) -> Result<(), Error> {
        for (key, value) in attributes {
            let set_attribute_result = element.set_attribute(key, &value);
            if set_attribute_result.is_err() {
                return Err(Error::DomError(set_attribute_result.unwrap_err()));
            }
        }
        Ok(())
    }

    /// Recursively calls the `self::construct_dom` function on each child of the current virtual node;
    /// This is when a child doesn't have a `render-*` special attribute. If it has one of those attributes,
    /// It will be added to the DOM only if its evaluated result is true according to how `if-else` expressions
    /// are evaluated.
    /// Returns an `Err` variant if an `Err` variant is returned from any of the calls to `self::construct_dom`.
    fn add_children(
        children: &Vec<VirtualNode>,
        current_component: &mut Component,
        element: &Element,
        document: &Document,
    ) -> Result<(), Error> {
        let mut if_state_expr: IfExprState = IfExprState::NotReached;
        for child in children {
            let render_node_result: Result<(bool, IfExprState), Error> =
                should_node_render(child.clone(), if_state_expr);
            if render_node_result.is_err() {
                return Err(render_node_result.unwrap_err());
            }
            let (should_add, new_if_state_expr) = render_node_result.unwrap();
            if_state_expr = new_if_state_expr;
            if !should_add {
                continue;
            }
            let node_type = child.node_type.clone();
            let construct_result;
            match node_type {
                NodeType::Component(mut component) => {
                    construct_result =
                        self::construct_dom(child.clone(), &mut component, &element, &document);
                }
                _ => {
                    construct_result =
                        self::construct_dom(child.clone(), current_component, &element, &document);
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
        current_root: VirtualNode,
        current_component: &mut Component,
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
        let add_attributes_result = add_attributes(attributes, &new_element);
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
    fn construct_text(text: &String, parent: &Element) -> Result<(), Error> {
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

    /// Given a node, the context of the component which it was used in and the previous state of
    /// `render-*`, determines whether a node should be added to the DOM or not. If no error happens,
    /// it returns an `Ok` variant which contains a tuple indicating if the node should be rendered, and
    /// the next state of `render-*` in the scope.
    fn should_node_render(
        current_root: VirtualNode,
        if_state_expr: IfExprState,
    ) -> Result<(bool, IfExprState), Error> {
        let attrs = &current_root.attributes;

        let if_ = attrs.get(RENDER_IF_ATTRIBUTE_NAME);
        let else_if = attrs.get(RENDER_ELSE_IF_ATTRIBUTE_NAME);
        let else_ = attrs.get(RENDER_ELSE_ATTRIBUTE_NAME);

        if if_.is_some() {
            let evaluated_if_value = if_.unwrap();
            let res = is_input_true_literal(&evaluated_if_value);
            let new_state;
            if res {
                new_state = IfExprState::True;
            } else {
                new_state = IfExprState::False;
            }
            return Ok((res, new_state));
        } else if else_if.is_some() {
            match if_state_expr {
                IfExprState::False => {}
                IfExprState::True => {
                    return Ok((false, if_state_expr));
                }
                IfExprState::NotReached => {
                    return Err(Error::ParsingError(format!(
                        "Didn't expect a `{RENDER_ELSE_IF_ATTRIBUTE_NAME}` attribute."
                    )));
                }
            }
            let evaluated_else_if_value = else_if.unwrap();
            let res = is_input_true_literal(&evaluated_else_if_value);
            let new_state;
            if res {
                new_state = IfExprState::True;
            } else {
                new_state = IfExprState::False;
            }
            return Ok((res, new_state));
        } else if else_.is_some() {
            match if_state_expr {
                IfExprState::False => return Ok((true, IfExprState::NotReached)),
                IfExprState::True => return Ok((false, IfExprState::NotReached)),
                IfExprState::NotReached => {
                    return Err(Error::ParsingError(format!(
                        "Didn't expect a `{RENDER_ELSE_ATTRIBUTE_NAME}` attribute."
                    )))
                }
            }
        }
        return Ok((true, if_state_expr));
    }

    fn run_mount_effects(component: &mut Component) -> Result<(), Error> {
        let prev_state = &component.state_parsed();
        let prev_props = &component.props_parsed();
        let did_mount_res = effects_runner(
            Effects::ComponentDidMount,
            component,
            prev_state,
            prev_props,
        );
        let update_res = effects_runner(
            Effects::ComponentDidUpdate,
            component,
            prev_state,
            prev_props,
        );
        match (did_mount_res, update_res) {
            (Err(e1), Err(_)) | (Err(e1), Ok(_)) => {
                return Err(e1);
            }
            (Ok(()), Err(e2)) => {
                return Err(e2);
            }
            (Ok(()), Ok(())) => return Ok(()),
        }
    }

    /// Constructs DOM using the provided virtual node and component as a root. Returns `Ok` variant
    /// if no errors are encountered while building DOM; an `Err` variant otherwise, explaining what
    /// went wrong.
    fn construct_dom(
        current_root: VirtualNode,
        current_component: &mut Component,
        parent: &Element,
        document: &Document,
    ) -> Result<(), Error> {
        let node_type = current_root.node_type.clone();
        let res;
        match node_type {
            NodeType::Component(mut component) => {
                let render_node_result: Result<(bool, IfExprState), Error> =
                    should_node_render(*component.get_vdom().clone(), IfExprState::NotReached);
                if render_node_result.is_err() {
                    return Err(render_node_result.unwrap_err());
                }
                let should_add = render_node_result.unwrap().0;
                if !should_add {
                    return Ok(());
                }
                res = construct_dom(
                    *component.get_vdom().clone(),
                    &mut component,
                    parent,
                    document,
                );
                let initial_effect_call_result = run_mount_effects(&mut component);
                if initial_effect_call_result.is_err() {
                    return Err(initial_effect_call_result.unwrap_err());
                }
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
                res = construct_text(&text, parent);
            }
        }
        return res;
    }

    /// Encapsulates the logic of preparing arguments for `self::construct_dom` function
    pub fn construct_dom_wrapper(root_component: &mut Component) -> Result<(), Error> {
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

        let render_node_result: Result<(bool, IfExprState), Error> =
            should_node_render(*root_component.get_vdom().clone(), IfExprState::NotReached);
        if render_node_result.is_err() {
            return Err(render_node_result.unwrap_err());
        }
        let should_add = render_node_result.unwrap().0;
        if !should_add {
            return Ok(());
        }

        let construct_dom_result = construct_dom(
            *root_component.get_vdom().clone(),
            root_component,
            &parent,
            &document,
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
                Error::ReferenceError(err) => {
                    let a = to_value(&err).unwrap();
                    log_1(&a);
                }
                Error::SerdeWasmBindgenError(err) => {
                    let a = err.to_string();
                    let a = to_value(&a).unwrap();
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

// rendering list of data:
// render-for is used as a special attribute to mark the "wrapper" of rendered list.
// render-for syntax is sth like this: render-for={"varName of state.someList"}
// this way, `varName` becomes a value which is available in the entire subtree of the "wrapper".
