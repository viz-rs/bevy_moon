#define_import_path bevy_moon::utils

#import bevy_moon::maths::{AA_T, SQRT_2}

fn is_all3(v: vec3<f32>) -> bool {
    return all(v != vec3(0.0));
}

fn is_empty3(v: vec3<f32>) -> bool {
    return all(v == vec3(0.0));
}

fn is_empty4(v: vec4<f32>) -> bool {
    return all(v == vec4(0.0));
}

// Anti-aliasing function by `clamp`
fn aa_c(d: f32) -> f32 {
    let t = d / AA_T / 0.5;
    return clamp(0.5 - t, 0.0, 1.0);
}

// Anti-aliasing function by `fwidth`
// Does not work well with atlas texture edges.
fn aa_f(d: f32) -> f32 {
    // length(vec2(dpdx(d), dpdy(d))) - fwidth(d) < 0.001;
    // let ps = length(vec2(dpdx(d), dpdy(d))); // pixel size
    let ps = fwidth(d); // pixel size
    let t = ps * SQRT_2;
    return 1.0 - d / (ps + 0.001);
}

// Anti-aliasing function by `smoothstep`
fn aa_s(d: f32) -> f32 {
    let t = d / AA_T / 0.5;
    return smoothstep(0.0, 1.0, 0.5 - t);
}
