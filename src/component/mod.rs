pub mod component_mod {
    use crate::presenter::presenter_mod::Presenter;
    use serde::{Deserialize, Serialize};
    use wasm_bindgen::prelude::wasm_bindgen;

    #[derive(Serialize, Deserialize)]
    #[wasm_bindgen]
    pub struct Component {
        state: String,
        presenter: Box<Presenter>,
        props: String,
    }

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
    }
}
