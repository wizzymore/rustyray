use std::f32::consts::PI;

/// Ease: Linear
pub fn ease_linear(t: f32, b: f32, c: f32, d: f32) -> f32 {
    c * t / d + b
}

/// Ease: Linear In
pub fn ease_linear_in(t: f32, b: f32, c: f32, d: f32) -> f32 {
    c * t / d + b
}

/// Ease: Linear Out
pub fn ease_linear_out(t: f32, b: f32, c: f32, d: f32) -> f32 {
    c * t / d + b
}

/// Ease: Linear In Out
pub fn ease_linear_inout(t: f32, b: f32, c: f32, d: f32) -> f32 {
    c * t / d + b
}

// Sine Easing functions
/// Ease: Sine In
pub fn ease_sine_in(t: f32, b: f32, c: f32, d: f32) -> f32 {
    -c * f32::cos(t / d * (PI / 2.0)) + c + b
}
/// Ease: Sine Out
pub fn ease_sine_out(t: f32, b: f32, c: f32, d: f32) -> f32 {
    c * f32::sin(t / d * (PI / 2.0)) + b
}
/// Ease: Sine In Out
pub fn ease_sine_inout(t: f32, b: f32, c: f32, d: f32) -> f32 {
    -c / 2.0 * (f32::cos(PI * t / d) - 1.0) + b
}

// Circular Easing functions
/// Ease: Circular In
pub fn ease_circ_in(t: f32, b: f32, c: f32, d: f32) -> f32 {
    let t = t / d;
    -c * (f32::sqrt(1.0 - t * t) - 1.0) + b
}
/// Ease: Circular Out
pub fn ease_circ_out(t: f32, b: f32, c: f32, d: f32) -> f32 {
    let t = t / d - 1.;
    c * f32::sqrt(1.0 - t * t) + b
}
/// Ease: Circular In Out
pub fn ease_circ_inout(t: f32, b: f32, c: f32, d: f32) -> f32 {
    let mut t = t / (d / 2.0);
    if t < 1.0 {
        return -c / 2.0 * (f32::sqrt(1.0 - t * t) - 1.0) + b;
    }
    t -= 2.0;
    c / 2.0 * (f32::sqrt(1.0 - t * t) + 1.0) + b
}

// Cubic Easing functions
/// Ease: Cubic In
pub fn ease_cubic_in(t: f32, b: f32, c: f32, d: f32) -> f32 {
    let t = t / d;
    c * t * t * t + b
}
/// Ease: Cubic Out
pub fn ease_cubic_out(t: f32, b: f32, c: f32, d: f32) -> f32 {
    let t = t / d - 1.0;
    c * (t * t * t + 1.0) + b
}
/// Ease: Cubic In Out
pub fn ease_cubic_inout(t: f32, b: f32, c: f32, d: f32) -> f32 {
    let mut t = t / (d / 2.0);
    if t < 1.0 {
        return c / 2.0 * t * t * t + b;
    }
    t -= 2.;
    c / 2.0 * (t * t * t + 2.0) + b
}

// Quadratic Easing functions
/// Ease: Quadratic In
pub fn ease_quad_in(t: f32, b: f32, c: f32, d: f32) -> f32 {
    let t = t / d;
    c * t * t + b
}
/// Ease: Quadratic Out
pub fn ease_quad_out(t: f32, b: f32, c: f32, d: f32) -> f32 {
    let t = t / d;
    -c * t * (t - 2.0) + b
}
/// Ease: Quadratic In Out
pub fn ease_quad_inout(t: f32, b: f32, c: f32, d: f32) -> f32 {
    let t = t / (d / 2.0);
    if t < 1.0 {
        return ((c / 2.0) * (t * t)) + b;
    }
    -c / 2.0 * (((t - 1.0) * (t - 3.)) - 1.0) + b
}

// Exponential Easing functions
/// Ease: Exponential In
pub fn ease_expo_in(t: f32, b: f32, c: f32, d: f32) -> f32 {
    if t == 0.0 {
        return b;
    }
    c * f32::powf(2.0, 10.0 * (t / d - 1.0)) + b
}
/// Ease: Exponential Out
pub fn ease_expo_out(t: f32, b: f32, c: f32, d: f32) -> f32 {
    if t == d {
        return b + c;
    }
    c * (-f32::powf(2.0, -10.0 * t / d) + 1.0) + b
}
/// Ease: Exponential In Out
pub fn ease_expo_inout(t: f32, b: f32, c: f32, d: f32) -> f32 {
    if t == 0.0 {
        return b;
    }
    if t == d {
        return b + c;
    }
    let t = t / (d / 2.0);

    if t < 1.0 {
        return c / 2.0 * f32::powf(2.0, 10.0 * (t - 1.0)) + b;
    }
    c / 2.0 * (-f32::powf(2.0, -10.0 * (t - 1.0)) + 2.0) + b
}

