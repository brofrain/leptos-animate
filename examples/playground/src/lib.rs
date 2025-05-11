#![expect(clippy::must_use_candidate)]

use config::{Config, ConfigStoreFields, TransitionStoreFields};
use item::ItemStoreFields;
use leptos::prelude::*;
use leptos_animate::{
    animate,
    animations::{fade, flip::Flip, resize::Resize, zoom},
};

mod components {
    pub mod base {
        mod button;
        mod checkbox;
        mod input;
        pub use button::Button;
        pub use checkbox::Checkbox;
        pub use input::Input;
    }

    mod controls;
    pub use controls::Controls;

    mod item;
    pub use item::Item;
}

mod config;
mod item;
mod state;
mod store;

use components::{Controls, Item};
use reactive_stores::Store;
use state::{State, StateStoreFields, Utils};

#[component]
pub fn app() -> impl IntoView {
    let config = Store::new(Config::default());
    let state = Store::new(State::new(config.rng_seed().get_untracked() as u64));

    view! {
        <main class="min-h-screen py-8 bg-[#19191d] text-gray-100">
            <Controls config state />

            <div class="mt-4 grid grid-cols-[repeat(3,auto)] md:grid-cols-[repeat(4,auto)] justify-center items-center justify-items-center">
                <For each=move || state.items() key=|item| item.id().get() let:item>
                    <Item
                        item
                        remove={
                            let id = item.id().get_untracked();
                            move || {
                                state.remove_item_by_id(id);
                            }
                        }
                        use:animate=(
                            {
                                let config = config.flip();
                                Flip::watch(state.items())
                                    .delay_signal(config.delay())
                                    .duration_signal(
                                        Signal::derive(move || config.duration().get().into()),
                                    )
                                    .easing_signal(config.easing())
                                    .enabled_signal(config.enabled())
                            },
                            {
                                let config = config.resize();
                                Resize::watch(state.items())
                                    .delay_signal(config.delay())
                                    .duration_signal(
                                        Signal::derive(move || config.duration().get().into()),
                                    )
                                    .easing_signal(config.easing())
                                    .enabled_signal(config.enabled())
                            },
                            {
                                let config = config.fade_in();
                                fade::In::default()
                                    .delay_signal(config.delay())
                                    .easing_signal(config.easing())
                                    .duration_signal(config.duration())
                                    .enabled_signal(config.enabled())
                            },
                            {
                                let config = config.fade_out();
                                fade::Out::default()
                                    .delay_signal(config.delay())
                                    .easing_signal(config.easing())
                                    .duration_signal(config.duration())
                                    .enabled_signal(config.enabled())
                            },
                            {
                                let config = config.zoom_in();
                                zoom::In::default()
                                    .delay_signal(config.delay())
                                    .easing_signal(config.easing())
                                    .duration_signal(config.duration())
                                    .enabled_signal(config.enabled())
                            },
                            {
                                let config = config.zoom_out();
                                zoom::Out::default()
                                    .delay_signal(config.delay())
                                    .easing_signal(config.easing())
                                    .duration_signal(config.duration())
                                    .enabled_signal(config.enabled())
                            },
                        )
                    />
                </For>
            </div>
        </main>
    }
}
