use std::{
    cell::{Ref, RefCell},
    collections::HashSet,
    rc::Rc,
};

use web_sys::{HtmlElement, MutationRecord};

use super::{listeners, Animation, Ext};

fn collect_indexes(
    animations: &[Rc<RefCell<dyn Animation>>],
    f: impl Fn(Ref<'_, dyn Animation>) -> bool,
) -> HashSet<usize> {
    animations
        .iter()
        .enumerate()
        .filter_map(|(i, m)| if f(m.borrow()) { Some(i) } else { None })
        .collect()
}

macro_rules! aggregated_struct {
    ($($listener_name:ident),+) => {
        paste::paste! {
            pub struct Aggregated {
                element: HtmlElement,
                animations: Vec<Rc<RefCell<dyn Animation>>>,
                idx_tracked: HashSet<usize>,
                $([< idx_listening_for_ $listener_name >]: HashSet<usize>,)*
            }

            impl Aggregated {
                pub fn new(
                    animations: Vec<Rc<RefCell<dyn Animation>>>,
                    element: HtmlElement,
                ) -> Self {
                    $(
                        let [< idx_listening_for_ $listener_name >] = collect_indexes(
                            &animations,
                            |m| m. [< listening_for_ $listener_name >]()
                        );
                    )*

                    Self {
                        element,
                        animations,
                        idx_tracked: idx_listening_for_immediate_effect
                            .union(&idx_listening_for_effect)
                            .copied()
                            .collect(),
                        $([< idx_listening_for_ $listener_name >],)*
                    }
                }
            }
        }
    }
}

aggregated_struct! {
    before_enter,
    enter,
    immediate_effect,
    effect,
    mutation,
    parent_mutation,
    enter_animations_finished,
    mutation_animations_finished,
    parent_mutation_animations_finished,
    cleanup
}

macro_rules! impl_aggregated {
    ($($listener:ident),+) => {
        paste::paste! {
            $(
                impl listeners::$listener for Aggregated {
                    fn [< listening_for_ $listener:snake >](&self) -> bool {
                        !self.[< idx_listening_for_ $listener:snake >].is_empty()
                    }

                    fn [< on_ $listener:snake >](&mut self) {
                        self.each_listening(&self.[< idx_listening_for_ $listener:snake >], |m| {
                            m.borrow_mut().[< on_ $listener:snake >]();
                        });
                    }
                }
            )*
        }
    };
}

impl_aggregated! {
    BeforeEnter,
    ImmediateEffect,
    EnterAnimationsFinished,
    MutationAnimationsFinished,
    ParentMutationAnimationsFinished,
    Cleanup
}

impl Aggregated {
    fn each_listening(
        &self,
        indexes: &HashSet<usize>,
        f: impl Fn(&Rc<RefCell<dyn Animation>>),
    ) {
        for (i, m) in self.animations.iter().enumerate() {
            if !indexes.contains(&i) || !m.borrow().enabled() {
                continue;
            }

            f(m);
        }
    }
}

impl listeners::Enter for Aggregated {
    fn listening_for_enter(&self) -> bool {
        !self.idx_listening_for_enter.is_empty()
    }

    fn on_enter(&mut self) {
        self.each_listening(&self.idx_listening_for_enter, |m| {
            m.delayed_on_enter(&self.element);
        });
    }
}

impl listeners::Effect for Aggregated {
    fn listening_for_effect(&self) -> bool {
        !self.idx_listening_for_effect.is_empty()
    }

    fn on_effect(&mut self) {
        self.each_listening(&self.idx_listening_for_effect, |m| {
            m.delayed_on_effect(&self.element);
        });
    }
}

impl listeners::Mutation for Aggregated {
    fn listening_for_mutation(&self) -> bool {
        !self.idx_listening_for_mutation.is_empty()
    }

    fn on_mutation(&mut self, mutations: &[MutationRecord]) {
        self.each_listening(&self.idx_listening_for_mutation, |m| {
            m.borrow_mut().on_mutation(mutations);
        });
    }
}

impl listeners::ParentMutation for Aggregated {
    fn listening_for_parent_mutation(&self) -> bool {
        !self.idx_listening_for_parent_mutation.is_empty()
    }

    fn on_parent_mutation(&mut self, mutations: &[MutationRecord]) {
        self.each_listening(&self.idx_listening_for_parent_mutation, |m| {
            m.borrow_mut().on_parent_mutation(mutations);
        });
    }
}

impl Animation for Aggregated {
    fn enabled(&self) -> bool {
        self.animations.iter().any(|m| m.borrow().enabled())
    }

    fn track(&self) {
        for (i, m) in self.animations.iter().enumerate() {
            if self.idx_tracked.contains(&i) {
                m.borrow().track();
            }
        }
    }
}
