use std::{future::Future, time::Duration};

use anyhow::{bail, Context};
use futures::join;
use leptos::{
    prelude::{document, GetUntracked, Set, Write},
    task::spawn_local,
};
use reactive_stores::ArcStore;
use send_wrapper::SendWrapper;
use wasm_bindgen::JsCast;
use web_sys::{DomRect, Element, HtmlElement, MutationRecord};

use crate::{
    animation::{listeners, Animation as AnimationTrait, Initializer},
    impl_empty_animation_listeners,
    utils::{animation_frame, define_options, log_error, sleep},
    TransitionDuration,
};

type Cb = SendWrapper<Box<dyn FnOnce(&HtmlElement)>>;

fn empty_cb() -> Cb {
    SendWrapper::new(Box::new(|_| ()))
}

define_options! {
    Zombie.options;
    before_enter: Cb = empty_cb(),
    enter: Cb = empty_cb(),
    @with_setters
    duration: TransitionDuration = TransitionDuration::default(),
    delay: Duration = Duration::ZERO,
    enabled: bool = true
}

/// Reappends the element in the DOM after it has been removed while applying
/// given `enter` and `before_enter` callbacks to it. Once `duration` is over,
/// the element is this time permanently removed from the DOM.
/// The "zombie" element utilizes the `position: fixed` CSS property to avoid
/// collisions with other elements of the same parent element.
///
/// It useful for creating leave animations that work only in the DOM level
/// without needing to preserve the whole component tree for the animation
/// duration and risking any weird reactivity issues.
#[must_use]
#[derive(Clone, Default)]
pub struct Zombie {
    options: ArcStore<Options>,
}

impl Zombie {
    pub fn before_enter(self, before_enter: impl FnOnce(&HtmlElement) + 'static) -> Self {
        self.options
            .clone()
            .before_enter()
            .set(SendWrapper::new(Box::new(before_enter)));
        self
    }

    pub fn enter(self, enter: impl FnOnce(&HtmlElement) + 'static) -> Self {
        self.options
            .clone()
            .enter()
            .set(SendWrapper::new(Box::new(enter)));
        self
    }
}

impl Initializer for Zombie {
    fn init_animation(self, element: HtmlElement) -> impl AnimationTrait {
        Animation::new(element, self.options)
    }
}

struct Animation {
    options: ArcStore<Options>,
    element: HtmlElement,
    parent_element: Option<Element>,
    last_rect: DomRect,
}

impl Animation {
    fn new(element: HtmlElement, options: ArcStore<Options>) -> Self {
        Self {
            options,
            last_rect: element.get_bounding_client_rect(),
            element,
            parent_element: None,
        }
    }

    fn fix_position(&self) {
        const IMPORTANT: &str = "important";

        let document_rect = document()
            .document_element()
            .expect("document to be Element")
            .get_bounding_client_rect();

        let top = self.last_rect.top() - document_rect.top();
        let left = self.last_rect.left() - document_rect.left();
        let width = self.last_rect.width();
        let height = self.last_rect.height();

        let style = self.element.style();
        _ = style.set_property_with_priority("position", "fixed", IMPORTANT);
        _ = style.set_property_with_priority("margin", "0px", IMPORTANT);
        _ = style.set_property_with_priority("top", &format!("{top}px"), IMPORTANT);
        _ = style.set_property_with_priority("left", &format!("{left}px"), IMPORTANT);
        _ = style.set_property_with_priority("width", &format!("{width}px"), IMPORTANT);
        _ = style.set_property_with_priority("pointer-events", "none", IMPORTANT);
        _ = style.set_property_with_priority(
            "height",
            &format!("{height}px"),
            "important",
        );
    }

    fn delay_fut(&self) -> impl Future<Output = ()> {
        let delay = self.options.clone().delay().get_untracked();

        async move {
            if !delay.is_zero() {
                sleep(delay).await;
            }
        }
    }

    fn record(&mut self) {
        self.last_rect = self.element.get_bounding_client_rect();
        self.parent_element = self.element.parent_element();
    }

