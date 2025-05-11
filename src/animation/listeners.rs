use std::time::Duration;

use web_sys::MutationRecord;

pub trait BeforeEnter {
    fn on_before_enter(&mut self) {}

    fn listening_for_before_enter(&self) -> bool {
        false
    }
}

pub trait Enter {
    fn on_enter(&mut self) {}

    fn enter_delay(&self) -> Duration {
        Duration::ZERO
    }

    fn listening_for_enter(&self) -> bool {
        false
    }
}

pub trait ImmediateEffect {
    fn on_immediate_effect(&mut self) {}

    fn listening_for_immediate_effect(&self) -> bool {
        false
    }
}

pub trait Effect {
    fn on_effect(&mut self) {}

    fn effect_delay(&self) -> Duration {
        Duration::ZERO
    }

    fn listening_for_effect(&self) -> bool {
        false
    }
}

pub trait Mutation {
    fn on_mutation(&mut self, _mutations: &[MutationRecord]) {}

    fn listening_for_mutation(&self) -> bool {
        false
    }
}

pub trait ParentMutation {
    fn on_parent_mutation(&mut self, _mutations: &[MutationRecord]) {}

    fn listening_for_parent_mutation(&self) -> bool {
        false
    }
}

pub trait EnterAnimationsFinished {
    fn on_enter_animations_finished(&mut self) {}

    fn listening_for_enter_animations_finished(&self) -> bool {
        false
    }
}

pub trait MutationAnimationsFinished {
    fn on_mutation_animations_finished(&mut self) {}

    fn listening_for_mutation_animations_finished(&self) -> bool {
        false
    }
}

pub trait ParentMutationAnimationsFinished {
    fn on_parent_mutation_animations_finished(&mut self) {}

    fn listening_for_parent_mutation_animations_finished(&self) -> bool {
        false
    }
}

pub trait Cleanup {
    fn on_cleanup(&mut self) {}

    fn listening_for_cleanup(&self) -> bool {
        false
    }
}
