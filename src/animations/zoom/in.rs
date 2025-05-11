use std::time::Duration;

use leptos::prelude::ReadUntracked;
use reactive_stores::ArcStore;
use web_sys::{self, HtmlElement};

use crate::{
    animation::{listeners, Animation as AnimationTrait, Initializer},
    easing::{cubic_out, Easing},
    impl_empty_animation_listeners,
    utils::{define_options, spawn_animation},
};

define_options! {
    In.options;
    @with_setters
    easing: Easing = cubic_out,
    duration: Duration = Duration::from_millis(200),
    delay: Duration = Duration::ZERO,
    enabled: bool = true
}

/// Zooms-in an element when it enters the DOM.
#[must_use]
#[derive(Clone, Default)]
pub struct In {
    options: ArcStore<Options>,
}

impl Initializer for In {
    fn init_animation(self, element: HtmlElement) -> impl AnimationTrait {
        Animation {
            element,
            options: self.options,
        }
    }
}

struct Animation {
    element: HtmlElement,
    options: ArcStore<Options>,
}

impl_empty_animation_listeners!(
    Animation;
    Enter,
    ImmediateEffect,
    Effect,
    Mutation,
    ParentMutation,
    EnterAnimationsFinished,
    MutationAnimationsFinished,
    ParentMutationAnimationsFinished,
    Cleanup
);

impl listeners::BeforeEnter for Animation {
    fn listening_for_before_enter(&self) -> bool {
        true
    }

    fn on_before_enter(&mut self) {
        let options = self.options.read_untracked();
        spawn_animation()
            .element(&self.element)
            .keyframe(|t| vec![("scale".into(), t.to_string())])
            .duration(options.duration)
            .easing(options.easing)
            .delay(options.delay)
            .call();
    }
}

impl AnimationTrait for Animation {
    fn enabled(&self) -> bool {
        self.options.read_untracked().enabled
    }
}
