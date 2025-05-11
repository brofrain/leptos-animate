use std::{rc::Rc, time};

use send_wrapper::SendWrapper;

/// Represents duration of rescaling an element. Can be fixed or based on the
/// scale factor.
///
/// Default is square root of the scale factor multiplied by 0.5 seconds.
#[derive(Clone)]
pub enum Duration {
    Fixed(time::Duration),
    ScaleBased(SendWrapper<Rc<dyn Fn(f64) -> time::Duration>>),
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
        Self::ScaleBased(SendWrapper::new(Rc::new(duration_fn)))
    }
}

impl Default for Duration {
    fn default() -> Self {
        Self::ScaleBased(SendWrapper::new(Rc::new(|size| {
            time::Duration::from_secs_f64(size.sqrt() * 0.5)
        })))
    }
}
