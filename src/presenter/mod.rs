pub mod presenter_mod {
    use crate::component::component_mod::Component;
    use serde::{Deserialize, Serialize};
    use serde_wasm_bindgen::{from_value, Error};
    use wasm_bindgen::{
        convert::{FromWasmAbi, IntoWasmAbi},
        describe::WasmDescribe,
        JsValue,
    };

    /// One of the building blocks of applications that utilize the integration of Rust and Javascript
    /// is data serialization. Data needs to be serialized so that it can be understood by the Wasm ABI
    /// in both directions(from Javascript to Rust and vice versa). In Rust, we often use the `serde`
    /// crate and its provided macros to handle it for us. In Javascript, we can do this by utilizing
    /// JSON.stringify method(*).
    ///
    /// Presenter enum represents what a Component actually returns(presents) to the DOM. For now,
    /// it only supports no return value, which doesn't affect the DOM; markup, which is plain HTML
    /// markup and finally, Component, which is another Component.
    ///
    /// FUTURE CONSIDERATIONS
    /// - The best case scenario, would be a support for another value, which is a mix(let's say a vector)
    ///   of Component and Markup
    /// - There is a string currently associated with the Nothing variant which is ignored but still
    ///   should be taken care of. There is no such value as Null in Rust, the closest we have is the Option enum.
    /// - *needs further investigation
    #[derive(Serialize, Deserialize)]
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
