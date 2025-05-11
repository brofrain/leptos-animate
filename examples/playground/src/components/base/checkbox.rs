use leptos::prelude::*;
use leptos_animate::{animate, animations::fade};
use reactive_stores::Field;

#[component]
pub fn checkbox(#[prop(into)] value: Field<bool>, label: &'static str) -> impl IntoView {
    view! {
        <div
            class="flex items-center gap-2 cursor-pointer group"
            on:click=move |_| {
                value.update(|v| *v = !*v);
            }
        >
            <div>{label}:</div>
            <div class=[
                "w-6 h-6 relative",
                "border border-gray-400 rounded",
                "outline outline-2 -outline-offset-1 outline-transparent group-hover:outline-rose-500",
                "transition-all",
            ]
                .join(" ")>
                <Show when=move || value.get()>
                    <div
                        use:animate=(fade::In::default(), fade::Out::default())
                        class="absolute inset-1 bg-gray-300 rounded-sm"
                    ></div>
                </Show>
            </div>
        </div>
    }
}
