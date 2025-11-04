use rand::prelude::*;
use hsl::HSL;
use ansi_colours::ansi256_from_rgb;
use std::f64::consts::PI;

pub fn random_colours(rng: &mut rand::rngs::ThreadRng, num: usize) -> Vec<u8> {
    // let's try some math magic, maybe it will work
    let initial_hue = rng.random_range(0.0..360.0);
    let hue_step = rng.random_range(0.5..5.0) * 360.0;
    let initial_sat = rng.random_range(0.0..1.0);
    let sat_multiplier = rng.random_range(1.0..10.0) * PI;
    let initial_brightness = rng.random_range(0.0..1.0);
    let brightness_multiplier = rng.random_range(1.0..10.0) * PI;

    let mut result = Vec::new();
    for i in 0..num {
        let i = i as f64;
        let hue = (initial_hue + (hue_step * i)) % 360.0;
        let saturation = (initial_sat + sat_multiplier * i).sin().abs() * 0.5 + 0.5;
        let brightness = (initial_brightness + brightness_multiplier * i).sin().abs() * 0.5 + 0.3;
        let hsl_col = HSL { h: hue, s: saturation, l: brightness };
        let ansi_col = ansi256_from_rgb(hsl_col.to_rgb());
        result.push(ansi_col);
    }
    result
}