// Back Easing functions
/// Ease: Back In
#[inline]
pub fn ease_back_in(t: f32, b: f32, c: f32, d: f32) -> f32 {
    const S: f32 = 1.70158;
    let t = t / d;
    c * t * t * ((S + 1.0) * t - S) + b
}
/// Ease: Back Out
#[inline]
pub fn ease_back_out(t: f32, b: f32, c: f32, d: f32) -> f32 {
    const S: f32 = 1.70158;
    let t = t / d - 1.0;
    c * (t * t * ((S + 1.0) * t + S) + 1.0) + b
}
/// Ease: Back In Out
#[inline]
pub fn ease_back_inout(t: f32, b: f32, c: f32, d: f32) -> f32 {
    let mut s = 1.70158;
    let mut t = t / (d / 2.0);
    if t < 1.0 {
        s *= 1.525;
        return c / 2.0 * (t * t * ((s + 1.0) * t - s)) + b;
    }
    t -= 2.0;
    s *= 1.525;
    c / 2.0 * (t * t * ((s + 1.0) * t + s) + 2.0) + b
}

// Bounce Easing functions
/// Ease: Bounce In
#[inline]
pub fn ease_bounce_in(t: f32, b: f32, c: f32, d: f32) -> f32 {
    c - ease_bounce_out(d - t, 0.0, c, d) + b
}
/// Ease: Bounce Out
#[inline]
pub fn ease_bounce_out(t: f32, b: f32, c: f32, d: f32) -> f32 {
    let mut t = t / d;
    if t < 1.0 / 2.75 {
        return (c * 7.5625 * t * t) + b;
    }
    if t < 2.0 / 2.75 {
        t -= 1.5 / 2.75;
        return c * (7.5625 * t * t + 0.75) + b;
    }
    if t < 2.5 / 2.75 {
        t -= 2.25 / 2.75;
        return c * (7.5625 * t * t + 0.9375) + b;
    }
    t -= 2.625 / 2.75;
    c * (7.5625 * t * t + 0.984375) + b
}
/// Ease: Bounce In Out
#[inline]
pub fn ease_bounce_inout(t: f32, b: f32, c: f32, d: f32) -> f32 {
    if t < d / 2.0 {
        return ease_bounce_in(t * 2.0, 0.0, c, d) * 0.5 + b;
    }
    ease_bounce_out(t * 2.0 - d, 0.0, c, d) * 0.5 + c * 0.5 + b
}

// Elastic Easing functions
/// Ease: Elastic In
#[inline]
pub fn ease_elastic_in(t: f32, b: f32, c: f32, d: f32) -> f32 {
    if t == 0.0 {
        return b;
    }
    let mut t = t;
    t /= d;
    if t == 1.0 {
        return b + c;
    }

    let p = d * 0.3;
    let a = c;
    let s = p / 4.0;
    t -= 1.0;
    let post_fix = a * f32::powf(2.0, 10.0 * t);

    -(post_fix * f32::sin((t * d - s) * (2.0 * PI) / p)) + b
}
/// Ease: Elastic Out
#[inline]
pub fn ease_elastic_out(t: f32, b: f32, c: f32, d: f32) -> f32 {
    if t == 0.0 {
        return b;
    }
    let t = t / d;
    if t == 1.0 {
        return b + c;
    }

    let p = d * 0.3;
    let a = c;
    let s = p / 4.0;

    a * f32::powf(2.0, -10.0 * t) * f32::sin((t * d - s) * (2.0 * PI) / p) + c + b
}
/// Ease: Elastic In Out
#[inline]
pub fn ease_elastic_inout(t: f32, b: f32, c: f32, d: f32) -> f32 {
    if t == 0.0 {
        return b;
    }
    let mut t = t;
    t /= d / 2.0;
    if t == 2.0 {
        return b + c;
    }

    let p = d * (0.3 * 1.5);
    let s = p / 4.0;

    t -= 1.0;
    let post_fix = c * f32::powf(2.0, -10.0 * t);
    if t < 1.0 {
        return -0.5 * (post_fix * f32::sin((t * d - s) * (2.0 * PI) / p)) + b;
    }

    post_fix * f32::sin((t * d - s) * (2.0 * PI) / p) * 0.5 + c + b
}
