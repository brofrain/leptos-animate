use std::time::Duration;

use futures::channel;
use leptos::prelude::set_timeout;

pub async fn sleep(duration: Duration) {
    let (tx, rx) = channel::oneshot::channel();

    set_timeout(
        move || {
            _ = tx.send(());
        },
        duration,
    );

    rx.await.unwrap();
}
