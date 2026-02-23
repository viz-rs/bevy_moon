#define_import_path bevy_moon::prelude

#import bevy_render::maths::{PI, PI_2, HALF_PI}

// AntiAlias Threshold
const AA_T: f32 = 0.5;

// 0.7071067811865476
const SQRT_HALF: f32 = sqrt(0.5);

// 1.414213562373095
const SQRT_2: f32 = sqrt(2.0);

// 0.7071067811865476
const INVERT_SQRT_2: f32 = 1.0 / SQRT_2;

// 1.772453850905515
const SQRT_PI: f32 = sqrt(PI);

// 2.5066282746310002
const SQRT_PI_2: f32 = sqrt(PI_2);

// 1.1283791670955126
const FRAC_2_SQRT_PI: f32 = 2.0 / SQRT_PI;

// 0.6366197723675814
const INVERT_HALF_PI: f32 = 1.0 / HALF_PI;

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
