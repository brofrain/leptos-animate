use std::time;

use leptos::prelude::ReadUntracked;
use reactive_stores::ArcStore;
use web_sys::{self, DomRect};

use super::{Duration, Options};
use crate::{
    animation::{listeners, Animation as AnimationTrait},
    impl_empty_animation_listeners,
    utils::{spawn_animation, Trackable},
};

pub struct Animation {
    element: web_sys::HtmlElement,
    trackable: Trackable,
    options: ArcStore<Options>,
    last_rect: DomRect,
    handle: Option<web_sys::Animation>,
}

fn progress_scale_axis(value: f64, progress: f64) -> f64 {
    (value - 1.0) * (1.0 - progress) + 1.0
}

impl Animation {
    pub fn new(
        element: web_sys::HtmlElement,
        trackable: Trackable,
        options: ArcStore<Options>,
    ) -> Self {
        Self {
            last_rect: element.get_bounding_client_rect(),
            element,
            trackable,
            options,
            handle: None,
        }
    }

    fn compute_scale(&mut self) -> Option<(f64, f64)> {
        let new_rect = self.element.get_bounding_client_rect();
        let new_width = new_rect.width();
        let new_height = new_rect.height();

        if new_width == 0.0 || new_height == 0.0 {
            return None;
        }

        let last_width = self.last_rect.width();
        let last_height = self.last_rect.height();

        let sx = last_width / new_width;
        let sy = last_height / new_height;

        Some((sx, sy))
    }

    fn compute_duration(&self, sx: f64, sy: f64) -> time::Duration {
        match &self.options.read_untracked().duration {
            Duration::Fixed(duration) => *duration,
            Duration::ScaleBased(duration_fn) => duration_fn(sx.abs().max(sy.abs())),
        }
    }

    fn resize(&mut self) {
        let Some((sx, sy)) = self.compute_scale() else {
            return;
        };

        if (sx - 1.0).abs() > f64::EPSILON || (sy - 1.0).abs() > f64::EPSILON {
            let scale = move |t: f64| {
                let sx = progress_scale_axis(sx, t);
                let sy = progress_scale_axis(sy, t);
                format!("{sx} {sy}")
            };

            let duration = self.compute_duration(sx, sy);

            let options = self.options.read_untracked();
            let handle = spawn_animation()
                .element(&self.element)
                .keyframe(|t| vec![("scale".into(), scale(t))])
                .duration(duration)
                .easing(options.easing)
                .delay(options.delay)
                .call();

            self.handle = Some(handle);
        }
    }
}

impl_empty_animation_listeners!(
    Animation;
    BeforeEnter,
    Enter,
    EnterAnimationsFinished,
    Mutation,
    MutationAnimationsFinished,
    ParentMutation,
    ParentMutationAnimationsFinished,
    Cleanup
);

impl listeners::ImmediateEffect for Animation {
    fn listening_for_immediate_effect(&self) -> bool {
        true
    }

    fn on_immediate_effect(&mut self) {
        self.last_rect = self.element.get_bounding_client_rect();
    }
}

impl listeners::Effect for Animation {
    fn listening_for_effect(&self) -> bool {
        true
    }

    fn on_effect(&mut self) {
        self.resize();
    }
}

impl AnimationTrait for Animation {
    fn enabled(&self) -> bool {
        self.options.read_untracked().enabled
    }

    fn track(&self) {
        self.trackable.track();
    }
}
