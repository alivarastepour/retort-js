pub mod presenter_mod {
    use crate::component::component_mod::Component;
    use serde::{Deserialize, Serialize};
    use serde_wasm_bindgen::{from_value, Error};
    use wasm_bindgen::{
        convert::{FromWasmAbi, IntoWasmAbi},
        describe::WasmDescribe,
        JsValue,
    };

    #[derive(Serialize, Deserialize)]
    // #[serde(tag = "type", content = "data")]
    pub enum Presenter {
        Component(Component),
        Markup(String),
        Nothing(String),
    }

    impl Clone for Presenter {
        fn clone(&self) -> Self {
            match self {
                Presenter::Component(component) => {
                    let new_component = component.clone();
                    Presenter::Component(new_component)
                }
                Presenter::Markup(markup) => Presenter::Markup(markup.to_string()),
                Presenter::Nothing(_) => Presenter::Nothing("".to_string()),
            }
        }
    }

    impl WasmDescribe for Presenter {
        fn describe() {
            JsValue::describe()
        }
    }

    impl FromWasmAbi for Presenter {
        unsafe fn from_abi(js: Self::Abi) -> Self {
            let a = JsValue::from_abi(js);
            let ea: Result<Presenter, Error> = from_value(a);
            if let Result::Err(err) = ea {
                return Presenter::Markup(err.to_string());
            }
            ea.unwrap()
        }
        type Abi = <JsValue as FromWasmAbi>::Abi;
    }

    impl IntoWasmAbi for Presenter {
        fn into_abi(self) -> Self::Abi {
            serde_wasm_bindgen::to_value(&self).unwrap().into_abi()
        }
        type Abi = <JsValue as IntoWasmAbi>::Abi;
    }
}
