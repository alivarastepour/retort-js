pub mod effects_mod {
    use std::any::Any;

    use serde_wasm_bindgen::to_value;
    use wasm_bindgen::{convert::IntoWasmAbi, JsValue};
    use web_sys::{
        console::{log_1, time_end_with_label, time_with_label},
        js_sys::{Array, Function, JSON},
    };

    use serde_json::{from_str, Map, Value};

    use crate::{component::component_mod::Component, error::error_mod::Error};

    /// An enum to keep variants of different effects in a component.
    pub enum Effects {
        ComponentDidMount,
        ComponentDidUpdate,
        ComponentWillUnmount,
    }

    /// Implementation details for running initial effects of a component, traditionally known
    /// as `component_did_mount`. Returns `Ok` if no error occurs while running effects; an `Err` variant
    /// explaining why otherwise.
    /// NOTE that its logic is partially incomplete. after any state update, a repaint must be done.
    fn component_did_mount_runner(
        component: &Component,
        prev_state: &JsValue,
        prev_props: &JsValue,
    ) -> Result<bool, Error> {
        // let a: Map<String, Value> = JsValue::intos
        // let a = from_str(s)
        let effects = component.get_component_did_mount().clone();
        let effect_callbacks = effects.into_iter().map(|f| Into::<Function>::into(f));

        for effect in effect_callbacks {
            let args: Array = Array::of4(
                prev_props,
                &JsValue::undefined(), // TODO: this must be replaced with correct value.
                prev_state,
                &component.state_parsed(),
            );

            // let a = to_value(&component).unwrap();
            // log_1(&a);
            let effect_result = effect.apply(&JsValue::undefined(), &args);
            // let a = to_value(&component).unwrap();
            // log_1(&a);
            // log_1(&a);

            if effect_result.is_err() {
                let error = effect_result.unwrap_err();
                let msg = format!("{:?}", error);
                return Err(Error::EvaluationError(msg));
            }

            // else {
            //     let new_state = effect_result.unwrap();

            //     if !new_state.is_undefined() {
            //         // undefined is the default returned value in functions in JS.
            //         // when no value is returned from effects, we assume that no state update has occurred.
            //         let set_state_result = component.set_state_with_value(new_state);
            //         if set_state_result.is_err() {
            //             return Err(set_state_result.unwrap_err());
            //         }
            //     }
            // }
        }

        // time_with_label("cdm check: ");
        let a = JSON::stringify(prev_state).unwrap();
        let b = JSON::stringify(&component.state_parsed()).unwrap();
        let c = a != b;
        // log_1(&component.state_parsed());
        // time_end_with_label("cdm check: ");

        return Ok(c);
    }

    /// Implementation details for running effects of a component, traditionally known as
    /// `component_did_update`.
    /// NOTE that its logic is partially incomplete. after any state update, a repaint must be done.
    fn component_did_update_runner(
        component: &Component,
        prev_state: &JsValue,
        prev_props: &JsValue,
        state: Option<&JsValue>, // `state` and `props` are used as a workaround for calling this variant during the initial render.
        props: Option<&JsValue>, // no caller from outside of this module can provide `Some` variant for these parameters, because the
                                 // exposed public function passes `None` by default. this way we make sure that effects run with the
                                 // initial state -and not the possibly updated version created by `component_did_mount` effects-
                                 // during the first render.
    ) -> Result<bool, Error> {
        let effects = component.get_effects().clone();
        let effect_callbacks = effects.into_iter().map(|f| Into::<Function>::into(f));
        let mut state_was_updated = false;
        for effect in effect_callbacks {
            let args: Array = Array::of4(
                prev_props,
                &JsValue::undefined(), // TODO: this must be replaced with correct value.
                prev_state,
                state.unwrap_or(&component.state_parsed()),
            );
            let effect_result = effect.apply(&JsValue::undefined(), &args);
            if effect_result.is_err() {
                let error = effect_result.unwrap_err();
                let msg = format!("{:?}", error);
                return Err(Error::EvaluationError(msg));
            }
            // else {
            //     let new_state = effect_result.unwrap();
            //     if !new_state.is_undefined() {
            //         state_was_updated = true;
            //         let set_state_result = component.set_state_with_value(new_state);
            //         if set_state_result.is_err() {
            //             return Err(set_state_result.unwrap_err());
            //         }
            //     }
            // }
        }

        // time_with_label("cdu check: ");
        let a = JSON::stringify(prev_state).unwrap();
        let b = JSON::stringify(&component.state_parsed()).unwrap();
        let c = a != b;
        log_1(&JsValue::from_bool(c));
        // time_end_with_label("cdu check: ");

        // if state_was_updated {
        // return component_did_update_runner(component, prev_state, prev_props, state, props);
        // }

        Ok(c)
    }

    /// not implemented
    fn component_will_unmount_runner() -> Result<bool, Error> {
        return Ok(true);
    }

    /// Exposes effect runners to other modules. This function must be the only way of accessing functionality
    /// in this module to the outer modules.
    pub fn effects_runner(
        effect: Effects,
        component: &Component,
        prev_state: &JsValue,
        prev_props: &JsValue,
    ) -> Result<bool, Error> {
        match effect {
            Effects::ComponentDidMount => {
                return component_did_mount_runner(component, prev_state, prev_props);
            }
            Effects::ComponentDidUpdate => {
                return component_did_update_runner(component, prev_state, prev_props, None, None)
            }
            Effects::ComponentWillUnmount => {
                return component_will_unmount_runner();
            }
        }
    }
}
