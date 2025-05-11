use std::time::Duration;

use leptos::prelude::ReadUntracked;
use reactive_stores::ArcStore;
use web_sys::{self, HtmlElement};

use crate::{
    animation::{Animation, Initializer},
    animations::zombie::Zombie,
    easing::{cubic_out, Easing},
    utils::{define_options, spawn_animation},
    TransitionDuration,
};

define_options! {
    Out.options;
    @with_setters
    easing: Easing = cubic_out,
    duration: Duration = Duration::from_millis(200),
    delay: Duration = Duration::ZERO,
    enabled: bool = true
}

/// Spawns a fading-out [`Zombie`] when the element leaves the DOM.
#[must_use]
#[derive(Clone, Default)]
pub struct Out {
    options: ArcStore<Options>,
}

impl Initializer for Out {
    fn init_animation(self, element: HtmlElement) -> impl Animation {
        Zombie::default()
            .enabled_signal(self.options.clone().enabled())
            .delay_signal(self.options.clone().delay())
            .duration(TransitionDuration::AnimationsFinished)
            .before_enter({
                let options = self.options;
                move |element| {
                    let options = options.read_untracked();
                    spawn_animation()
                        .element(element)
                        .keyframe(|t| vec![("opacity".into(), (1.0 - t).to_string())])
                        .duration(options.duration)
                        .easing(options.easing)
                        .delay(options.delay)
                        .call();
                }
            })
            .init_animation(element)
    }
}
