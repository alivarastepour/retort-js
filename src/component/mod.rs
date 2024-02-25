pub mod component_mod {
    use std::collections::HashMap;

    use crate::presenter::presenter_mod::Presenter;
    use serde::{Deserialize, Serialize};
    use serde_json::{from_str, Map, Value};
    use strfmt::strfmt;
    use wasm_bindgen::prelude::wasm_bindgen;

    #[derive(Serialize, Deserialize)]
    #[wasm_bindgen]
    pub struct Component {
        state: String,
        presenter: Box<Presenter>,
        props: String,
    }

    const NO_VALUE: &str = "undefined";
    const OBJ: &str = "[object]";

    // impl Into<HashMap<String, String>> for serde_json::Map<std::string::String, Value> {
    //     fn into(self) -> HashMap<String, String> {

    //     }
    // }

    // fn into_hashamp(map: &serde_json::Map<std::string::String, Value>) -> HashMap<String, String> {
    //     let res: HashMap<String, String> = HashMap::new();
    //     for (k, v) in map {
    //         match v {
    //             // Value::
    //         }
    //         res.insert(k.to_owned(), v.to_owned());
    //     }
    //     res
    // }

    #[wasm_bindgen]
    impl Component {
        #[wasm_bindgen(constructor)]
        pub fn new(state: String, presenter: Presenter, props: String) -> Component {
            Component {
                state,
                presenter: Box::new(presenter),
                props,
            }
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

        // TODO: consider the reason why getter/ setters should (not)be accessible
        #[wasm_bindgen(getter)]
        pub fn presenter(&self) -> String {
            // let presenter = *self.presenter;

            // to_value(&presenter)

            return "oops".to_owned();
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

        pub fn render(&self) {
            let presenter = &*self.presenter;
            match presenter {
                Presenter::Component(component) => {
                    component.render();
                }
                Presenter::Markup(markup) => {
                    let state = &self.state;
                    let state_object: Map<String, Value> = from_str(&state).unwrap();
                }
                Presenter::Nothing() => {}
            }
        }
    }
}
