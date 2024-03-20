pub mod presenter_mod {
    use crate::component::component_mod::Component;
    use serde::{Deserialize, Serialize};
    use serde_wasm_bindgen::{from_value, to_value, Error};
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
        /// Clones a Presenter variant based on its criteria
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

    /// rn, I have no clue why this is necessary.
    impl WasmDescribe for Presenter {
        fn describe() {
            JsValue::describe()
        }
    }

    impl FromWasmAbi for Presenter {
        /// Handles the type conversion of a value which is coming out of Wasm ABI.
        /// The value is first converted into a JsValue and is then converted to a Presenter variant.
        /// In case of error while converting JsValue to Presenter, the error message is wrapped inside
        /// a Presenter::Markup for debugging purposes.
        unsafe fn from_abi(js: Self::Abi) -> Self {
            let presenter_js_value = JsValue::from_abi(js);
            let presenter_result: Result<Presenter, Error> = from_value(presenter_js_value);
            if let Result::Err(err) = presenter_result {
                return Presenter::Markup(err.to_string());
            }
            presenter_result.unwrap()
        }

        /// The Abi associated type of JsValue struct in its FromWasmAbi implementation block
        type Abi = <JsValue as FromWasmAbi>::Abi;
    }

    impl IntoWasmAbi for Presenter {
        /// Handles the type conversion of a value which is passing Wasm ABI.
        /// In case of error while converting Presenter to JsValue, execution panics.
        /// Note that using the same error handling as FromWasmAbi block may not be the
        /// best solution since a new Presenter enum must go through the same conversion which
        /// failed in the first place.
        fn into_abi(self) -> Self::Abi {
            let presenter_js_value = to_value(&self);
            if let Result::Err(err) = presenter_js_value {
                panic!("Operation failed while converting Presenter enum to JsValue: {err}")
            }
            presenter_js_value.unwrap().into_abi()
        }

        /// The Abi associated type of JsValue struct in its IntoWasmAbi implementation block
        type Abi = <JsValue as IntoWasmAbi>::Abi;
    }
}
