pub mod effects_mod {
    use wasm_bindgen::JsValue;
    use web_sys::js_sys::{Array, Function};

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
        component: &mut Component,
        prev_state: &JsValue,
        prev_props: &JsValue,
    ) -> Result<(), Error> {
        let effects = component.get_component_did_mount().clone();
        let effect_callbacks = effects.into_iter().map(|f| Into::<Function>::into(f));

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
                let msg = format!("{:?}", error);
                return Err(Error::EvaluationError(msg));
            } else {
                let new_state = effect_result.unwrap();

                if !new_state.is_undefined() {
                    // undefined is the default returned value in functions in JS.
                    // when no value is returned from effects, we assume that no state update has occurred.
                    let set_state_result = component.set_state_with_value(new_state);
                    if set_state_result.is_err() {
                        return Err(set_state_result.unwrap_err());
                    }
                }
            }
        }
        return Ok(());
    }

    /// Implementation details for running effects of a component, traditionally known as
    /// `component_did_update`.
    /// NOTE that its logic is partially incomplete. after any state update, a repaint must be done.
    fn component_did_update_runner(
        component: &mut Component,
        prev_state: &JsValue,
        prev_props: &JsValue,
        state: Option<&JsValue>, // `state` and `props` are used as a workaround for calling this variant during the initial render.
        props: Option<&JsValue>, // no caller from outside of this module can provide `Some` variant for these parameters, because the
                                 // exposed public function passes `None` by default. this way we make sure that effects run with the
                                 // initial state -and not the possibly updated version created by `component_did_mount` effects-
                                 // during the first render.
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
                let msg = format!("{:?}", error);
                return Err(Error::EvaluationError(msg));
            } else {
                let new_state = effect_result.unwrap();
                if !new_state.is_undefined() {
                    state_was_updated = true;
                    let set_state_result = component.set_state_with_value(new_state);
                    if set_state_result.is_err() {
                        return Err(set_state_result.unwrap_err());
                    }
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
