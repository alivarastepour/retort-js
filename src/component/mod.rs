mod effects;

pub mod component_mod {

    use std::{collections::HashMap, ops::Deref};

    use crate::{
        dom::dom_mod::construct_dom_wrapper,
        error::error_mod::{error_handler, Error},
        parser::parser_mod::{NodeType, VirtualNode},
    };
    use serde::{Deserialize, Serialize};
    use serde_wasm_bindgen::{from_value, to_value};
    use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
    use web_sys::{
        console::{time, time_end, time_end_with_label, time_with_label},
        js_sys::{Array, Function, JSON},
    };

    use crate::{
        parser::parser_mod::parse_vdom_from_string, presenter::presenter_mod::parse_presenter,
    };

    pub use super::effects::effects_mod::{effects_runner, Effects};

    #[derive(Serialize, Deserialize, Debug)]
    #[wasm_bindgen]
    pub struct Component {
        state: String,
        presenter: String,
        props: String,
        vdom: Box<VirtualNode>,
        #[serde(with = "serde_wasm_bindgen::preserve")]
        effects: Array,
        #[serde(with = "serde_wasm_bindgen::preserve")]
        component_did_mount: Array,
        #[serde(with = "serde_wasm_bindgen::preserve")]
        component_will_unmount: Array,
    }

    impl Clone for Component {
        fn clone(&self) -> Self {
            Component {
                presenter: self.presenter.clone(),
                props: self.props.to_owned(),
                state: self.state.to_owned(),
                component_did_mount: self.component_did_mount.clone(),
                component_will_unmount: self.component_will_unmount.clone(),
                vdom: Box::from(self.vdom.deref().to_owned()),
                effects: self.effects.clone(),
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

        pub fn get_component_did_mount<'a>(&'a self) -> &'a Array {
            return &self.component_did_mount;
        }

