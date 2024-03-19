pub mod component_mod {

    use crate::presenter::presenter_mod::Presenter;
    use serde::{Deserialize, Serialize};
    use serde_json::{Map, Value};
    use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
    use web_sys::{console::log_1, js_sys::Function};

    #[derive(Serialize, Deserialize)]
    #[wasm_bindgen]
    pub struct Component {
        state: String,
        presenter: Box<Presenter>,
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
            presenter: Presenter,
            props: String,
            component_did_mount: &Function,
        ) -> Component {
            Component {
                state,
                presenter: Box::new(presenter),
                props,
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
            self.state.clone()
        }

        #[wasm_bindgen(setter)]
        pub fn set_state(&mut self, state: String) {
            self.state = state;
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
        pub fn presenter(&self) -> Presenter {
            *self.presenter.clone()
        }

        #[wasm_bindgen(setter)]
        pub fn set_presenter(&mut self, presenter: Presenter) {
            self.presenter = Box::new(presenter);
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

        pub fn render(&self) {
            let window = web_sys::window().expect("where window?");
            let document = window.document().expect("where document?");
            let root = document.get_element_by_id("root").expect("where root?");

            let presenter = &*self.presenter;
            match presenter {
                Presenter::Component(component) => {
                    component.render();
                }
                Presenter::Markup(markup) => {
                    let m = format!("{markup}");
                    root.set_inner_html(&m);
                }
                Presenter::Nothing(_) => {
                    root.set_inner_html("<div>nothing</div>");
                }
            }
            let component_js_value =
                serde_wasm_bindgen::to_value(self).unwrap_or(JsValue::undefined());
            let res = self.component_did_mount.call0(&component_js_value);
            match res {
                Result::Err(err) => {
                    log_1(&err);
                }
                Result::Ok(_a) => {}
            }
        }
    }
}
