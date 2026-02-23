#define_import_path bevy_moon::utils

#import bevy_render::maths::PI

// AntiAlias Threshold
const AA_T: f32 = 0.5;

 // 1.414213562373095
const SQRT_2: f32 = sqrt(2.0);

// 0.7071067811865476
const INVERT_SQRT_2: f32 = 1.0 / SQRT_2;

 // 1.1283791670955126
const FRAC_2_SQRT_PI: f32 = 2.0 / sqrt(PI);

// const quad_vertices: array<vec2<f32>, 6> = array(
//     vec2(-0.5, -0.5), // 0 - BottomLeft
//     vec2( 0.5, -0.5), // 1 - BottomRight
//     vec2(-0.5,  0.5), // 2 - TopLeft
//     vec2( 0.5,  0.5), // 3 - TopRight
//     vec2(-0.5, -0.5), // 4 - BottomLeft
//     vec2( 0.5, -0.5), // 5 - BottomRight
// );
const quad_vertices: array<vec2<f32>, 4> = array(
    vec2(-0.5, -0.5), // 0 - BottomLeft
    vec2( 0.5, -0.5), // 1 - BottomRight
    vec2(-0.5,  0.5), // 2 - TopLeft
    vec2( 0.5,  0.5), // 3 - TopRight
);

fn normalize_vertex_index(index: u32) -> u32 {
    return index % 4;
}

fn get_vertex_by_index(index: u32) -> vec2<f32> {
    return quad_vertices[index];
}

fn get_corner_index(point: vec2<f32>) -> u32 {
    let s = sign(point);
    let c = select(0u, 1u, s.x == s.y);
    let r = select(2u, 0u, s.y == 1.0);
    return c + r;
}

fn get_inset_by_index(insets: vec4<f32>, index: u32) -> vec2<f32> {
    return array(insets.wx, insets.yx, insets.yz, insets.wz)[index];
}

fn is_xyzw_zero(v: vec4<f32>) -> bool {
    return all(v == vec4(0.0));
}

fn antialias_f(d: f32) -> f32 {
    // length(vec2(dpdx(d), dpdy(d))) - fwidth(d) < 0.001;
    // let ps = length(vec2(dpdx(d), dpdy(d))); // pixel size
    let ps = fwidth(d); // pixel size
    let t = ps * SQRT_2;
    return 1.0 - d / (ps + 0.001);
}

fn antialias_c(d: f32) -> f32 {
    let t = d / AA_T / 0.5;
    return clamp(0.5 - t, 0.0, 1.0);
}

fn antialias_s(d: f32) -> f32 {
    let t = d / AA_T / 0.5;
    return smoothstep(0.0, 1.0, 0.5 - t);
}
