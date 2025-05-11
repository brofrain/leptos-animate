use std::{cell::RefCell, rc::Rc};

use initial::Initial;
use leptos::{
    prelude::{on_cleanup, Effect, ImmediateEffect, StoredValue},
    task::{spawn_local, tick},
};
use send_wrapper::SendWrapper;
use use_observer::use_observer;
use wasm_bindgen::JsCast;
use web_sys::{self, Element, HtmlElement, MutationRecord};

use crate::{
    animation::{Animation, Ext, Initializer},
    utils::{animation_frame, log_error, OnAnimationsFinishedExt},
};

mod initial;
mod use_observer;

fn parent(element: &HtmlElement) -> Option<HtmlElement> {
    element.parent_element()?.dyn_into::<HtmlElement>().ok()
}

type AnimationCell = Rc<RefCell<dyn Animation>>;

struct Composer {
    element: HtmlElement,
    animation: AnimationCell,
}

impl Composer {
    fn owned_element_and_animation(&self) -> (HtmlElement, AnimationCell) {
        (self.element.clone(), Rc::clone(&self.animation))
    }

    fn setup_enter(&self) {
        let (element, animation) = self.owned_element_and_animation();

        spawn_local(async move {
            animation_frame().await;
            animation.delayed_on_enter(&element);
        });
    }

    fn setup_enter_animations_finished(&self) {
        let (element, animation) = self.owned_element_and_animation();

        spawn_local(async move {
            element.on_animations_finished(
                move || animation.borrow_mut().on_enter_animations_finished(),
                false,
            );
        });
    }

    fn setup_immediate_effect(&self) {
        let (element, animation) = self.owned_element_and_animation();
        let element = SendWrapper::new(element);
        let animation = SendWrapper::new(animation);
        let initial = Initial::new();

        StoredValue::new(ImmediateEffect::new(move || {
            animation.borrow().track();

            if initial.get() || !animation.borrow().enabled() || !element.is_connected() {
                return;
            }

            animation.borrow_mut().on_immediate_effect();
        }));
    }

    fn setup_effect(&self) {
        let (element, animation) = self.owned_element_and_animation();
        let initial = Initial::new();

        Effect::new(move || {
            animation.borrow_mut().track();

            if initial.get() {
                return;
            }

            let animation = Rc::clone(&animation);
            let element = element.clone();
            spawn_local(async move {
                tick().await;
                animation.delayed_on_effect(&element);
            });
        });
    }

    fn setup_mutation(&self) {
        let (element, animation) = self.owned_element_and_animation();

        use_observer(&element.clone(), move |mutations| {
            if !animation.borrow().enabled() || !element.is_connected() {
                return;
            }

            animation.borrow_mut().on_mutation(&mutations);
        });
    }

    fn setup_mutation_animations_finished(&self) {
        let (element, animation) = self.owned_element_and_animation();

        use_observer(&element.clone(), move |_| {
            let animation = Rc::clone(&animation);
            element.on_animations_finished(
                move || {
                    animation.borrow_mut().on_mutation_animations_finished();
                },
                true,
            );
        });
    }

    fn observe_parent(
        &self,
        cb: impl Fn(&HtmlElement, Vec<MutationRecord>) + Clone + 'static,
    ) {
        let element = self.element.clone();

        Effect::new(move || {
            let Some(parent_element) = parent(&element) else {
                return;
            };

            let cb = cb.clone();
            use_observer(&parent_element.clone(), move |mutations| {
                cb(&parent_element, mutations);
            });
        });
    }

    fn setup_parent_mutation(&self) {
        let animation = Rc::clone(&self.animation);

        self.observe_parent(move |parent_element, mutations| {
            if !animation.borrow().enabled() || !parent_element.is_connected() {
                return;
            }

            animation.borrow_mut().on_parent_mutation(&mutations);
        });
    }

    fn setup_parent_mutation_animations_finished(&self) {
        let animation = Rc::clone(&self.animation);

        self.observe_parent(move |parent_element, _| {
            let animation = Rc::clone(&animation);
            parent_element.on_animations_finished(
                move || {
                    animation
                        .borrow_mut()
                        .on_parent_mutation_animations_finished();
                },
                true,
            );
        });
    }

    fn compose(self) {
        if self.animation.borrow().listening_for_before_enter()
            && self.animation.borrow().enabled()
        {
            self.animation.borrow_mut().on_before_enter();
        }

        if self.animation.borrow().listening_for_enter() {
            self.setup_enter();
        }

        if self.animation.borrow().listening_for_immediate_effect() {
            self.setup_immediate_effect();
        }

        if self.animation.borrow().listening_for_effect() {
            self.setup_effect();
        }

        if self.animation.borrow().listening_for_mutation() {
            self.setup_mutation();
        }

        if self.animation.borrow().listening_for_parent_mutation() {
            self.setup_parent_mutation();
        }

        if self
            .animation
            .borrow()
            .listening_for_enter_animations_finished()
        {
            self.setup_enter_animations_finished();
        }

        if self
            .animation
            .borrow()
            .listening_for_mutation_animations_finished()
        {
            self.setup_mutation_animations_finished();
        }

        if self
            .animation
            .borrow()
            .listening_for_parent_mutation_animations_finished()
        {
            self.setup_parent_mutation_animations_finished();
        }

        if self.animation.borrow().listening_for_cleanup() {
            let animation = SendWrapper::new(self.animation);
            on_cleanup(move || {
                animation.borrow_mut().on_cleanup();
            });
        }
    }
}

/// Directive to attach and manage animations on an element.
///
/// Usage:
/// ```no_run
/// view! {
///     <div use:animate=SomeAnimation>
///         // ...
///     </div>
/// }
/// ```
///
/// where `SomeAnimation` must implement [`Initializer`] trait.
///
/// The directive runs when the element is created or hydrated.
pub fn animate(element: Element, animation_initializer: impl Initializer) {
    let Ok(element) = element.dyn_into::<HtmlElement>() else {
        log_error!("Could not animate a non-HtmlElement");
        return;
    };

    let animation: AnimationCell = Rc::new(RefCell::new(
        animation_initializer.init_animation(element.clone()),
    ));

    Composer { element, animation }.compose();
}