        pub fn get_effects<'a>(&'a self) -> &'a Array {
            return &self.effects;
        }

        pub fn set_vdom(&mut self, v_node: &VirtualNode) {
            self.vdom = Box::new(v_node.clone());
        }

        pub fn effect_arr_into_vec(&self) -> Vec<JsValue> {
            let arr = &self.effects;
            let iter_arr = &arr.clone().into_iter();
            iter_arr.clone().collect()
        }

        pub fn set_state_with_value(&mut self, new_state: JsValue) -> Result<(), Error> {
            let stringified_state = JSON::stringify(&new_state);
            if stringified_state.is_err() {
                let error = stringified_state.unwrap_err();
                let msg: String = from_value(error).unwrap_or(String::from(
                    "Invalid object format caused JSON::stringify to fail.",
                ));
                return Err(Error::TypeError(msg));
            }

            let new_state_string_result = stringified_state.unwrap().as_string();
            if new_state_string_result.is_none() {
                return Err(Error::TypeError(String::from(
                    "Failed to parse JsString into String.",
                )));
            }
            let new_state = new_state_string_result.unwrap();
            self.state = new_state;
            Ok(())
        }
    }

    #[wasm_bindgen]
    impl Component {
        #[wasm_bindgen(constructor)]
        pub fn new(state: String, presenter: String) -> Component {
            let empty_vdom = Box::new(VirtualNode {
                // no need to have a valid vdom at this point
                attributes: HashMap::new(),
                children: Vec::new(),
                node_type: NodeType::Tag(" ".to_owned()),
            });
            Component {
                state,
                presenter,
                props: "{}".to_owned(),
                vdom: empty_vdom,
                effects: Array::new(),
                component_will_unmount: Array::new(),
                component_did_mount: Array::new(),
            }
        }

        #[wasm_bindgen(getter)]
        pub fn component_did_mount(&self) -> Array {
            self.component_did_mount.clone()
        }

        #[wasm_bindgen(getter)]
        pub fn component_will_unmount(&self) -> Array {
            self.component_will_unmount.clone()
        }

        #[wasm_bindgen(getter)]
        pub fn vdom(&self) -> JsValue {
            to_value(&self.vdom).unwrap()
        }

        #[wasm_bindgen(getter)]
        pub fn effects(&self) -> Array {
            self.effects.clone()
        }

        #[wasm_bindgen(setter)]
        pub fn set_effects(&mut self, effects: Array) {
            self.effects = effects;
        }

        #[wasm_bindgen(setter)]
        pub fn set_component_will_unmount(&mut self, callbacks: Array) {
            self.component_will_unmount = callbacks;
        }

        #[wasm_bindgen(setter)]
        pub fn set_component_did_mount(&mut self, callbacks: Array) {
            self.component_did_mount = callbacks;
        }

        #[wasm_bindgen(getter)]
        pub fn state_parsed(&self) -> JsValue {
            //TODO: consider writing a macro for this functionality
            // TODO:: observe usages of state property and their types, remove extra functionalities
            JSON::parse(&self.state).unwrap_or(JsValue::null())
        }

        #[wasm_bindgen(getter)]
        pub fn props_parsed(&self) -> JsValue {
            JSON::parse(&self.props).unwrap_or(JsValue::null())
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
        /// adds the provided callback to component's effects list.
        pub fn register_effect(&mut self, callback: Function) {
            let prev = &self.effects;
            let new = vec![callback];
            let new_array: Array = new.into_iter().collect();
            self.set_effects(prev.concat(&new_array));
        }

        #[wasm_bindgen]
        /// adds the provided callback to component's component_did_mount effects list.
        pub fn register_component_did_mount(&mut self, callback: Function) {
            let prev = &self.component_did_mount;
            let new = vec![callback];
            let new_array: Array = new.into_iter().collect();
            self.set_component_did_mount(prev.concat(&new_array));
        }

        #[wasm_bindgen]
        /// adds the provided callback to component's component_will_unmount effects list.
        pub fn register_component_will_unmount(&mut self, callback: Function) {
            let prev = &self.component_will_unmount;
            let new = vec![callback];
            let new_array: Array = new.into_iter().collect();
            self.set_component_will_unmount(prev.concat(&new_array));
        }

        /// Implementation details of setting state of a component.
        fn set_state_inner(
            &mut self,
            prev_state: &JsValue,
            callback: Function,
        ) -> Result<(), Error> {
            let new_state_result = callback.call1(&JsValue::undefined(), &prev_state);
            if new_state_result.is_err() {
                let msg_js_value = new_state_result.as_ref().unwrap_err();
                let msg = JsValue::as_string(&msg_js_value).unwrap_or(String::from(
                    "Error occurred while setting the state of component.",
                ));
                let error = Error::EvaluationError(msg);
                return Err(error);
            }
            let new_state = new_state_result.unwrap();
            let set_state_result = self.set_state_with_value(new_state);
            if set_state_result.is_err() {
                return Err(set_state_result.unwrap_err());
            }
            Ok(())
        }

        #[wasm_bindgen]
        /// Updates the `state` of a component which this function is called with, using a callback function.
        /// provided callback is called with component's current `state` as an argument, allowing user to
        /// return the component's next `state` accordingly.
        /// NOTE that its logic is partially incomplete. after any state update, a repaint must be done.
        pub fn set_state(&mut self, callback: Function) {
            let prev_state = self.state_parsed();
            let new_state_result = self.set_state_inner(&prev_state, callback);
            if new_state_result.is_err() {
                error_handler(new_state_result.unwrap_err());
            }
            // diffing algorithm, DOM update, VDOM update and all other shenanigan here.
            time();
            let result = effects_runner(
                Effects::ComponentDidUpdate,
                self,
                &prev_state,
                &self.props_parsed(),
            );
            // let result = self.run_effects(&prev_state, &self.props_parsed());
            time_end();
            if result.is_err() {
                error_handler(result.unwrap_err());
            }
        }

        /// Given a component object, parses its presenter using the `parse_presenter` function and then
        /// constructs a `VirtualNode` from its result, which corresponds to the current component's
        /// markup structure. An `Ok` variant is returned if nothing goes wrong, `Err` variant otherwise,
        /// explaining what went wrong.
        async fn create_vdom(component: &mut Component) -> Result<(), Error> {
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
                error_handler(err);
            }
            return self.clone();
        }

        #[wasm_bindgen]
        /// constructs the DOM from a given entry point. This should be called from the component
        /// that wraps the entire component tree, otherwise a subtree of components will be added to DOM.
        pub fn mount(&mut self) {
            time_with_label("total render time:");

            // currently, root component is not being recognized as a `Component` object in vdom; this
            // introduces some problems to overall logic. So as a workaround, we change its `node_type` type manually
            // to `Component`. HOWEVER, i'm not sure this is the best solution.

            self.set_vdom(&VirtualNode {
                attributes: HashMap::new(), // root component should have no props.
                children: Vec::new(),       // children for component in inheritably not supported
                node_type: NodeType::Component(self.clone()), // change node type of root from Tag to Component
            });

            let res = construct_dom_wrapper(self);
            time_end_with_label("total render time:");
            if res.is_err() {
                error_handler(res.unwrap_err());
            }
        }
    }
}
