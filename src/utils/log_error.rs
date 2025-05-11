macro_rules! log_error {
    ($($t:tt)*) => {
        leptos::logging::warn!("[{}] {}", env!("CARGO_PKG_NAME"), format_args!($($t)*));
    };
}

pub(crate) use log_error;
