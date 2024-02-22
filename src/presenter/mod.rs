pub mod presenter_mod {
    use crate::component::component_mod::Component;
    use serde::{Deserialize, Serialize};
    use serde_wasm_bindgen::from_value;
    use wasm_bindgen::{convert::FromWasmAbi, describe::WasmDescribe, JsValue};

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

    impl FromWasmAbi for Presenter {
        unsafe fn from_abi(js: Self::Abi) -> Self {
            from_value(JsValue::from_abi(js)).unwrap_or(Presenter::Nothing())
        }
        type Abi = <JsValue as FromWasmAbi>::Abi;
    }
}
