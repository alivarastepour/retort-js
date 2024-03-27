pub mod component_mod {
    use serde::{Deserialize, Serialize};
    use serde_json::{from_str, to_string, to_value, Error, Map, Value};
    use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
    use web_sys::{console::log_1, js_sys::Function};

    use crate::file_util::file_util_mod::read_file;

    #[derive(Serialize, Deserialize)]
    #[wasm_bindgen]
    pub struct Component {
        state: Value,
        // presenter: Box<Presenter>,
        presenter: String,
        props: String,
        #[serde(with = "serde_wasm_bindgen::preserve")]
        component_did_mount: Function,
    }

    const NO_VALUE: &str = "undefined";
    const OBJ: &str = "[object]";

    impl Clone for Component {
        fn clone(&self) -> Self {
            Component {
                presenter: self.presenter.clone(),
                props: self.props.to_owned(),
                state: self.state.to_owned(),
                component_did_mount: self.component_did_mount.to_owned(),
            }
        }
    }

    #[wasm_bindgen]
    impl Component {
        #[wasm_bindgen(constructor)]
        pub fn new(
            state: String,
            presenter: String,
            // props: String,
            component_did_mount: &Function,
        ) -> Component {
            let state: Result<Value, Error> = from_str(&state);
            if let Result::Err(err) = state {
                panic!("could not convert it: {err}");
            }
            // log_1(&JsValue::from_str("invoked cons"));
            // component_did_mount.call0(&JsValue::null());
            Component {
                state: state.unwrap(),
                presenter,
                props: "{}".to_owned(),
                component_did_mount: component_did_mount.clone(),
            }
        }

        #[wasm_bindgen(getter)]
        pub fn component_did_mount(&self) -> Function {
            self.component_did_mount.clone()
        }

        #[wasm_bindgen(setter)]
        pub fn set_component_did_mount(&mut self, component_did_mount: &Function) {
            self.component_did_mount = component_did_mount.clone();
        }

        #[wasm_bindgen(getter)]
        pub fn state(&self) -> String {
            // log_1(&JsValue::from_str("invoked getter"));
            to_string(&self.state).unwrap()
        }

        #[wasm_bindgen(setter)]
        pub fn set_state(&mut self, state: String) {
            // TODO: generalize this repeated code
            let deserialized_state: Result<Value, Error> = from_str(&state);
            if let Result::Err(err) = deserialized_state {
                panic!("could not convert it: {err}");
            }
            self.state = deserialized_state.unwrap();
        }

        /*
         note the importance of &mut self parameter. although it sounds implicit, removing it will
        forbid you to use it in Javascript.
        */
        #[wasm_bindgen]
        pub fn set_state_wrapper(&mut self, state: String) {
            log_1(&JsValue::from_str(&state));
            let deserialized_state: Result<Value, Error> = from_str(&state);
            if let Result::Err(err) = deserialized_state {
                panic!("could not convert it: {err}");
            }
            let new_state = deserialized_state.unwrap();
            self.state = new_state;
            // self.render();
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

        // pub fn nest_render(&self) -> String {
        //     let pr: &Presenter = self.presenter.deref();
        //     match pr {
        //         Presenter::Component(_comp) => return "waa".to_owned(),
        //         Presenter::Markup(markup) => {
        //             return markup.to_owned();
        //         }
        //         Presenter::Nothing(_n) => {
        //             return "n".to_owned();
        //         }
        //     }
        // }

        pub fn new_render(&self) -> () {
            // return to_value(self).expect("could not convert to to value");
            // return self.clone();
            // Component {
            //     // component_did_mount: self.component_did_mount
            // }
            // .expect("could not convert to string");
        }

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

        fn create_vdom(component: &mut Component, module_resolver: &Function) {
            let component_presenter_path = &component.presenter;
            let res = read_file(component_presenter_path);
            // let a= JsValue::from(res);
            // log_1(to_value(res));
            // read content of above line's file
            // parse the markup, look for component imports
            // add the markup to vdom structure as is, transform modules imported from module_resolver to Component struct
            // after that, add components to the vdom of current component as is, then call this function on the newly created components.
            // after the execution of this function finalizes, vdom structure of all components should be prepared
            // and we are ready to create its unified version.

            // let component = module_resolver.call1(
            //     &JsValue::null(),
            //     &JsValue::from_str(component_presenter_path),
            // );
        }

        pub fn mount(&mut self, module_resolver: &Function) {
            Self::create_vdom(self, module_resolver);
        }

        // pub fn render(&self) {
        //     let window = web_sys::window().expect("where window?");
        //     let document = window.document().expect("where document?");
        //     let root = document.get_element_by_id("root").expect("where root?");

        //     let presenter = &*self.presenter;
        //     match presenter {
        //         Presenter::Component(component) => {
        //             component.render();
        //         }
        //         Presenter::Markup(markup) => {
        //             let m = format!("{markup}");
        //             root.set_inner_html(&m);
        //         }
        //         Presenter::Nothing(_) => {
        //             root.set_inner_html("<div>nothing</div>");
        //         }
        //     }
        //     let component_js_value =
        //         serde_wasm_bindgen::to_value(self).unwrap_or(JsValue::undefined());
        //     let res = self.component_did_mount.call0(&component_js_value);
        //     match res {
        //         Result::Err(err) => {
        //             log_1(&err);
        //         }
        //         Result::Ok(_a) => {}
        //     }
        // }
    }
}
