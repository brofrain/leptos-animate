use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
    time::Duration,
};

use leptos::task::spawn_local;
use web_sys::HtmlElement;

use super::Animation;
use crate::utils::sleep;

pub trait Ext {
    fn delayed_on_enter(&self, element: &HtmlElement);
    fn delayed_on_effect(&self, element: &HtmlElement);
}

type AnimationCell = Rc<RefCell<dyn Animation>>;

fn should_run(animation: &AnimationCell, element: &HtmlElement) -> bool {
    animation.borrow().enabled() && element.is_connected()
}

fn run_now_or_after_delay(
    animation: &AnimationCell,
    element: &HtmlElement,
    delay: impl Fn(Ref<'_, dyn Animation>) -> Duration,
    cb: impl Fn(RefMut<'_, dyn Animation>) + 'static,
) {
    let delay = delay(animation.borrow());

    if delay.is_zero() {
        if should_run(animation, element) {
            cb(animation.borrow_mut());
        }
        return;
    }

    let animation = Rc::clone(animation);
    let element = element.clone();
    spawn_local(async move {
        sleep(delay).await;

        if should_run(&animation, &element) {
            cb(animation.borrow_mut());
        }
    });
}

impl Ext for AnimationCell {
    fn delayed_on_enter(&self, element: &HtmlElement) {
        run_now_or_after_delay(self, element, |a| a.enter_delay(), |mut a| a.on_enter());
    }

    fn delayed_on_effect(&self, element: &HtmlElement) {
        run_now_or_after_delay(
            self,
            element,
            |a| a.effect_delay(),
            |mut a| a.on_effect(),
        );
    }
}
