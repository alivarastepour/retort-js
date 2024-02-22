pub mod component_mod {
    use std::collections::HashMap;

    use crate::presenter::presenter_mod::Presenter;
    use serde::{Deserialize, Serialize};
    use serde_json::{from_str, Value};
    use strfmt::strfmt;
    use wasm_bindgen::prelude::wasm_bindgen;

    #[derive(Serialize, Deserialize)]
    #[wasm_bindgen]
    pub struct Component {
        state: String,
        presenter: Box<Presenter>,
        props: String,
    }

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

        pub fn render(&self) {
            let presenter = &*self.presenter;
            match presenter {
                Presenter::Component(component) => {
                    component.render();
                }
                Presenter::Markup(markup) => {
                    let state = &self.state;
                    let state_object: Value = from_str(&state).unwrap();
                    match state_object {
                        // Value::Array(arr) => {}
                        // Value::Bool(b) => {}
                        // Value::Null => {}
                        // Value::Number(num) => {}
                        Value::Object(obj) => {
                            let a = obj.get("a");
                            // let formatted_markup = strfmt(&markup, &into_hashamp(&obj));
                        }
                        _ => {
                            // not a legal state
                        } // Value::String(st) => {}
                    }

                    //some formatting needs to be done according to state
                }
                Presenter::Nothing() => {}
            }
        }
    }
}
