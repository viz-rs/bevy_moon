#import bevy_render::view::View
#import bevy_moon::maths::{
    FRAC_2_SQRT_PI,
    INVERT_SQRT_2,
    SQRT_PI_2,
}
#import bevy_moon::quad::{
    normalize_vertex_index,
    get_vertex_by_index,
    get_corner_index,
    get_inset_by_index,
}
#import bevy_moon::rectangles::{sd_rounded_box}

@group(0) @binding(0) var<uniform> view: View;

const SAMPLES: i32 = max(4, #SHADOW_SAMPLES);

// A standard gaussian function, used for weighting samples
// 
// <https://en.wikipedia.org/wiki/Gaussian_function>
fn gaussian(x: f32, sigma: f32) -> f32 {
    return exp(-(x * x) / (2.0 * sigma * sigma)) / (SQRT_PI_2 * sigma);
}

// This approximates the error function, needed for the gaussian integral
// 
// <https://en.wikipedia.org/wiki/Error_function>
fn erf(v: vec2<f32>) -> vec2<f32> {
    let s = sign(v);
    let a = abs(v);
    // fourth degree polynomial approximation for erf
    var r = 1.0 + (0.278393 + (0.230389 + (0.000972 + 0.078108 * a) * a) * a) * a;
    r *= r;
    r *= r;
    return s - s / r;
}

fn blur_along_x(point: vec2<f32>, half_size: vec2<f32>, radius: f32, sigma: f32) -> f32 {
    let v = INVERT_SQRT_2 / sigma;
    let ranged = calc_x_range(point, half_size, radius);
    let integral = 0.5 + 0.5 * erf(ranged * v);
    return integral.y - integral.x;
}

fn blur(point: vec2<f32>, half_size: vec2<f32>, radius: f32, sigma: f32) -> f32 {
    let range = calc_range(point, half_size, sigma);
    let start = range.x;                
    let end = range.y;
    let step = (end - start) / f32(SAMPLES);

    var y = start + step * 0.5;
    var alpha = 0.0;
    for (var i = 0; i < SAMPLES; i += 1) {
        let blur = blur_along_x(point - vec2(0.0, y), half_size, radius, sigma);
        alpha += blur * gaussian(y, sigma) * step;
        y += step;
    }

    return alpha;
}

fn calc_range(point: vec2<f32>, half_size: vec2<f32>, sigma: f32) -> vec2<f32> {
    let low = point.y - half_size.y;
    let high = point.y + half_size.y;
    let start = clamp(-3.0 * sigma, low, high);
    let end = clamp(3.0 * sigma, low, high);
    return vec2(start, end);
}

fn calc_x_range(point: vec2<f32>, half_size: vec2<f32>, radius: f32) -> vec2<f32> {
    let delta = min(half_size.y - radius - abs(point.y), 0.0);
    let curved = half_size.x - radius + sqrt(max(0.0, radius * radius - delta * delta));
    let ranged = point.x + vec2(-curved, curved);
    return ranged;
}

// Approximate the erf function
//
// <https://raphlinus.github.io/audio/2018/09/05/sigmoid.html>
fn erf7(v: vec2<f32>) -> vec2<f32> {
    var x = v * FRAC_2_SQRT_PI;
    let xx = x * x;
    x = x + (0.24295 + (0.03395 + 0.0104 * xx) * xx) * (x * xx);
    return x / sqrt(1.0 + x * x);
}

// Fast gaussian blur
fn blur7(point: vec2<f32>, half_size: vec2<f32>, radius: f32, sigma: f32) -> f32 {
    let range = calc_range(point, half_size, sigma);
    let start = range.x;                
    let end = range.y;
    let step = (end - start);

    let v = INVERT_SQRT_2 / sigma;
    let d = sd_rounded_box(point, half_size, radius);
    let ranged = d + vec2(0.0, select(radius, 0.5, radius == 0.0) * step);
    let integral = 0.5 * erf7(ranged * v);
    return integral.y - integral.x;
}

struct VertexInput {
    @builtin(vertex_index) vertex_id: u32,

    @location(0) x_axis: vec4<f32>,
    @location(1) y_axis: vec4<f32>,
    @location(2) z_axis: vec4<f32>,
    @location(3) w_axis: vec4<f32>,

    @location(4) color: vec4<f32>,
    @location(5) size: vec2<f32>,
    @location(6) corner_radii: vec4<f32>,
    @location(7) blur_radius: f32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,

    @location(0) local_position: vec2<f32>,

    @location(1) @interpolate(flat) color: vec4<f32>,
    @location(2) @interpolate(flat) size: vec2<f32>,
    @location(3) @interpolate(flat) corner_radii: vec4<f32>,
    @location(4) @interpolate(flat) blur_radius: f32,
};

@vertex
fn vertex(in: VertexInput) -> VertexOutput {
    let vertex_index = normalize_vertex_index(in.vertex_id);
    let vertex = get_vertex_by_index(vertex_index);

    let margin = in.blur_radius * 3.0;
    let bounds = in.size + margin * 2.0; // shadow bounds
    let local_position = vertex * bounds;
    let world_from_local = vec4(local_position, 0.0, 1.0);
    let matrix = mat4x4(in.x_axis, in.y_axis, in.z_axis, in.w_axis);
    let world_position = matrix * world_from_local;
    let clip_position = view.clip_from_world * world_position;

    return VertexOutput(
        clip_position,
        local_position,
        in.color,
        in.size,
        in.corner_radii,
        in.blur_radius,
    );
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let half_size = in.size * 0.5;
    let point = in.local_position;
    let blur_radius = in.blur_radius;
    let corner_index = get_corner_index(point);
    let radius = in.corner_radii[corner_index];

    let a = blur7(point, half_size, radius, blur_radius);
    // let a = blur(point, half_size, radius, blur_radius);

    var color = in.color;

    // debug
    // color.a *= smoothstep(0.0, 0.25, a);
    color.a *= a;

    return color;
}
