use futures::channel;
use leptos::prelude::request_animation_frame;

pub async fn animation_frame() {
    let (tx, rx) = channel::oneshot::channel();

    request_animation_frame(move || {
        request_animation_frame(move || {
            tx.send(()).unwrap();
        });
    });

    rx.await.unwrap();
}
