use leptos::prelude::*;

#[component]
pub fn button(children: Children) -> impl IntoView {
    view! {
        <button class=[
            "rounded border border-gray-400 hover:border-rose-500",
            "bg-transparent hover:bg-rose-500",
            "transition-all",
            "px-2 py-1",
        ]
            .join(" ")>{children()}</button>
    }
}
