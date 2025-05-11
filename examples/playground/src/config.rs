use std::time::Duration;

use educe::Educe;
use leptos_animate::easing::{cubic_out, Easing};
use reactive_stores_macro::Store;

#[derive(Store, Educe)]
#[educe(Default)]
pub struct Transition {
    #[educe(Default = Duration::from_secs(1))]
    duration: Duration,
    #[educe(Default = Duration::ZERO)]
    delay: Duration,
    #[educe(Default = cubic_out)]
    easing: Easing,
    #[educe(Default = true)]
    enabled: bool,
}

#[derive(Store, Default)]
pub struct Config {
    fade_in: Transition,
    fade_out: Transition,
    zoom_in: Transition,
    zoom_out: Transition,

    // Flip and resize may have separate configuration structs due to their more flexible
    // durations. That would, however, require slight UI enhancements.
    flip: Transition,
    resize: Transition,

    rng_seed: usize,
}
