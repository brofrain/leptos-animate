use std::{rc::Rc, time};

use send_wrapper::SendWrapper;

/// Represents duration of move transition in FLIP. Can be fixed or based on the
/// distance to travel.
///
/// Default is square root of distance multiplied by 0.05 seconds.
#[derive(Clone)]
pub enum Duration {
    Fixed(time::Duration),
    DistanceBased(SendWrapper<Rc<dyn Fn(f64) -> time::Duration>>),
}

impl From<time::Duration> for Duration {
    fn from(duration: time::Duration) -> Self {
        Self::Fixed(duration)
    }
}

impl<F> From<F> for Duration
where
    F: Fn(f64) -> time::Duration + 'static,
{
    fn from(duration_fn: F) -> Self {
        Self::DistanceBased(SendWrapper::new(Rc::new(duration_fn)))
    }
}

impl Default for Duration {
    fn default() -> Self {
        Self::DistanceBased(SendWrapper::new(Rc::new(|distance| {
            time::Duration::from_secs_f64(distance.sqrt() * 0.05)
        })))
    }
}
