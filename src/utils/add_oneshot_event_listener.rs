use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{AddEventListenerOptions, EventTarget};

use crate::utils::log_error;

thread_local! {
    static OPTIONS: AddEventListenerOptions = {
        let options = AddEventListenerOptions::new();
        options.set_once(true);
        options
    };
}

pub fn add_oneshot_event_listener(
    target: &EventTarget,
    type_: &str,
    closure: &Closure<dyn Fn()>,
) {
    OPTIONS.with(|options| {
        if target
            .add_event_listener_with_callback_and_add_event_listener_options(
                type_,
                closure.as_ref().unchecked_ref(),
                options,
            )
            .is_err()
        {
            log_error!("Failed to add {type_} event listener");
        }
    });
}
