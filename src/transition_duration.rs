use std::{future::Future, pin::Pin, rc::Rc, time::Duration};

use leptos::task::spawn_local;
use send_wrapper::SendWrapper;
use wasm_bindgen::prelude::Closure;
use web_sys::HtmlElement;

use crate::utils::{add_oneshot_event_listener, sleep, OnAnimationsFinishedExt};

type FutureCb = Rc<dyn Fn() -> Pin<Box<dyn Future<Output = ()> + Unpin>>>;

/// Defines when a transition should be considered finished:
/// - (default) on "transitionend" event
/// - on "animationend" event
/// - after a fixed duration
/// - after a custom future is resolved
/// - on the first "finish" event of the element's animations
#[derive(Default, Clone)]
pub enum TransitionDuration {
    Fixed(Duration),
    AnimationEnd,
    #[default]
    TransitionEnd,
    Future(SendWrapper<FutureCb>),
    AnimationsFinished,
}

impl std::fmt::Debug for TransitionDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fixed(duration) => write!(f, "Fixed({duration:?})"),
            Self::AnimationEnd => write!(f, "AnimationEnd"),
            Self::TransitionEnd => write!(f, "TransitionEnd"),
            Self::Future(_) => write!(f, "Future"),
            Self::AnimationsFinished => write!(f, "AnimationsFinished"),
        }
    }
}

impl TransitionDuration {
    pub(crate) fn on_transition_end(
        &self,
        element: &HtmlElement,
        cb: impl Fn(&HtmlElement) + 'static,
    ) {
        fn build_closure(
            element: &HtmlElement,
            cb: impl Fn(&HtmlElement) + 'static,
        ) -> Closure<dyn Fn()> {
            Closure::new({
                let element = element.clone();
                move || cb(&element)
            })
        }

        fn set_closure(
            element: &HtmlElement,
            event_type: &str,
            cb: impl Fn(&HtmlElement) + 'static,
        ) {
            let closure = build_closure(element, cb);
            add_oneshot_event_listener(element, event_type, &closure);
            closure.forget();
        }

        match self {
            Self::AnimationEnd => set_closure(element, "animationend", cb),
            Self::TransitionEnd => {
                set_closure(element, "transitionend", cb);
            }
            Self::Fixed(duration) => {
                let element = element.clone();
                let duration = *duration;
                spawn_local(async move {
                    sleep(duration).await;
                    cb(&element);
                });
            }
            Self::Future(f) => {
                let element = element.clone();
                let f = Rc::clone(f);
                spawn_local(async move {
                    f().await;
                    cb(&element);
                });
            }
            Self::AnimationsFinished => {
                element.on_animations_finished(
                    {
                        let element = element.clone();
                        move || cb(&element)
                    },
                    true,
                );
            }
        }
    }
}

impl From<Duration> for TransitionDuration {
    fn from(duration: Duration) -> Self {
        Self::Fixed(duration)
    }
}

impl<F, Fut> From<F> for TransitionDuration
where
    F: Fn() -> Fut + 'static,
    Fut: Future<Output = ()> + Unpin + 'static,
{
    fn from(f: F) -> Self {
        Self::Future(SendWrapper::new(Rc::new(move || Box::pin(f()))))
    }
}
