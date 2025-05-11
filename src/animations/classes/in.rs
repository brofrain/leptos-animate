use std::time::Duration;

use leptos::prelude::{ReadUntracked, WithUntracked};
use reactive_stores::ArcStore;
use web_sys::HtmlElement;

use super::html_element_ext::HtmlElementExt;
use crate::{
    animation::{listeners, Animation as AnimationTrait, Initializer},
    impl_empty_animation_listeners,
    utils::{define_options, log_error},
    TransitionDuration,
};

define_options! {
    In.options;
    @with_setters
    duration: TransitionDuration = TransitionDuration::default(),
    delay: Duration = Duration::ZERO,
    enabled: bool = true,
    source: String = String::new(),
    target: String = String::new(),
    active: String = String::new()
}

/// Applies CSS classes to an element when it enters the DOM:
/// - `source` - initial classes added before the element is inserted and
///   removed one frame after
/// - `active` - applied during the entire entering phase and removed once the
///   animation is finished.
/// - `target` - classed added one frame after the element is inserted and
///   removed once the animation is finished.
///
/// Example fade-in animation utilizing Tailwind CSS:
/// ```no_run
/// view! {
///     <div
///         use:animate=In::default()
///             .source("opacity-0")
///             .active("duration-150")
///             .target("opacity-100")
///     >
///         // ...
///     </div>
/// }
/// ```
#[must_use]
#[derive(Clone, Default)]
pub struct In {
    options: ArcStore<Options>,
}

impl Initializer for In {
    fn init_animation(self, element: HtmlElement) -> impl AnimationTrait {
        Animation::new(element, self.options)
    }
}

struct Animation {
    element: HtmlElement,
    initial_transition_duration: Option<(String, String)>,
    classes_to_remove_on_enter: Option<Vec<String>>,
    classes_to_remove_on_end: Option<Vec<String>>,
    options: ArcStore<Options>,
}

impl Animation {
    const fn new(element: HtmlElement, options: ArcStore<Options>) -> Self {
        Self {
            element,
            options,
            initial_transition_duration: None,
            classes_to_remove_on_enter: None,
            classes_to_remove_on_end: None,
        }
    }

    fn apply_source_classes(&mut self) {
        self.options.clone().source().with_untracked(|classes| {
            if classes.is_empty() {
                return;
            }

            self.classes_to_remove_on_enter =
                Some(self.element.add_unique_classes(classes));
        });
    }

    fn apply_active_classes(&mut self) {
        self.options.clone().active().with_untracked(|classes| {
            if classes.is_empty() {
                return;
            }

            self.classes_to_remove_on_end =
                Some(self.element.add_unique_classes(classes));
        });
    }

    fn apply_target_classes(&mut self) {
        self.options.clone().target().with_untracked(|classes| {
            if classes.is_empty() {
                return;
            }

            let classes_to_remove = self.element.add_unique_classes(classes);

            if let Some(classes_to_remove_on_end) = &mut self.classes_to_remove_on_end {
                classes_to_remove_on_end.extend(classes_to_remove);
            } else {
                self.classes_to_remove_on_end = Some(classes_to_remove);
            }
        });
    }

    fn remove_classes_on_enter(&mut self) {
        if let Some(classes_to_remove_on_enter) = &self.classes_to_remove_on_enter {
            self.element.remove_classes(classes_to_remove_on_enter);
            self.classes_to_remove_on_enter = None;
        }
    }

    fn remove_classes_on_transition_end(&mut self) {
        let Some(classes_to_remove_on_end) = self.classes_to_remove_on_end.take() else {
            return;
        };

        self.options.clone().duration().with_untracked(|duration| {
            duration.on_transition_end(&self.element, move |element| {
                element.remove_classes(&classes_to_remove_on_end);
            });
        });
    }
}

impl listeners::BeforeEnter for Animation {
    fn listening_for_before_enter(&self) -> bool {
        true
    }

    fn on_before_enter(&mut self) {
        let style = self.element.style();

        self.initial_transition_duration = Some((
            style
                .get_property_value("transition-duration")
                .unwrap_or_default(),
            style.get_property_priority("transition-duration"),
        ));

        _ = style.set_property_with_priority("transition-duration", "0s", "important");

        // add "active" classes first, so they are not later removed along with "source"
        // classes
        self.apply_active_classes();
        self.apply_source_classes();
    }
}

impl listeners::Enter for Animation {
    fn listening_for_enter(&self) -> bool {
        true
    }

    fn enter_delay(&self) -> Duration {
        self.options.read_untracked().delay
    }

    fn on_enter(&mut self) {
        let Some((initial_transition_duration, initial_transition_duration_priority)) =
            &self.initial_transition_duration
        else {
            log_error!("Could not apply entering classes");
            return;
        };

        let style = self.element.style();

        _ = style.set_property_with_priority(
            "transition-duration",
            initial_transition_duration,
            initial_transition_duration_priority,
        );

        self.remove_classes_on_enter();
        self.apply_target_classes();
        self.remove_classes_on_transition_end();
    }
}

impl_empty_animation_listeners!(
    Animation;
    ImmediateEffect,
    Effect,
    Mutation,
    ParentMutation,
    EnterAnimationsFinished,
    MutationAnimationsFinished,
    ParentMutationAnimationsFinished,
    Cleanup
);

impl AnimationTrait for Animation {
    fn enabled(&self) -> bool {
        self.options.read_untracked().enabled
    }
}
