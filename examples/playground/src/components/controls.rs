use std::time::Duration;

use leptos::prelude::*;
use leptos_animate::{animate, animations::classes};
use reactive_stores::{Field, Store};

use crate::{
    components::base::{Button, Checkbox, Input},
    config::{self, Config, ConfigStoreFields, TransitionStoreFields},
    state::{State, Utils},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ConfigTab {
    FadeIn,
    FadeOut,
    ZoomIn,
    ZoomOut,
    Flip,
    Resize,
    Misc,
}

fn create_field_usize_slice<T>(
    field: impl Into<Field<T>>,
    getter: impl Fn(T) -> usize,
    setter: impl Fn(usize) -> T + 'static,
) -> RwSignal<usize>
where
    T: Clone + 'static,
{
    let field = field.into();
    let signal = RwSignal::<usize>::new(getter(field.get_untracked()));

    Effect::watch(
        move || signal.get(),
        move |v, _, _| {
            *field.write() = setter(*v);
        },
        false,
    );

    signal
}

fn create_field_duration_to_usize_slice(
    field: impl Into<Field<Duration>>,
) -> RwSignal<usize> {
    create_field_usize_slice(
        field,
        |duration| duration.as_millis() as usize,
        |ms| Duration::from_millis(ms as u64),
    )
}

#[component]
fn card(children: Children) -> impl IntoView {
    view! {
        <div
            class="flex flex-wrap gap-2 justify-center"
            use:animate=(
                classes::In::default()
                    .source("opacity-0")
                    .active("duration-[250ms]")
                    .delay(Duration::from_millis(200)),
                classes::Out::default().target("opacity-0").active("duration-[250ms]"),
            )
        >
            {children()}
        </div>
    }
}

fn transition_tab(config: impl Into<Field<config::Transition>>) -> AnyView {
    let config = config.into();
    let duration_ms = create_field_duration_to_usize_slice(config.duration());
    let delay_ms = create_field_duration_to_usize_slice(config.delay());

    view! {
        <Card>
            <Input value=duration_ms label="Duration" />
            <Input value=delay_ms label="Delay" />
            <Checkbox value=config.enabled() label="Enabled" />
        </Card>
    }
    .into_any()
}

#[component]
fn misc(config: Store<Config>, state: Store<State>) -> impl IntoView {
    Effect::watch(
        move || config.rng_seed().get(),
        move |seed, _, _| {
            state.reset_rng(*seed as u64);
        },
        false,
    );

    view! {
        <Card>
            <Input value=config.rng_seed() label="RNG seed" />
        </Card>
    }
}

#[component]
fn tab_button(
    tab: ConfigTab,
    label: &'static str,
    current_tab: RwSignal<ConfigTab>,
) -> impl IntoView {
    view! {
        <button
            class=move || {
                [
                    "py-1 px-2 rounded hover:bg-rose-500 transition-all",
                    if current_tab.get() == tab { "bg-gray-100/20" } else { "" },
                ]
                    .join(" ")
            }
            on:click=move |_| {
                current_tab.set(tab);
            }
        >
            {label}
        </button>
    }
}

#[component]
pub fn controls(config: Store<Config>, state: Store<State>) -> impl IntoView {
    let config_tab = RwSignal::new(ConfigTab::Flip);
    let config_tab_memo = Memo::new(move |_| config_tab.get());

    let config_view = move || match config_tab_memo.get() {
        ConfigTab::Flip => transition_tab(config.flip()),
        ConfigTab::Resize => transition_tab(config.resize()),
        ConfigTab::FadeIn => transition_tab(config.fade_in()),
        ConfigTab::FadeOut => transition_tab(config.fade_out()),
        ConfigTab::ZoomIn => transition_tab(config.zoom_in()),
        ConfigTab::ZoomOut => transition_tab(config.zoom_out()),
        ConfigTab::Misc => view! { <Misc config state /> }.into_any(),
    };

    view! {
        <div class="flex flex-wrap gap-1 justify-center">
            <Button on:click=move |_| {
                state.add_item_at_start();
            }>{"Add start"}</Button>

            <Button on:click=move |_| {
                state.add_item_at_end();
            }>{"Add end"}</Button>

            <Button on:click=move |_| {
                state.add_item_at_random();
            }>{"Add random"}</Button>

            <Button on:click=move |_| {
                state.shuffle_items();
            }>{"Shuffle"}</Button>
        </div>

        <div class="flex justify-center mt-1">
            <div class="flex flex-col gap-1 p-1 border-gray-400/20 border rounded">
                <div class="flex flex-wrap justify-center gap-1">
                    <TabButton tab=ConfigTab::Flip label="Flip" current_tab=config_tab />
                    <TabButton tab=ConfigTab::Resize label="Resize" current_tab=config_tab />
                    <TabButton tab=ConfigTab::FadeIn label="Fade-in" current_tab=config_tab />
                    <TabButton tab=ConfigTab::FadeOut label="Fade-out" current_tab=config_tab />
                    <TabButton tab=ConfigTab::ZoomIn label="Zoom-in" current_tab=config_tab />
                    <TabButton tab=ConfigTab::ZoomOut label="Zoom-out" current_tab=config_tab />
                    <TabButton tab=ConfigTab::Misc label="Misc" current_tab=config_tab />
                </div>

                {config_view}
            </div>
        </div>
    }
}
