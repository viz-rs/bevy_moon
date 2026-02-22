#import bevy_render::maths::PI
#import bevy_render::view::View
#import bevy_moon::utils::{normalize_vertex_index, get_vertex_by_index}
#import bevy_moon::utils::{get_corner_index, get_inset_by_index}

@group(0) @binding(0) var<uniform> view: View;

const FRAC_2_SQRT_PI: f32 = 2.0 / sqrt(PI);
const SAMPLES: i32 = max(4, #SHADOW_SAMPLES);

// A standard gaussian function, used for weighting samples
// 
// <https://en.wikipedia.org/wiki/Gaussian_function>
fn gaussian(x: f32, sigma: f32) -> f32 {
    return exp(-(x * x) / (2.0 * sigma * sigma)) / (sqrt(2.0 * PI) * sigma);
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

// Approximate the erf function
//
// <https://raphlinus.github.io/audio/2018/09/05/sigmoid.html>
fn compute_erf7(v: vec2<f32>) -> vec2<f32> {
    var x = v * FRAC_2_SQRT_PI;
    let xx = x * x;
    x = x + (0.24295 + (0.03395 + 0.0104 * xx) * xx) * (x * xx);
    return x / sqrt(1.0 + x * x);
}

fn blur_along_x(x: f32, y: f32, half_size: vec2<f32>, radius: f32, sigma: f32) -> f32 {
    let delta = min(half_size.y - radius - abs(y), 0.0);
    let curved = half_size.x - radius + sqrt(max(0.0, radius * radius - delta * delta));
    // let integral = 0.5 + 0.5 * erf((x + vec2(-curved, curved)) * (sqrt(0.5) / sigma));
    let integral = 0.5 + 0.5 * compute_erf7((x + vec2(-curved, curved)) * (sqrt(0.5) / sigma));
    return integral.y - integral.x;
}


fn rounded_box_shadow(point: vec2<f32>, half_size: vec2<f32>, radius: f32, sigma: f32) -> f32 {
    let low = point.y - half_size.y;
    let high = point.y + half_size.y;
    let start = clamp(-3.0 * sigma, low, high);
    let end = clamp(3.0 * sigma, low, high);
    
    let step = (end - start) / f32(SAMPLES);
    var y = start + step * 0.5;
    var alpha = 0.0;
    for (var i = 0; i < SAMPLES; i += 1) {
        let blur = blur_along_x(point.x, point.y - y, half_size, radius, sigma);
        alpha += blur * gaussian(y, sigma) * step;
        y += step;
    }
    
    return alpha;
}

struct VertexInput {
    @builtin(vertex_index) vertex_index: u32,
    
    @location(0) position: vec3<f32>,
    @location(1) size: vec2<f32>,
    @location(2) color: vec4<f32>,
    @location(3) corner_radii: vec4<f32>,
    @location(4) blur_radius: f32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    
    @location(0) uv: vec2<f32>,
    @location(1) local_position: vec2<f32>,
    
    @location(2) @interpolate(flat) size: vec2<f32>,
    @location(3) @interpolate(flat) color: vec4<f32>,
    @location(4) @interpolate(flat) corner_radii: vec4<f32>,
    @location(5) @interpolate(flat) blur_radius: f32,
};

@vertex
fn vertex(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    
    out.size = in.size;
    out.color = in.color;
    out.corner_radii = in.corner_radii;
    out.blur_radius = in.blur_radius;
    
    let vertex_index = normalize_vertex_index(in.vertex_index);
    let vertex = get_vertex_by_index(vertex_index);
    
    let margin = in.blur_radius * 3.0;
    let bounds = in.size + margin * 2.0;
    let local_position = vertex * bounds;
    let world_position = in.position.xyz + vec3(local_position, 0.0);
    
    out.uv = vertex + vec2(0.5);
    out.local_position = local_position;
    out.clip_position = view.clip_from_world * vec4(world_position, 1.0);
    
    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let size = in.size;
    let half_size = size * 0.5;
    let point = in.local_position;
    let corner_index = get_corner_index(point);
    let radius = in.corner_radii[corner_index];
    let blur_radius = in.blur_radius;
    var color = in.color;
    
    let alpha = rounded_box_shadow(point, half_size, radius, blur_radius);
    
    // debug
    // color.a *= smoothstep(0.0, 0.25, alpha);
    color.a *= alpha;
    
    return color;
}
