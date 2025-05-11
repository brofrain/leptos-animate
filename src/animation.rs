use std::{cell::RefCell, rc::Rc};

use web_sys::HtmlElement;

pub trait Initializer: Clone {
    fn init_animation(self, element: HtmlElement) -> impl Animation;
}

pub mod listeners;

pub trait Animation:
    listeners::BeforeEnter
    + listeners::Enter
    + listeners::ImmediateEffect
    + listeners::Effect
    + listeners::Mutation
    + listeners::ParentMutation
    + listeners::EnterAnimationsFinished
    + listeners::MutationAnimationsFinished
    + listeners::ParentMutationAnimationsFinished
    + listeners::Cleanup
    + 'static
{
    fn enabled(&self) -> bool {
        true
    }

    fn track(&self) {}
}

mod aggregated;
use aggregated::Aggregated;

mod ext;
pub use ext::Ext;

macro_rules! impl_animation_initializer {
    ($($idx:tt $t:tt),+) => {
        impl<$($t,)+> Initializer for ($($t,)+)
        where
            $($t: Initializer),+
        {
            fn init_animation(self, element: HtmlElement) -> impl Animation {
                Aggregated::new(
                    vec![
                        $(
                            Rc::new(
                                RefCell::new(self.$idx.init_animation(element.clone()))
                            )
                        ),+
                    ],
                    element
                )
            }
        }
    };
}

impl_animation_initializer!(0 A);
impl_animation_initializer!(0 A, 1 B);
impl_animation_initializer!(0 A, 1 B, 2 C);
impl_animation_initializer!(0 A, 1 B, 2 C, 3 D);
impl_animation_initializer!(0 A, 1 B, 2 C, 3 D, 4 E);
impl_animation_initializer!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F);
impl_animation_initializer!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G);
impl_animation_initializer!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G, 7 H);

#[macro_export]
macro_rules! impl_empty_animation_listeners {
    ($animation:tt; $($listener:tt),+) => {
        $(
            impl $crate::animation::listeners::$listener for $animation {}
        )+
    };
}