    fn is_element_already_connected(&mut self) -> anyhow::Result<bool> {
        const ZOMBIE_ATTR: &str = "zombie";

        // The element could be already re-appended by another zombie
        let mut already_connected = self.element.is_connected();

        // or the component's DOM might have been reused by a succeeding component
        if already_connected && !self.element.has_attribute(ZOMBIE_ATTR) {
            let Ok(Ok(element)) = self
                .element
                .clone_node_with_deep(true)
                .map(<_ as JsCast>::dyn_into::<HtmlElement>)
            else {
                bail!("Could not clone node");
            };

            self.element = element;
            already_connected = false;
        } else if self.element.set_attribute(ZOMBIE_ATTR, "").is_err() {
            bail!("Could not set zombie attribute");
        }

        Ok(already_connected)
    }

    fn element_transition_duration(&self) -> (String, String) {
        let style = self.element.style();
        let original_transition_duration = style
            .get_property_value("transition-duration")
            .unwrap_or_default();
        let original_transition_duration_priority =
            style.get_property_priority("transition-duration");

        (
            original_transition_duration,
            original_transition_duration_priority,
        )
    }

    fn spawn_zombie(&mut self) -> anyhow::Result<()> {
        let already_connected = self.is_element_already_connected()?;

        let parent_element = self
            .parent_element
            .as_ref()
            .context("Parent element not found")?;

        if !parent_element.is_connected() {
            bail!("Parent element is not connected");
        }

        if !already_connected {
            self.fix_position();
        }

        std::mem::replace(
            &mut *self.options.clone().before_enter().write(),
            empty_cb(),
        )
        .take()(&self.element);

        let original_transition_duration = if already_connected {
            None
        } else {
            let v = self.element_transition_duration();

            _ = self.element.style().set_property_with_priority(
                "transition-duration",
                "0s",
                "important",
            );

            Some(v)
        };

        if !already_connected {
            parent_element.append_child(&self.element).unwrap();
        }

        spawn_local({
            let duration = self.options.clone().duration().get_untracked();
            let delay_fut = self.delay_fut();
            let element = self.element.clone();
            let enter = self.options.clone().enter();

            async move {
                join!(animation_frame(), delay_fut);

                if let Some((v, priority)) = original_transition_duration {
                    _ = element.style().set_property_with_priority(
                        "transition-duration",
                        &v,
                        &priority,
                    );
                }

                std::mem::replace(&mut *enter.write(), empty_cb()).take()(&element);
                duration.on_transition_end(&element, |element| {
                    element.remove();
                });
            }
        });

        Ok(())
    }
}

impl_empty_animation_listeners!(
    Animation;
    BeforeEnter,
    ImmediateEffect,
    Effect
);

impl listeners::Enter for Animation {
    fn listening_for_enter(&self) -> bool {
        true
    }

    fn on_enter(&mut self) {
        self.record();
    }
}

impl listeners::Mutation for Animation {
    fn listening_for_mutation(&self) -> bool {
        true
    }

    fn on_mutation(&mut self, _mutations: &[MutationRecord]) {
        self.record();
    }
}

impl listeners::ParentMutation for Animation {
    fn listening_for_parent_mutation(&self) -> bool {
        true
    }

    fn on_parent_mutation(&mut self, _mutations: &[MutationRecord]) {
        self.record();
    }
}

impl listeners::EnterAnimationsFinished for Animation {
    fn listening_for_enter_animations_finished(&self) -> bool {
        true
    }

    fn on_enter_animations_finished(&mut self) {
        self.record();
    }
}

impl listeners::MutationAnimationsFinished for Animation {
    fn listening_for_mutation_animations_finished(&self) -> bool {
        true
    }

    fn on_mutation_animations_finished(&mut self) {
        self.record();
    }
}

impl listeners::ParentMutationAnimationsFinished for Animation {
    fn listening_for_parent_mutation_animations_finished(&self) -> bool {
        true
    }

    fn on_parent_mutation_animations_finished(&mut self) {
        self.record();
    }
}

impl listeners::Cleanup for Animation {
    fn listening_for_cleanup(&self) -> bool {
        true
    }

    fn on_cleanup(&mut self) {
        if let Err(err) = self.spawn_zombie() {
            log_error!("Could not spawn zombie: {err}");
        }
    }
}

impl AnimationTrait for Animation {
    fn enabled(&self) -> bool {
        self.options.clone().enabled().get_untracked()
    }
}
