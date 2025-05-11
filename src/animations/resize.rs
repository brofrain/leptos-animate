use std::time;

use reactive_stores::ArcStore;
use web_sys::HtmlElement;

mod duration;
pub use duration::Duration;

mod animation;
use animation::Animation;

use crate::{
    animation::{Animation as AnimationTrait, Initializer},
    easing::{cubic_out, Easing},
    utils::{define_options, Trackable},
};

define_options! {
    Resize.options;
    @with_setters
    easing: Easing = cubic_out,
    duration: Duration = Duration::default(),
    delay: time::Duration = time::Duration::ZERO,
    enabled: bool = true
}

/// Initializes a resize FLIP-based transition for an element.
///
/// ```no_run
/// view! {
///     <div use:animate=Resize::watch(some_signal)>
///         // ...
///     </div>
/// }
/// ```
///
/// When `some_signal` changes and influences the size of the element,
/// the rescaling animation will be played.
///
/// Note that it applies a scale transformation to the element, so if its own
/// size has changed but not the size of its content, the content will be scaled
/// as well and may look off.
#[must_use]
#[derive(Clone)]
pub struct Resize {
    trackable: Trackable,
    options: ArcStore<Options>,
}

impl Resize {
    pub fn watch(trackable: impl Into<Trackable>) -> Self {
        Self {
            trackable: trackable.into(),
            options: ArcStore::default(),
        }
    }
}

impl Initializer for Resize {
    fn init_animation(self, element: HtmlElement) -> impl AnimationTrait {
        Animation::new(element, self.trackable, self.options)
    }
}
