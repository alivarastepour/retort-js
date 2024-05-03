pub mod component_mod {
    use std::{collections::HashMap, ops::Deref};

    use crate::{
        dom::dom_mod::construct_dom_wrapper,
        error::error_mod::Error as CustomError,
        parser::parser_mod::{NodeType, VirtualNode},
    };
    use serde::{Deserialize, Serialize};
    use serde_wasm_bindgen::to_value;
    use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
    use web_sys::{
        console::log_1,
        js_sys::{Function, JSON},
    };

    use crate::{
        parser::parser_mod::parse_vdom_from_string, presenter::presenter_mod::parse_presenter,
    };

    #[derive(Serialize, Deserialize, Debug)]
    #[wasm_bindgen]
    pub struct Component {
        state: String,
        presenter: String,
        props: String,
        #[serde(with = "serde_wasm_bindgen::preserve")]
        component_did_mount: Function,
        vdom: Box<VirtualNode>,
    }

    impl Clone for Component {
        fn clone(&self) -> Self {
            Component {
                presenter: self.presenter.clone(),
                props: self.props.to_owned(),
                state: self.state.to_owned(),
                component_did_mount: self.component_did_mount.clone(),
                vdom: Box::from(self.vdom.deref().to_owned()),
            }
        }
    }

    // TODO: refactor as much clone() call you can with lifetime parameters.

    impl Component {
        pub fn get_vdom<'a>(&'a self) -> &'a Box<VirtualNode> {
            return &self.vdom;
        }

        pub fn get_state<'a>(&'a self) -> &'a String {
            return &self.state;
        }

        pub fn get_props<'a>(&'a self) -> &'a String {
            return &self.props;
        }

        pub fn set_vdom(&mut self, v_node: &VirtualNode) {
            self.vdom = Box::new(v_node.clone());
        }
    }

    #[wasm_bindgen]
    impl Component {
        #[wasm_bindgen(constructor)]
        pub fn new(state: String, presenter: String, component_did_mount: &Function) -> Component {
            Component {
                state,
                presenter,
                props: "{}".to_owned(),
                component_did_mount: component_did_mount.clone(),
                vdom: Box::new(VirtualNode {
                    attributes: HashMap::new(),
                    children: Vec::new(),
                    node_type: NodeType::Tag(" ".to_owned()),
                }),
            }
        }

        #[wasm_bindgen(getter)]
        pub fn component_did_mount(&self) -> Function {
            self.component_did_mount.clone()
        }

        #[wasm_bindgen]
        pub fn call_component_did_mount(&self) -> Result<JsValue, JsValue> {
            let res = self.component_did_mount.call0(&JsValue::null());
            return res;
        }

        #[wasm_bindgen(getter)]
        pub fn vdom(&self) -> JsValue {
            to_value(&self.vdom).unwrap()
        }

        #[wasm_bindgen(setter)]
        pub fn set_component_did_mount(&mut self, component_did_mount: &Function) {
            self.component_did_mount = component_did_mount.clone();
        }

        #[wasm_bindgen(getter)]
        pub fn state_parsed(&self) -> JsValue {
            // TODO:: observe usages of state property and their types, remove extra functionalities
            JSON::parse(&self.state).unwrap_or(JsValue::null())
        }

        #[wasm_bindgen(getter)]
        pub fn state(&self) -> String {
            self.state.clone()
        }

        #[wasm_bindgen(getter)]
        pub fn props(&self) -> String {
            self.props.clone()
        }

        #[wasm_bindgen(setter)]
        pub fn set_props(&mut self, props: String) {
            self.props = props;
        }

        #[wasm_bindgen(getter)]
        pub fn presenter(&self) -> String {
            self.presenter.clone()
        }

        #[wasm_bindgen(setter)]
        pub fn set_presenter(&mut self, presenter: String) {
            self.presenter = presenter;
        }

        #[wasm_bindgen]
        /// Updates the `state` of a component which this function is called with, using a callback function.
        /// provided callback is called with component's current `state` as an argument, allowing user to
        /// return the component's next `state` accordingly.
        pub fn set_state(&mut self, callback: Function) {
            let state_js_value = self.state_parsed();
            let new_state_result = callback.call1(&JsValue::undefined(), &state_js_value);
            if new_state_result.is_err() {
                let msg_js_value = new_state_result.as_ref().unwrap_err();
                let msg = JsValue::as_string(&msg_js_value).unwrap_or(String::from(
                    "Error occurred while setting the state of component.",
                ));
            }
            let new_state = new_state_result.unwrap();
            let new_state_string = JSON::stringify(&new_state).unwrap();
            self.state = new_state_string.into();
        }

        /// Given a component object, parses its presenter using the `parse_presenter` function and then
        /// constructs a `VirtualNode` from its result, which corresponds to the current component's
        /// markup structure. An `Ok` variant is returned if nothing goes wrong, `Err` variant otherwise,
        /// explaining what went wrong.
        async fn create_vdom(component: &mut Component) -> Result<(), CustomError> {
            let presenter = &component.presenter;
            let parsed_presenter_result = parse_presenter(presenter);
            if let Result::Err(err) = parsed_presenter_result {
                return Err(err);
            }
            let parsed_presenter = parsed_presenter_result.unwrap();

            let vdom_result = parse_vdom_from_string(&parsed_presenter).await;

            if let Result::Err(err) = vdom_result {
                return Err(err);
            }
            let virtual_node = vdom_result.unwrap();
            component.set_vdom(&virtual_node);

            Ok(())
        }

        #[wasm_bindgen]
        /// An async wrapper for calling the `Self::create_vdom(self)`.
        pub async fn render(&mut self) -> Component {
            let vdom_creation_result = Self::create_vdom(self).await;
            if vdom_creation_result.is_err() {
                let err = vdom_creation_result.unwrap_err();
                match err {
                    CustomError::TypeError(e) => {
                        log_1(&JsValue::from_str(&e));
                    }
                    CustomError::SerdeWasmBindgenError(e) => {
                        let a = e.to_string();
                        log_1(&JsValue::from_str(&a));
                    }
                    CustomError::ResolveError(e) => {
                        log_1(&JsValue::from_str(&e));
                    }
                    _ => {
                        log_1(&JsValue::from_str("others"));
                    }
                }
            }
            return self.clone();
        }

        #[wasm_bindgen]
        /// constructs the DOM from a given entry point. This should be called from the component
        /// that wraps the entire component tree, otherwise a subtree of components will be added to DOM.
        pub fn mount(&mut self) {
            let res = construct_dom_wrapper(&self);
            if res.is_err() {
                panic!("")
            }
        }
    }
}
