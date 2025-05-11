use std::{cell::Cell, rc::Rc, time::Duration};

use leptos::prelude::{GetUntracked, WithUntracked};
use reactive_stores::ArcStore;
use web_sys::HtmlElement;

use super::html_element_ext::HtmlElementExt;
use crate::{
    animation::{Animation, Initializer},
    animations::zombie::Zombie,
    utils::define_options,
    TransitionDuration,
};

define_options! {
    Out.options;
    @with_setters
    duration: TransitionDuration = TransitionDuration::default(),
    delay: Duration = Duration::ZERO,
    enabled: bool = true,
    source: String = String::new(),
    target: String = String::new(),
    active: String = String::new()
}

/// When the element is removed from the DOM, it is reinserted with CSS classes
/// powering its leaving animation and then removed again:
/// - `source` - initial classes added before the element is reinserted and
///   removed one frame after
/// - `active` - applied during the entire leaving phase and removed once the
///   animation is finished.
/// - `target` - classed added one frame after the element is inserted and
///   removed once the animation is finished.
///
/// Example fade-out animation utilizing Tailwind CSS:
/// ```no_run
/// view! {
///     <div
///         use:animate=Out::default()
///             .source("opacity-100")
///             .active("duration-150")
///             .target("opacity-0")
///     >
///         // ...
///     </div>
/// }
/// ```
#[must_use]
#[derive(Clone, Default)]
pub struct Out {
    options: ArcStore<Options>,
}

fn unwrap_cell<T>(cell: Rc<Cell<T>>) -> T {
    Rc::try_unwrap(cell)
        .map_err(|_| ())
        .expect("The cell should have only one strong reference")
        .into_inner()
}

impl Initializer for Out {
    fn init_animation(self, element: HtmlElement) -> impl Animation {
        let initial_transition_duration = Rc::new(Cell::new(String::new()));
        let initial_transition_duration_priority = Rc::new(Cell::new(String::new()));
        let classes_to_remove_on_enter = Rc::new(Cell::new(None::<Vec<String>>));

        let options = self.options;

        let apply_source_classes = {
            let classes_to_remove_on_enter = Rc::clone(&classes_to_remove_on_enter);
            let options = options.clone();
            move |element: &HtmlElement| {
                options.clone().source().with_untracked(|classes| {
                    if classes.is_empty() {
                        return;
                    }

                    classes_to_remove_on_enter
                        .set(Some(element.add_unique_classes(classes)));
                });

                options
                    .clone()
                    .active()
                    .with_untracked(|classes| element.add_unique_classes(classes));
            }
        };

        Zombie::default()
            .enabled_signal(options.clone().enabled())
            .delay_signal(options.clone().delay())
            .duration_signal(options.clone().duration())
            .before_enter({
                let initial_transition_duration = Rc::clone(&initial_transition_duration);
                let initial_transition_duration_priority =
                    Rc::clone(&initial_transition_duration_priority);

                move |element: &HtmlElement| {
                    let style = element.style();

                    initial_transition_duration.set(
                        style
                            .get_property_value("transition-duration")
                            .unwrap_or_default(),
                    );

                    initial_transition_duration_priority
                        .set(style.get_property_priority("transition-duration"));

                    apply_source_classes(element);
                }
            })
            .enter(move |element: &HtmlElement| {
                let style = element.style();

                _ = style.set_property_with_priority(
                    "transition-duration",
                    &unwrap_cell(initial_transition_duration),
                    &unwrap_cell(initial_transition_duration_priority),
                );

                if let Some(classes) = unwrap_cell(classes_to_remove_on_enter).as_ref() {
                    element.remove_classes(classes);
                }

                element.add_unique_classes(&options.target().get_untracked());
            })
            .init_animation(element)
    }
}
