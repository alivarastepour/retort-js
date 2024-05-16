pub mod effects_mod {
    use serde_wasm_bindgen::from_value;
    use wasm_bindgen::JsValue;
    use web_sys::js_sys::{Array, Function};

    use crate::{component::component_mod::Component, error::error_mod::Error};

    pub enum Effects {
        ComponentDidMount,
        ComponentDidUpdate,
        ComponentWillUnmount,
    }

    fn component_did_mount_runner(
        component: &mut Component,
        prev_state: &JsValue,
        prev_props: &JsValue,
    ) -> Result<(), Error> {
        let effects = component.get_component_did_mount().clone();
        let effects_call_result_arr = effects.into_iter().map(|f| Into::<Function>::into(f));
        // let mut state_has_changed = false;
        let mut state_results: Vec<JsValue> = Vec::new();
        // let a = effects_call_result_arr.len() as f64;
        // log_1(&JsValue::from_f64(a));
        for effect in effects_call_result_arr {
            let args: Array = Array::of4(
                prev_props,
                &JsValue::undefined(), // TODO: this must be replaced with correct value.
                prev_state,
                &component.state_parsed(),
            );
            let effect_result = effect.apply(&JsValue::undefined(), &args);
            if effect_result.is_err() {
                let error = effect_result.unwrap_err();
                let msg = from_value(error)
                    .unwrap_or(String::from("There was an error while running effects."));
                return Err(Error::EvaluationError(msg));
            } else {
                let new_state = effect_result.unwrap();
                if !new_state.is_undefined() {
                    // state_has_changed = true;
                    // let prev_state = &component.state_parsed();
                    // let prev_props = &component.props_parsed();
                    state_results.push(new_state);
                    // let set_state_result = component.set_state_with_value(new_state);
                    // if set_state_result.is_err() {
                    //     return Err(set_state_result.unwrap_err());
                    // }
                    // return component.run_effects(prev_state, prev_props);
                }
            }
        }

        // if !state_has_changed {
        //     return Ok(());
        // }

        let result = effects_runner(
            Effects::ComponentDidUpdate,
            component,
            prev_state,
            prev_props,
        );

        if result.is_err() {
            return Err(result.unwrap_err());
        }

        for new_state in state_results {
            let set_state_result = component.set_state_with_value(new_state);
            if set_state_result.is_err() {
                return Err(set_state_result.unwrap_err());
            }
        }

        return Ok(());
    }

    /// Implementation details for running effects of a component. Calls registered effects in the provided
    /// order. May call itself due to state updates that took place inside registered effects.
    /// NOTE that its logic is partially incomplete. after any state update, a repaint must be done.
    fn component_did_update_runner(
        component: &mut Component,
        prev_state: &JsValue,
        prev_props: &JsValue,
    ) -> Result<(), Error> {
        let effects = component.get_effects().clone();
        let effects_call_result_arr = effects.into_iter().map(|f| Into::<Function>::into(f));
        for effect in effects_call_result_arr {
            let args: Array = Array::of4(
                prev_props,
                &JsValue::undefined(), // TODO: this must be replaced with correct value.
                prev_state,
                &component.state_parsed(),
            );
            let effect_result = effect.apply(&JsValue::undefined(), &args);
            if effect_result.is_err() {
                let error = effect_result.unwrap_err();
                let msg = from_value(error)
                    .unwrap_or(String::from("There was an error while running effects."));
                return Err(Error::EvaluationError(msg));
            } else {
                let new_state = effect_result.unwrap();
                if !new_state.is_undefined() {
                    let prev_state = &component.state_parsed();
                    let prev_props = &component.props_parsed();
                    let set_state_result = component.set_state_with_value(new_state);
                    if set_state_result.is_err() {
                        return Err(set_state_result.unwrap_err());
                    }
                    return component_did_mount_runner(component, prev_state, prev_props);
                }
            }
        }

        Ok(())
    }

    fn component_will_unmount_runner() -> Result<(), Error> {
        return Ok(());
    }

    pub fn effects_runner(
        effect: Effects,
        component: &mut Component,
        prev_state: &JsValue,
        prev_props: &JsValue,
    ) -> Result<(), Error> {
        match effect {
            Effects::ComponentDidMount => {
                return component_did_mount_runner(component, prev_state, prev_props);
            }
            Effects::ComponentDidUpdate => {
                return component_did_update_runner(component, prev_state, prev_props)
            }
            Effects::ComponentWillUnmount => {
                return component_will_unmount_runner();
            }
        }
    }

    // impl Effect for ComponentDidMountEffects {}

    // impl Effect for ComponentDidMountEffects {}
}

// on component mount:
// all effects run, including didMount and effects
// didMounts run first
// then effects run
// state updates in didMounts won't trigger an immediate rerender.
// state updates in didMounts are not accessible in other didMounts
// then effects run.
// effects run once after component mount. also when state updates have happened in didMounts.(assuming any)
