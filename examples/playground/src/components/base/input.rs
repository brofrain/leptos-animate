use leptos::prelude::*;

#[component]
pub fn input(
    value: impl Get<Value = usize>
        + WithUntracked<Value = usize>
        + Write<Value = usize>
        + 'static,
    label: &'static str,
) -> impl IntoView {
    let valid = RwSignal::new(true);

    view! {
        <label class="flex gap-2">
            {label}:
            <input
                class=move || {
                    [
                        "w-20 m-0",
                        "text-center",
                        "bg-transparent",
                        "rounded border border-gray-400",
                        "outline outline-2 -outline-offset-1 outline-transparent",
                        "hover:outline-rose-500 focus:outline-rose-600",
                        "transition-all",
                        if valid.get() { "" } else { "border-red-500" },
                    ]
                        .join(" ")
                }
                type="number"
                prop:value=value.with_untracked(ToString::to_string)
                on:input=move |ev| {
                    let new_value_string = event_target_value(&ev);
                    if let Ok(new_value) = new_value_string.parse() {
                        *value.write() = new_value;
                        valid.set(true);
                    } else {
                        valid.set(false);
                    }
                }
            />
        </label>
    }
}
