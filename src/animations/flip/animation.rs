use std::time;

use anyhow::{anyhow, bail, Context};
use leptos::prelude::{window, ReadUntracked};
use reactive_stores::ArcStore;
use web_sys::{self, DomRect};

use super::{Duration, Options};
use crate::{
    animation::{listeners, Animation as AnimationTrait},
    impl_empty_animation_listeners,
    utils::{log_error, spawn_animation, Trackable},
};

pub struct Animation {
    element: web_sys::HtmlElement,
    trackable: Trackable,
    options: ArcStore<Options>,
    last_rect: DomRect,
    handle: Option<web_sys::Animation>,
}

impl Animation {
    pub fn new(
        element: web_sys::HtmlElement,
        trackable: Trackable,
        options: ArcStore<Options>,
    ) -> Self {
        Self {
            trackable,
            last_rect: element.get_bounding_client_rect(),
            element,
            options,
            handle: None,
        }
    }

    fn compute_transform_origin(&self) -> anyhow::Result<(f64, f64)> {
        let Ok(Some(computed_style)) = window().get_computed_style(&self.element) else {
            bail!("Could not get computed style");
        };

        let origin = computed_style
            .get_property_value("transform-origin")
            .map_err(|err| anyhow!("Could not get computed transform-origin: {err:#?}"))?
            .replace("px", "");
        let mut origin = origin.split(' ');

        {
            const ERR: &str = "Invalid transform-origin format";

            let ox = origin.next().context(ERR)?.parse::<f64>()?;
            let oy = origin.next().context(ERR)?.parse::<f64>()?;

            Ok((ox, oy))
        }
    }

    fn compute_distance(&self) -> anyhow::Result<(f64, f64)> {
        let new_rect = self.element.get_bounding_client_rect();
        let new_width = new_rect.width();
        let new_height = new_rect.height();

        // if the element is no longer visible, then it could not possibly move
        if new_width == 0.0 || new_height == 0.0 {
            return Ok((0.0, 0.0));
        }

        let (ox, oy) = self.compute_transform_origin()?;

        let dx = self.last_rect.left() + (self.last_rect.width() * ox / new_width)
            - (new_rect.left() + ox);

        let dy = self.last_rect.top() + (self.last_rect.height() * oy / new_height)
            - (new_rect.top() + oy);

        Ok((dx, dy))
    }

    fn compute_duration(&self, dx: f64, dy: f64) -> time::Duration {
        match &self.options.read_untracked().duration {
            Duration::Fixed(duration) => *duration,
            Duration::DistanceBased(duration_fn) => {
                duration_fn((dx * dx + dy * dy).sqrt())
            }
        }
    }

    fn clear_previous_animation(&mut self) {
        if let Some(animation) = self.handle.take() {
            animation.cancel();
        }
    }

    fn flip(&mut self) -> anyhow::Result<()> {
        self.clear_previous_animation();

        let (dx, dy) = self.compute_distance()?;

        if dx.abs() > f64::EPSILON || dy.abs() > f64::EPSILON {
            let translate = move |t| {
                let rev = 1.0 - t;
                format!("translate({}px,{}px)", dx * rev, dy * rev)
            };

            let duration = self.compute_duration(dx, dy);

            let options = self.options.read_untracked();
            let handle = spawn_animation()
                .element(&self.element)
                .keyframe(|t| vec![("transform".into(), translate(t))])
                .duration(duration)
                .easing(options.easing)
                .delay(options.delay)
                .call();

            self.handle = Some(handle);
        }

        Ok(())
    }
}

impl_empty_animation_listeners!(
    Animation;
    BeforeEnter,
    Enter,
    Mutation,
    ParentMutation,
    EnterAnimationsFinished,
    MutationAnimationsFinished,
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
        if let Err(err) = self.flip() {
            log_error!("Failed to perform a FLIP animation: {err}");
        }
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
