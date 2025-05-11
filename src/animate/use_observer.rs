use leptos::prelude::on_cleanup;
use send_wrapper::SendWrapper;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{
    self,
    js_sys::Array,
    HtmlElement,
    MutationObserver,
    MutationObserverInit,
    MutationRecord,
};

use super::initial::Initial;

pub fn use_observer(element: &HtmlElement, cb: impl Fn(Vec<MutationRecord>) + 'static) {
    let initial = Initial::new();

    let closure = Closure::<dyn Fn(Array)>::new(move |mutations: Array| {
        if initial.get() {
            return;
        }

        let mutations = mutations
            .to_vec()
            .into_iter()
            .map(JsCast::unchecked_into)
            .collect::<Vec<_>>();

        cb(mutations);
    });

    let observer = MutationObserver::new(closure.as_ref().unchecked_ref()).unwrap();
    closure.forget();

    let options = MutationObserverInit::new();
    options.set_attributes(true);
    options.set_child_list(true);
    options.set_subtree(true);
    observer.observe_with_options(element, &options).unwrap();

    let observer = SendWrapper::new(observer);
    on_cleanup(move || {
        observer.disconnect();
    });
}
