use std::f64::consts::PI;

pub type Easing = fn(f64) -> f64;

// adopted from https://github.com/mattdesl/eases

pub const fn linear(t: f64) -> f64 {
    t
}

pub fn back_in_out(t: f64) -> f64 {
    let s = 1.70158 * 1.525;
    if t < 0.5 {
        0.5 * (t * t * ((s + 1.0) * t - s))
    } else {
        0.5 * ((t - 2.0) * t * ((s + 1.0) * t + s) + 2.0)
    }
}

pub fn back_in(t: f64) -> f64 {
    let s = 1.70158;
    t * t * ((s + 1.0) * t - s)
}

pub fn back_out(t: f64) -> f64 {
    let s = 1.70158;
    (t - 1.0) * (t - 1.0) * ((s + 1.0) * (t - 1.0) + s) + 1.0
}

pub fn bounce_out(t: f64) -> f64 {
    let a = 4.0 / 11.0;
    let b = 8.0 / 11.0;
    let c = 9.0 / 10.0;
    let ca = 4356.0 / 361.0;
    let cb = 35442.0 / 1805.0;
    let cc = 16061.0 / 1805.0;
    let t2 = t * t;
    if t < a {
        7.5625 * t2
    } else if t < b {
        9.075 * t2 - 9.9 * t + 3.4
    } else if t < c {
        ca * t2 - cb * t + cc
    } else {
        10.8 * t * t - 20.52 * t + 10.72
    }
}

pub fn bounce_in_out(t: f64) -> f64 {
    if t < 0.5 {
        0.5 * (1.0 - bounce_out(1.0 - t * 2.0))
    } else {
        0.5 * bounce_out(t * 2.0 - 1.0) + 0.5
    }
}

pub fn bounce_in(t: f64) -> f64 {
    1.0 - bounce_out(1.0 - t)
}

pub fn circ_in_out(t: f64) -> f64 {
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        0.5 * (2.0 * t - 2.0).powi(3) + 1.0
    }
}

pub fn circ_in(t: f64) -> f64 {
    1.0 - (1.0 - t * t).sqrt()
}

pub fn circ_out(t: f64) -> f64 {
    (1.0 - (t - 1.0) * (t - 1.0)).sqrt()
}

pub fn cubic_in_out(t: f64) -> f64 {
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        0.5 * (2.0 * t - 2.0).powi(3) + 1.0
    }
}

pub fn cubic_in(t: f64) -> f64 {
    t * t * t
}

pub fn cubic_out(t: f64) -> f64 {
    let f = t - 1.0;
    f * f * f + 1.0
}

pub fn elastic_in_out(t: f64) -> f64 {
    if t < 0.5 {
        0.5 * (13.0 * PI * 2.0 * t).sin() * 2.0_f64.powf(10.0 * (2.0 * t - 1.0))
    } else {
        0.5 * (13.0 * PI * 2.0 * t - 1.0 + 1.0).sin()
            * 2.0_f64.powf(-10.0 * (2.0 * t - 1.0))
            + 1.0
    }
}

pub fn elastic_in(t: f64) -> f64 {
    (13.0 * t * PI / 2.0).sin() * 2.0_f64.powf(10.0 * (t - 1.0))
}

pub fn elastic_out(t: f64) -> f64 {
    (13.0 * (t + 1.0) * PI / 2.0).sin() * 2.0_f64.powf(-10.0 * t) + 1.0
}

pub fn expo_in_out(t: f64) -> f64 {
    if t.abs() < f64::EPSILON || (t - 1.0).abs() < f64::EPSILON {
        t
    } else if t < 0.5 {
        0.5 * 2.0_f64.powf(20.0 * t - 10.0)
    } else {
        -0.5 * 2.0_f64.powf(10.0 - t * 20.0) + 1.0
    }
}

pub fn expo_in(t: f64) -> f64 {
    if t == 0.0 {
        t
    } else {
        2.0_f64.powf(10.0 * (t - 1.0))
    }
}

pub fn expo_out(t: f64) -> f64 {
    if (t - 1.0).abs() < f64::EPSILON {
        1.0
    } else {
        1.0 - 2.0_f64.powf(-10.0 * t)
    }
}

pub fn quad_in_out(mut t: f64) -> f64 {
    t /= 0.5;
    if t < 1.0 {
        0.5 * t * t
    } else {
        t -= 1.0;
        -0.5 * (t * (t - 2.0) - 1.0)
    }
}

pub fn quad_in(t: f64) -> f64 {
    t * t
}

pub fn quad_out(t: f64) -> f64 {
    -t * (t - 2.0)
}

pub fn quart_in_out(t: f64) -> f64 {
    if t < 0.5 {
        8.0 * t.powi(4)
    } else {
        -8.0 * (t - 1.0).powi(4) + 1.0
    }
}

pub fn quart_in(t: f64) -> f64 {
    t.powi(4)
}

pub fn quart_out(t: f64) -> f64 {
    -((t - 1.0).powi(4) - 1.0)
}

pub fn quint_in_out(mut t: f64) -> f64 {
    t *= 2.0;
    if t < 1.0 {
        0.5 * t * t * t * t * t
    } else {
        t -= 2.0;
        0.5 * (t * t * t * t * t + 2.0)
    }
}

pub fn quint_in(t: f64) -> f64 {
    t * t * t * t * t
}

pub fn quint_out(t: f64) -> f64 {
    let f = t - 1.0;
    f * f * f * f * f + 1.0
}

pub fn sine_in_out(t: f64) -> f64 {
    -0.5 * (PI * t).cos() + 1.0
}

pub fn sine_in(t: f64) -> f64 {
    1.0 - (PI * 0.5 * t).cos()
}

pub fn sine_out(t: f64) -> f64 {
    (PI * 0.5 * t).sin()
}
