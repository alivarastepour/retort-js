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
        let effect_callbacks = effects.into_iter().map(|f| Into::<Function>::into(f));

        let mut state_has_changed = false;

        for effect in effect_callbacks {
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
                    // undefined is the default returned value in functions in JS.
                    // when no value is returned from effects, we assume that no state update has happened.
                    let set_state_result = component.set_state_with_value(new_state);
                    state_has_changed = true;
                    if set_state_result.is_err() {
                        return Err(set_state_result.unwrap_err());
                    }
                }
            }
        }

        // if !state_has_changed {
        //     // there's no point in rerunning effects if state hasn't changed.
        //     return Ok(());
        // }

        // let result =
        //     component_did_update_runner(component, prev_state, prev_props, Some(&prev_state), None);

        // if result.is_err() {
        //     return Err(result.unwrap_err());
        // }

        return Ok(());
    }

    /// Implementation details for running effects of a component, a.k.a. `component_did_update`.
    /// Calls registered effects in the provided order.
    /// NOTE that its logic is partially incomplete. after any state update, a repaint must be done.
    fn component_did_update_runner(
        component: &mut Component,
        prev_state: &JsValue,
        prev_props: &JsValue,
        state: Option<&JsValue>,
        props: Option<&JsValue>,
    ) -> Result<(), Error> {
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
                let msg = from_value(error)
                    .unwrap_or(String::from("There was an error while running effects."));
                return Err(Error::EvaluationError(msg));
            } else {
                let new_state = effect_result.unwrap();
                if !new_state.is_undefined() {
                    state_was_updated = true;
                    // let prev_state = &component.state_parsed();
                    // let prev_props = &component.props_parsed();
                    let set_state_result = component.set_state_with_value(new_state);
                    if set_state_result.is_err() {
                        return Err(set_state_result.unwrap_err());
                    }
                    // OBSERVE: what the hell was this?
                    // return component_did_mount_runner(component, prev_state, prev_props);
                }
            }
        }

        if state_was_updated {
            return component_did_update_runner(component, prev_state, prev_props, state, props);
        }

        Ok(())
    }

    /// not implemented
    fn component_will_unmount_runner() -> Result<(), Error> {
        return Ok(());
    }

    /// Exposes effect runners to other modules. This function must be the only way of accessing functionality
    /// in this module to the outer modules.
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
                return component_did_update_runner(component, prev_state, prev_props, None, None)
            }
            Effects::ComponentWillUnmount => {
                return component_will_unmount_runner();
            }
        }
    }
}

// on component mount:
// all effects run, including didMount and effects
// didMounts run first
// then effects run
// state updates in didMounts won't trigger an immediate rerender.
// state updates in didMounts are not accessible in other didMounts
// then effects run.
// effects run once after component mount. also when state updates have happened in didMounts.(assuming any)
