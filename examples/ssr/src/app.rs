use button::Button;
use leptos::prelude::*;
use leptos_animate::{animate, animations::flip::Flip};
use leptos_meta::{provide_meta_context, MetaTags};
use leptos_router::{
    components::{FlatRoutes, Route, Router},
    StaticSegment,
};
use rand::{rng, seq::SliceRandom, Rng};
use rotate_in::RotateIn;

mod button;
mod rotate_in;

const INITIAL_ITEM_COUNT: usize = 40;

#[component]
fn home() -> impl IntoView {
    let next_item = StoredValue::new(INITIAL_ITEM_COUNT);
    let items = RwSignal::new((0..INITIAL_ITEM_COUNT).collect::<Vec<_>>());

    let create_new_item = Callback::new(move |()| {
        let item = next_item.get_value();
        next_item.set_value(item + 1);
        item
    });

    let rng = StoredValue::new_local(rng());

    let shuffle = move |_| {
        items.write().shuffle(&mut rng.write_value());
    };

    let add_start = move |_| {
        items.write().insert(0, create_new_item.run(()));
    };

    let add_end = move |_| {
        items.write().push(create_new_item.run(()));
    };

    let add_random = move |_| {
        let mut items = items.write();
        let len = items.len();
        items.insert(
            rng.write_value().random_range(0..len),
            create_new_item.run(()),
        );
    };

    let remove_start = move |_| {
        let mut items = items.write();

        if !items.is_empty() {
            items.remove(0);
        }
    };

    let remove_end = move |_| {
        items.write().pop();
    };

    let remove_random = move |_| {
        let mut items = items.write();
        let len = items.len();

        if !items.is_empty() {
            items.remove(rng.write_value().random_range(0..len));
        }
    };

    view! {
        <main class="min-h-screen py-8 bg-gray-800 font-bold text-white">
            <div class="flex flex-wrap gap-2 justify-center">
                <Button on:click=add_start>{"Add start"}</Button>
                <Button on:click=add_end>{"Add end"}</Button>
                <Button on:click=add_random>{"Add random"}</Button>
                <Button on:click=remove_start>{"Remove start"}</Button>
                <Button on:click=remove_end>{"Remove end"}</Button>
                <Button on:click=remove_random>{"Remove random"}</Button>
                <Button on:click=shuffle>{"Shuffle"}</Button>
            </div>

            <div class="mt-4 grid grid-cols-[repeat(5,auto)] gap-2 justify-center text-xl [&>*]:w-15 [&>*]:h-15">
                <For each=move || items.get() key=|item| *item let:item>
                    <div
                        class="rounded flex items-center justify-center bg-blue-500/70"
                        use:animate=(Flip::watch(items), RotateIn)
                    >
                        {item}
                    </div>
                </For>
            </div>
        </main>
    }
}

#[component]
pub fn app() -> impl IntoView {
    provide_meta_context();
    view! {
        <Router>
            <FlatRoutes fallback=|| "Page not found.">
                <Route path=StaticSegment("") view=Home />
            </FlatRoutes>
        </Router>
    }
}

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <link rel="stylesheet" id="leptos" href="/pkg/ssr-example.css" />
                <link rel="shortcut icon" type="image/ico" href="/favicon.ico" />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}
