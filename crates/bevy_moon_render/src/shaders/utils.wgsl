#define_import_path bevy_moon::utils

#import bevy_moon::prelude::{AA_T, SQRT_2, quad_vertices}

fn normalize_vertex_index(index: u32) -> u32 {
    return index % 4;
}

fn get_vertex_by_index(index: u32) -> vec2<f32> {
    return quad_vertices[index];
}

fn get_corner_index(point: vec2<f32>) -> u32 {
    let s = sign(point);
    let c = select(0u, 1u, s.x == s.y); // column
    let r = select(2u, 0u, s.y == 1.0); // row
    return c + r;
}

fn get_inset_by_index(insets: vec4<f32>, index: u32) -> vec2<f32> {
    return array(insets.wx, insets.yx, insets.yz, insets.wz)[index];
}

// Converts a local position to UV coordinates
// fn to_uv(vertex: vec2<f32>) -> vec2<f32> {
//     return vertex * vec2(1.0, -1.0) + vec2(0.5);
// }

// Converts a vertex index to UV coordinates
fn to_uv(vertex_index: u32) -> vec2<f32> {
    let v = vertex_index ^ 2u;
    return vec2(f32(v & 1u), f32(v >> 1u));
}

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
