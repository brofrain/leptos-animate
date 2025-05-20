#[cfg(not(web_sys_unstable_apis))]
compile_error!(
    "This crate utilizes unstable web-sys features and requires \
     `--cfg=web_sys_unstable_apis` flag to work"
);

pub mod animation;

mod animate;
pub use animate::animate;

pub mod animations {
    pub mod classes;
    pub mod fade;
    pub mod flip;
    pub mod resize;
    pub mod zombie;
    pub mod zoom;

    // TODO: in / out animations may additionally support `display: none`

    // TODO: more animations
}

pub mod easing;

mod transition_duration;
pub use transition_duration::*;

pub mod utils;
