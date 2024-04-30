pub mod component_mod {
    use futures::{
        future::{BoxFuture, FutureExt},
        Future,
    };
    use std::{collections::HashMap, ops::Deref, pin::Pin};

    use crate::{
        dom::dom_mod::construct_dom_wrapper,
        error::error_mod::Error as CustomError,
        parser::parser_mod::{call_module_resolver, NodeType, VirtualNode},
    };
    use serde::{Deserialize, Serialize};
    use serde_json::{from_str, to_string, Error, Map, Value};
    use serde_wasm_bindgen::to_value;
    use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
    use web_sys::{
        console::{log_1, time, time_end},
        js_sys::Function,
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

    const NO_VALUE: &str = "undefined";
    const OBJ: &str = "[object]";

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
            // let state: Result<Value, Error> = from_str(&state);
            // if let Result::Err(err) = state {
            //     panic!("could not convert it: {err}");
            // }
            // let virtual_node_result: Result<VirtualNode, CustomError> =
            //     Self::create_component_vdom(presenter.clone()).await;
            // if let Result::Err(err) = virtual_node_result {
            //     panic!("");
            // }
            Component {
                state,
                presenter,
                props: "{}".to_owned(),
                component_did_mount: component_did_mount.clone(),
                vdom: Box::new(VirtualNode {
                    attributes: HashMap::new(),
                    children: Vec::new(),
                    node_type: NodeType::Tag("t".to_owned()),
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
        pub fn state(&self) -> String {
            to_string(&self.state).unwrap()
        }

        // #[wasm_bindgen(setter)]
        // pub fn set_state(&mut self, state: String) {
        //     // TODO: generalize this repeated code
        //     let deserialized_state: Result<Value, Error> = from_str(&state);
        //     if let Result::Err(err) = deserialized_state {
        //         panic!("could not convert it: {err}");
        //     }
        //     self.state = deserialized_state.unwrap();
        // }

        /*
         note the importance of &mut self parameter. although it sounds implicit, removing it will
        forbid you to use it in Javascript.
        */
        // #[wasm_bindgen]
        // pub fn set_state_wrapper(&mut self, state: String) {
        //     let deserialized_state: Result<Value, Error> = from_str(&state);
        //     if let Result::Err(err) = deserialized_state {
        //         panic!("could not convert it: {err}");
        //     }
        //     let new_state = deserialized_state.unwrap();
        //     self.state = new_state;
        // }

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

        fn obj_key_path_to_value(map: Map<String, Value>, path: String) -> String {
            let mut mp = map.clone();

            let mut iterator = path.split('.');
            let temp_iterator = iterator.clone();
            let iterator_vec: &Vec<&str> = &temp_iterator.collect();

            let len = iterator_vec.len();
            let mut current = 1;

            let val: String = loop {
                let val = iterator.next();
                match val {
                    Option::None => {
                        break NO_VALUE.to_string();
                    }
                    Option::Some(v) => {
                        let map_item = mp.get(v);
                        match map_item {
                            Option::None => {
                                break NO_VALUE.to_string();
                            }
                            Option::Some(value_pair) => match value_pair {
                                Value::String(st) => {
                                    if current == len {
                                        break st.to_string();
                                    } else {
                                        break NO_VALUE.to_string();
                                    }
                                }
                                Value::Object(mx) => {
                                    if current == len {
                                        break OBJ.to_string();
                                    } else {
                                        mp = mx.clone();
                                    }
                                }
                                Value::Number(nb) => {
                                    if current == len {
                                        break nb.to_string();
                                    } else {
                                        break NO_VALUE.to_string();
                                    }
                                }
                                Value::Null => {
                                    break NO_VALUE.to_string();
                                }
                                Value::Bool(bl) => {
                                    if current == len {
                                        break bl.to_string();
                                    } else {
                                        break NO_VALUE.to_string();
                                    }
                                }
                                Value::Array(_) => {
                                    break OBJ.to_string();
                                }
                            },
                        }
                    }
                }
                current += 1;
            };
            val
        }

        #[wasm_bindgen]
        pub fn register_event_listener(
            id: &str,
            event_type: &str,
            event_listener: &Function,
        ) -> bool {
            let window = web_sys::window().expect("where window?");
            let document = window.document().expect("where document?");
            let target = document.get_element_by_id(id).expect("where target?");
            let result = target.add_event_listener_with_callback(event_type, event_listener);
            match result {
                Result::Err(_err) => false,
                Result::Ok(_v) => true,
            }
        }

        async fn create_component_vdom(presenter: String) -> Result<VirtualNode, CustomError> {
            let res = parse_presenter(&presenter);
            if let Result::Err(err) = res {
                return Err(err);
            }
            let res = res.unwrap();
            let virtual_node_result = parse_vdom_from_string(&res).await;
            if let Result::Err(err) = virtual_node_result {
                return Err(err);
            }
            let virtual_node = virtual_node_result.unwrap();
            let a = to_value(&virtual_node).unwrap();
            log_1(&a);
            Ok(virtual_node)

            // NOTE: comments are some what deprecated but not removed because they still provide road map

            // read content of above line's file
            // parse the markup, look for component imports
            // add the markup to vdom structure as is, transform modules imported from module_resolver to Component struct
            // after that, add components to the vdom of current component as is, then call this function on the newly created components.
            // after the execution of this function finalizes, vdom structure of all components should be prepared
            // and we are ready to create its unified version.
        }

        /// NOTE: comments are some what deprecated but not removed because they still provide road map

        // 1- mount should be called on a root node
        // 2- add a root field to component struct to determine if it has permission to call mount(optional)
        // 3- mount is called with a reference to the root component
        // 4- it calls the TBD function in JS that creates VDOM for components.
        // 5- the JS function should look for path field in component, which represents where the actual markup resides.
        // 6- the JS function should parse that file to create VDOM.
        // 7- the JS function should look for imports should it reach a component call
        // 8- for know, the info of the component is stored in VDOM, not what it returns. we are not making a unified VDOM at this stage.
        // 9- after all markup is parsed and VDOM for each component is created, it's about time to make VDOM objects to DOM nodes.
        // 10- at this point, we should again come back to the mount function, and start from the component which mount was called on.
        // 11- this time we are trying to make a unified VDOM first, so we will start from root, and add only markup to the VDOM.
        // 12- this means that we will ask components to return what their child components actually return(with state and stuff).
        // 13- this process created a unified VDOM.
        // 14- from here we can start creating the actual DOM.
        // 15- I suspect if need to keep this initially created unified VDOM, since we will only work with component updates from here on, and we have VDOM of each component
        fn create_vdom(
            component: &mut Component,
        ) -> Pin<Box<dyn Future<Output = Result<(), CustomError>> + '_>> {
            Box::pin(async {
                let presenter = &component.presenter;
                let res = parse_presenter(presenter);
                if let Result::Err(err) = res {
                    return Err(err);
                }
                let res = res.unwrap();
                let x = parse_vdom_from_string(&res).await;

                if let Result::Err(err) = x {
                    return Err(err);
                }
                let virtual_node = x.unwrap();
                component.set_vdom(&virtual_node);

                let imports = &res.imports;
                for (_, path) in imports {
                    let component_result = call_module_resolver(path).await;
                    if component_result.is_err() {
                        return Err(component_result.unwrap_err());
                    }
                    let mut component = component_result.unwrap();
                    let result = Self::create_vdom(&mut component).await;
                    if result.is_err() {
                        return Err(result.unwrap_err());
                    }
                }
                Ok(())
            })

            // read content of above line's file
            // parse the markup, look for component imports
            // add the markup to vdom structure as is, transform modules imported from module_resolver to Component struct
            // after that, add components to the vdom of current component as is, then call this function on the newly created components.
            // after the execution of this function finalizes, vdom structure of all components should be prepared
            // and we are ready to create its unified version.

            // this function is called with the root node.
            // in the root node's object, we have access to all components' vdom structure.
            // the vdom structure currently contains tag nodes, text nodes and component nodes.
            // we can't render a component node; that is meaningless. so in order to create a representation
            // that is convertible to DOM, we need to do the following:
            // 1- access the vdom of root node.
            // 2- traverse through its vdom field.
            // 3- Text and Tag variants of `node_type` should be left as is(do what with them?).
            // 4- Component variants though, must be replaced with what they "return".
            // 5-
        }
        pub async fn mount(&mut self) {
            time();
            let vdom_creation_result = Self::create_vdom(self).await;
            if vdom_creation_result.is_err() {
                log_1(&JsValue::from_str("!oops!"));
                // return Err(vdom_creation_result.unwrap_err());
            }

            let res = construct_dom_wrapper(&self);
            if res.is_err() {
                log_1(&JsValue::from_str("!!oops!!"));
                // return Err(vdom_creation_result.unwrap_err());
            }
            // self.component_did_mount.call0(&JsValue::undefined());
            // self.call_component_did_mount();
            time_end();
        }
    }
}
