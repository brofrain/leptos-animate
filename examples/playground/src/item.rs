use std::sync::atomic::{AtomicUsize, Ordering};

use leptos::{
    prelude::{CustomAttribute, ElementChild, IntoAny},
    view,
    IntoView,
};
use rand::RngCore;
use reactive_stores_macro::Store;

pub trait Random {
    fn random(rng: &mut impl RngCore) -> Self;
}

#[derive(PartialEq, Eq, Debug)]
pub enum Shape {
    Circle,
    Square,
    Triangle,
    Hexagon,
}

impl Random for Shape {
    fn random(rng: &mut impl RngCore) -> Self {
        match rng.next_u32() % 4 {
            0 => Self::Circle,
            1 => Self::Square,
            2 => Self::Triangle,
            _ => Self::Hexagon,
        }
    }
}

impl Shape {
    pub fn to_view(&self) -> impl IntoView {
        match self {
            Self::Circle => view! {
                <svg
                    width="24"
                    height="24"
                    viewBox="0 0 24 24"
                    stroke-width="2"
                    stroke="currentColor"
                    fill="none"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                >
                    <path stroke="none" d="M0 0h24v24H0z" />
                    <circle cx="12" cy="12" r="9" />
                </svg>
            }.into_any(),

            Self::Square => view! {
                <svg
                    width="24"
                    height="24"
                    viewBox="0 0 24 24"
                    stroke-width="2"
                    stroke="currentColor"
                    fill="none"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                >
                    <path stroke="none" d="M0 0h24v24H0z" />
                    <rect x="4" y="4" width="16" height="16" rx="2" />
                </svg>
            }.into_any(),

            Self::Triangle => view! {
                <svg
                    width="24"
                    height="24"
                    viewBox="0 0 24 24"
                    stroke-width="2"
                    stroke="currentColor"
                    fill="none"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                >
                    <path stroke="none" d="M0 0h24v24H0z" />
                    <path d="M5.07 19H19a2 2 0 0 0 1.75 -2.75L13.75 4a2 2 0 0 0 -3.5 0L3.25 16.25a2 2 0 0 0 1.75 2.75" />
                </svg>
            }.into_any(),

            Self::Hexagon => view! {
                <svg
                    width="24"
                    height="24"
                    viewBox="0 0 24 24"
                    stroke-width="2"
                    stroke="currentColor"
                    fill="none"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                >
                    <path stroke="none" d="M0 0h24v24H0z" />
                    <path d="M12.971 3.54l6 3.333A2 2 0 0120 8.62v6.536a2 2 0 0 1 -1.029 1.748l-6 3.333a2 2 0 0 1 -1.942 0l-6-3.333A2 2 0 014 15.157V8.62a2 2 0 0 1 1.029 -1.748l6-3.333a2 2 0 0 1 1.942 0z" />
                </svg>
            }.into_any(),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum Color {
    Red,
    Green,
    Blue,
}

impl Random for Color {
    fn random(rng: &mut impl RngCore) -> Self {
        match rng.next_u32() % 3 {
            0 => Self::Red,
            1 => Self::Green,
            _ => Self::Blue,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum Size {
    Small,
    Medium,
    Large,
}

impl Size {
    pub const fn increase(&mut self) {
        *self = match self {
            Self::Small => Self::Medium,
            Self::Medium => Self::Large,
            Self::Large => {
                return;
            }
        };
    }

    pub const fn decrease(&mut self) {
        *self = match self {
            Self::Small => {
                return;
            }
            Self::Medium => Self::Small,
            Self::Large => Self::Medium,
        };
    }
}

impl Random for Size {
    fn random(rng: &mut impl RngCore) -> Self {
        match rng.next_u32() % 10 {
            0..5 => Self::Small,
            5..8 => Self::Medium,
            _ => Self::Large,
        }
    }
}

#[derive(Store, Debug)]
pub struct Item {
    pub id: usize,
    pub shape: Shape,
    pub color: Color,
    pub size: Size,
}

impl Random for Item {
    fn random(rng: &mut impl RngCore) -> Self {
        static LAST_ID: AtomicUsize = AtomicUsize::new(0);
        Self {
            id: LAST_ID.fetch_add(1, Ordering::AcqRel),
            shape: Shape::random(rng),
            color: Color::random(rng),
            size: Size::random(rng),
        }
    }
}
