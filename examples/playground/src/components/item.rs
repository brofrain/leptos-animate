use leptos::prelude::*;
use reactive_stores::Field;

use crate::item::{Color, Item as ItemData, ItemStoreFields, Size};

#[component]
fn plus_icon() -> impl IntoView {
    view! {
        <svg
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
        >
            <line x1="12" y1="5" x2="12" y2="19" />
            <line x1="5" y1="12" x2="19" y2="12" />
        </svg>
    }
}

#[component]
fn minus_icon() -> impl IntoView {
    view! {
        <svg
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
        >
            <line x1="5" y1="12" x2="19" y2="12" />
        </svg>
    }
}

#[component]
fn trash_icon() -> impl IntoView {
    view! {
        <svg
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
        >
            <polyline points="3 6 5 6 21 6" />
            <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2" />
            <line x1="10" y1="11" x2="10" y2="17" />
            <line x1="14" y1="11" x2="14" y2="17" />
        </svg>
    }
}

#[component]
fn button(children: Children) -> impl IntoView {
    view! {
        <button class="bg-black/60 text-white rounded-full flex items-center justify-center h-5 w-5">
            {children()}
        </button>
    }
}

#[component]
fn container(#[prop(into)] size: Field<Size>, children: Children) -> impl IntoView {
    let size_classes = Memo::new(move |_| match *size.read() {
        Size::Small => "w-[4rem] h-[4rem]",
        Size::Medium => "w-[4.5rem] h-[4.5rem]",
        Size::Large => "w-[5rem] h-[5rem]",
    });

    view! {
        <div class=move || {
            ["group relative [&>*]:absolute [&>*]:inset-0", size_classes.get()].join(" ")
        }>{children()}</div>
    }
}

#[component]
pub fn item(
    #[prop(into)] item: Field<ItemData>,
    #[prop(into)] remove: Callback<()>,
) -> impl IntoView {
    let size = item.size();
    let shape = item.shape();

    let color_classes = Memo::new(move |_| match *item.color().read() {
        Color::Red => "text-red-500",
        Color::Green => "text-green-500",
        Color::Blue => "text-blue-500",
    });

    view! {
        <Container size=size>
            <div class=move || {
                ["flex justify-center items-center [&>*]:h-full [&>*]:w-full", color_classes.get()]
                    .join(" ")
            }>{move || shape.read().to_view()}</div>

            <div class="flex group-hover:hidden items-center justify-center text-white text-sm font-bold">
                {move || item.id().get()}
            </div>

            <div class="hidden group-hover:flex flex-col items-center justify-center">
                <div class="flex justify-center gap-1">
                    <Button on:click=move |_| size.write().increase()>
                        <PlusIcon />
                    </Button>
                    <Button on:click=move |_| size.write().decrease()>
                        <MinusIcon />
                    </Button>
                </div>
                <Button on:click=move |_| remove.run(())>
                    <TrashIcon />
                </Button>
            </div>
        </Container>
    }
}
