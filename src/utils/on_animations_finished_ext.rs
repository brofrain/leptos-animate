use leptos::task::spawn_local;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{GetAnimationsOptions, HtmlElement};

use super::{add_oneshot_event_listener, animation_frame};

pub trait OnAnimationsFinishedExt {
    fn on_animations_finished(&self, cb: impl Fn() + 'static, subtree: bool);
}

fn get_animations(element: &HtmlElement, subtree: bool) -> Vec<web_sys::Animation> {
    let animations = if subtree {
        let options = GetAnimationsOptions::new();
        options.set_subtree(true);
        element.get_animations_with_options(&options)
    } else {
        element.get_animations()
    };

    animations
        .into_iter()
        .filter_map(|animation| animation.dyn_into::<web_sys::Animation>().ok())
        .collect()
}

impl OnAnimationsFinishedExt for HtmlElement {
    fn on_animations_finished(&self, cb: impl Fn() + 'static, subtree: bool) {
        let closure = Closure::<dyn Fn()>::new(cb);

        for animation in &get_animations(self, subtree) {
            add_oneshot_event_listener(animation, "finish", &closure);
        }

        // sometimes animations appear in the next tick, so let's catch them too
        spawn_local({
            let element = self.clone();
            async move {
                animation_frame().await;

                for animation in get_animations(&element, subtree) {
                    add_oneshot_event_listener(&animation, "finish", &closure);
                }

                closure.forget();
            }
        });
    }
}
