use std::rc::Rc;

use leptos::prelude::Track;

#[derive(Clone)]
pub struct Trackable(Rc<dyn Fn()>);

impl Trackable {
    pub fn track(&self) {
        (self.0)();
    }
}

impl<T> From<T> for Trackable
where
    T: Track + 'static,
{
    fn from(value: T) -> Self {
        Self(Rc::new(move || {
            value.track();
        }))
    }
}
