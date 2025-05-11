use std::time::Duration;

use leptos::web_sys::HtmlElement;
use leptos_animate::{
    animation::{listeners, Animation as AnimationTrait, Initializer},
    easing::elastic_out,
    impl_empty_animation_listeners,
    utils::spawn_animation,
};

#[derive(Clone)]
pub struct RotateIn;

impl Initializer for RotateIn {
    fn init_animation(self, element: HtmlElement) -> impl AnimationTrait {
        Animation { element }
    }
}

struct Animation {
    element: HtmlElement,
}

impl_empty_animation_listeners!(
    Animation;
    Enter,
    ImmediateEffect,
    Effect,
    Mutation,
    ParentMutation,
    EnterAnimationsFinished,
    MutationAnimationsFinished,
    ParentMutationAnimationsFinished,
    Cleanup
);

impl listeners::BeforeEnter for Animation {
    fn listening_for_before_enter(&self) -> bool {
        true
    }

    fn on_before_enter(&mut self) {
        spawn_animation()
            .element(&self.element)
            .keyframe(|t| {
                vec![
                    ("opacity".into(), t.to_string()),
                    (
                        "transform".into(),
                        format!("rotate({}deg)", (1.0 - t) * 180.0),
                    ),
                ]
            })
            .duration(Duration::from_millis(1600))
            .easing(elastic_out)
            .call();
    }
}

impl AnimationTrait for Animation {}
