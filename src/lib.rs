pub mod lib_mod {
    use serde::{Deserialize, Serialize};
    use serde_wasm_bindgen::from_value;
    use wasm_bindgen::{
        convert::{FromWasmAbi, IntoWasmAbi},
        describe::WasmDescribe,
        prelude::*,
        JsValue,
    };

    #[derive(Serialize, Deserialize)]
    #[serde(tag = "type", content = "data")]
    pub enum Presenter {
        Component(Component),
        Markup(String),
        Nothing(),
    }

    impl WasmDescribe for Presenter {
        fn describe() {
            JsValue::describe()
        }
    }

    // impl Clone for Presenter {
    //     fn clone(&self) -> Self {
    //         match self {
    //             Presenter::Component(c) => Presenter::Component(c.to_owned()),
    //             Presenter::Markup => Presenter::Markup,
    //             Presenter::Nothing => Presenter::Nothing,
    //         }
    //     }
    // }

    // impl From<Presenter> for JsValue {
    //     fn from(val: Presenter) -> Self {
    //         serde_wasm_bindgen::to_value(&val).unwrap()
    //     }
    // }

    // impl IntoWasmAbi for Presenter {
    //     type Abi = <JsValue as IntoWasmAbi>::Abi;

    //     fn into_abi(self) -> Self::Abi {
    //         serde_wasm_bindgen::to_value(&self)
    //             .unwrap_or(JsValue::undefined())
    //             .into_abi()
    //     }
    // }

    impl FromWasmAbi for Presenter {
        unsafe fn from_abi(js: Self::Abi) -> Self {
            from_value(JsValue::from_abi(js)).unwrap_or(Presenter::Nothing())
        }
        type Abi = <JsValue as FromWasmAbi>::Abi;
    }

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
