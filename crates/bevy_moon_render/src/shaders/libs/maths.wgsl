#define_import_path bevy_moon::maths

#import bevy_render::maths

const PI = maths::PI;
const PI_2 = maths::PI_2;
const HALF_PI = maths::HALF_PI;

// AntiAliasing Threshold
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
