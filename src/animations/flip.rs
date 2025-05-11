use std::time;

use reactive_stores::ArcStore;
use web_sys::HtmlElement;

use crate::{
    animation::{Animation as AnimationTrait, Initializer},
    easing::{cubic_out, Easing},
    utils::{define_options, Trackable},
};

mod duration;
pub use duration::Duration;

mod animation;
use animation::Animation;

define_options! {
    Flip.options;
    @with_setters
    easing: Easing = cubic_out,
    duration: Duration = Duration::default(),
    delay: time::Duration = time::Duration::ZERO,
    enabled: bool = true
}

/// Initializes FLIP transition for an element.
///
/// ```no_run
/// view! {
///     <div use:animate=Flip::watch(some_signal)>
///         // ...
///     </div>
/// }
/// ```
///
/// When `some_signal` changes and influences the position of the element,
/// the travel animation will be played.
#[must_use]
#[derive(Clone)]
pub struct Flip {
    trackable: Trackable,
    options: ArcStore<Options>,
}

impl Flip {
    pub fn watch(trackable: impl Into<Trackable>) -> Self {
        Self {
            trackable: trackable.into(),
            options: ArcStore::default(),
        }
    }
}

impl Initializer for Flip {
    fn init_animation(self, element: HtmlElement) -> impl AnimationTrait {
        let Self { trackable, options } = self;
        Animation::new(element, trackable, options)
    }
}
