use std::time::Duration;

use bon::builder;
use leptos::task::spawn_local;
use wasm_bindgen::JsValue;
use web_sys::{
    js_sys::{Array, Object, Reflect},
    Animation,
    AnimationPlayState,
    HtmlElement,
    KeyframeAnimationOptions,
};

use super::sleep;
use crate::easing::Easing;

const KEYFRAME_INTERVAL_MS: f64 = 10.0;

#[builder]
pub fn spawn_animation(
    element: &HtmlElement,
    keyframe: impl Fn(f64) -> Vec<(String, String)>,
    duration: Duration,
    easing: Easing,
    #[builder(default)] delay: Duration,
) -> Animation {
    let duration = duration.as_millis() as f64;
    let keyframes = Array::new();

    let num_steps =
        (duration.max(KEYFRAME_INTERVAL_MS) / KEYFRAME_INTERVAL_MS).ceil() as usize;

    for step in 0..=num_steps {
        let keyframe = keyframe(easing(step as f64 / num_steps as f64));

        let object = Object::new();

        for (property, value) in keyframe {
            Reflect::set(&object, &property.into(), &value.into()).unwrap();
        }

        keyframes.push(&object);
    }

    let options = KeyframeAnimationOptions::new();
    options.set_duration(&JsValue::from_f64(duration));

    let animation =
        element.animate_with_keyframe_animation_options(Some(&keyframes), &options);

    if delay.is_zero() {
        return animation;
    }

    _ = animation.pause();

    spawn_local({
        let animation = animation.clone();
        let element = element.clone();
        async move {
            sleep(delay).await;

            if !element.is_connected()
                // check if the animation has already been cancelled
                || animation.play_state() == AnimationPlayState::Idle
            {
                return;
            }

            _ = animation.play();
        }
    });

    animation
}
