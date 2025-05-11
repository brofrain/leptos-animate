use leptos::prelude::*;

#[component]
pub fn button(children: Children) -> impl IntoView {
    view! {
        <button class="rounded px-4 py-1 bg-purple-500 hover:bg-purple-600 transition cursor-pointer select-none">
            {children()}
        </button>
    }
}
