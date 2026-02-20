#import bevy_render::view::View
#import bevy_moon::utils::{antialias, antialias_alpha, over}
#import bevy_moon::utils::{is_xyzw_zero, get_vertex_by_index, get_corner_index, get_inset_by_index}
#import bevy_moon::rectangles::{sd_rounded_box, sd_inset_rounded_box}

@group(0) @binding(0) var<uniform> view: View;

// @group(1) @binding(0) var sprite_texture: texture_2d<f32>;
// @group(1) @binding(1) var sprite_sampler: sampler;

struct VertexInput {
    @builtin(vertex_index) index: u32,
    
    @location(0) position: vec3<f32>,
    @location(1) size: vec2<f32>,
    @location(2) color: vec4<f32>,
    @location(3) corner_radii: vec4<f32>,
    @location(4) border_widths: vec4<f32>,
    @location(5) border_color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    
    @location(0) uv: vec2<f32>,
    
    @location(1) local_position: vec2<f32>,
    @location(2) @interpolate(flat) size: vec2<f32>,
    @location(3) @interpolate(flat) background_color: vec4<f32>,
    @location(4) @interpolate(flat) corner_radii: vec4<f32>,
    @location(5) @interpolate(flat) border_widths: vec4<f32>,
    @location(6) @interpolate(flat) border_color: vec4<f32>,
};

@vertex
fn vertex(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    
    out.size = in.size;
    out.border_color = in.border_color;
    out.border_widths = in.border_widths;
    out.corner_radii = in.corner_radii;
    out.background_color = in.color;
    
    let vertex = get_vertex_by_index(in.index);
    let local_position = vertex * in.size;
    let world_position = in.position.xyz + vec3(local_position, 0.0);
    
    out.uv = vertex + vec2(0.5);
    out.local_position = local_position;
    out.clip_position = view.clip_from_world * vec4(world_position, 1.0);
    
    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let size = in.size;
    let border_widths = in.border_widths;
    let corner_radii = in.corner_radii;
    
    let unborded = is_xyzw_zero(border_widths);
    let unrounded = is_xyzw_zero(corner_radii);
    
    let background_color = in.background_color;
    let border_color = in.border_color;
    
    // fast path
    if unborded && unrounded {
        return background_color;
    }
    
    let half_size = size * 0.5;
    // position relative to the center of the box
    let point = in.local_position;
    let corner_index = get_corner_index(point);
    let radius = corner_radii[corner_index];
    
    let tl = get_inset_by_index(border_widths, 0); // TopLeft
    let br = get_inset_by_index(border_widths, 2); // BottomRight
    var cb: vec2<f32>;                             // current corner border
    switch corner_index {
        case 0u: { cb = tl; }
        case 2u: { cb = br; }
        default: { cb = get_inset_by_index(border_widths, corner_index); }
    }
    
    var color = background_color;
    
    // If there's a border color and border width we need to calculate the inner sdf.
    if all(vec3(cb, border_color.a) != vec3(0.0)) {
        // inner sdf
        let internal_distance = sd_inset_rounded_box(point, half_size, radius, tl, br, cb);
        
        // Blend in the color with the border color.
        color = mix(background_color, border_color, smoothstep(-0.5, 0.5, internal_distance));
    }
    
    // If there's a corner radius we need to do some anti aliasing to smooth out the rounded corner effect.
    if radius > 0.0 {
        // outer sdf
        let external_distance = sd_rounded_box(point, half_size, radius);
    
        let a = 1.0 - smoothstep(-0.75, -0.1, external_distance);
        let b = 1.0 - smoothstep(-0.1, 0.55, external_distance); // +0.65
        
        // color.a *= a; // The original is just multiplied by a.
        color.a *= mix(a, b, b); // Repair the blank gap caused by antiasing.
    }
        
    return color;
}
