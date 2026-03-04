#import bevy_render::view::View

#import bevy_moon::maths::{from_3x4_to_mat4x4}
#import bevy_moon::quad::{
    normalize_vertex_index,
    get_vertex_by_index, 
    get_corner_index,
    get_inset_by_index,
    to_uv,
}
#import bevy_moon::utils::{is_all3, is_empty4}
#import bevy_moon::utils::{aa_c, aa_s}
#import bevy_moon::rectangles::{sd_rounded_box, sd_inset_rounded_box}
#import bevy_moon::atlas

@group(0) @binding(0) var<uniform> view: View;

struct VertexInput {
    @builtin(vertex_index) vertex_id: u32,

    @location(0) x_axis: vec3<f32>,
    @location(1) y_axis: vec3<f32>,
    @location(2) z_axis: vec3<f32>,
    @location(3) translation: vec3<f32>,

    @location(4) color: vec4<f32>,
    @location(5) size: vec2<f32>,
    @location(6) corner_radii: vec4<f32>,
    @location(7) border_color: vec4<f32>,
    @location(8) border_widths: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
 
    @location(0) uv: vec2<f32>,
    @location(1) local_position: vec2<f32>,

    @location(2) @interpolate(flat) color: vec4<f32>,
    @location(3) @interpolate(flat) size: vec2<f32>,
    @location(4) @interpolate(flat) corner_radii: vec4<f32>,
    @location(5) @interpolate(flat) border_color: vec4<f32>,
    @location(6) @interpolate(flat) border_widths: vec4<f32>,
};

@vertex
fn vertex(in: VertexInput) -> VertexOutput {
    let vertex_index = normalize_vertex_index(in.vertex_id);
    let vertex = get_vertex_by_index(vertex_index);

    let uv = to_uv(vertex_index);
    let local_position = vertex * in.size;
    let world_from_local = vec4(local_position, 0.0, 1.0);
    let matrix = from_3x4_to_mat4x4(in.x_axis, in.y_axis, in.z_axis, in.translation);
    let world_position = matrix * world_from_local;
    let clip_position = view.clip_from_world * world_position;

    return VertexOutput(
        clip_position,
        uv,
        local_position,
        in.color,
        in.size,
        in.corner_radii,
        in.border_color,
        in.border_widths,
    );
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    var color = in.color;

    let corner_radii = in.corner_radii;
    let border_widths = in.border_widths;

    let unborded = is_empty4(border_widths);
    let unrounded = is_empty4(corner_radii);

    // fast path
    if (unborded && unrounded) {
        return color;
    }

    // position relative to the center of the box
    let point = in.local_position;
    let corner_index = get_corner_index(point);
    
    let tl = get_inset_by_index(border_widths, 0); // TopLeft
    let br = get_inset_by_index(border_widths, 2); // BottomRight
    let cb = get_inset_by_index(border_widths, corner_index); // Current corner border
 
    let half_size = in.size * 0.5;
    let radius = corner_radii[corner_index];
    let border_color = in.border_color;

    // If there's a border color and border width we need to calculate the inner sdf.
    if (is_all3(vec3(cb, border_color.a))) {
        // inner sdf
        let internal_distance = sd_inset_rounded_box(point, half_size, radius, tl, br, cb);

        // Blend in the color with the border color.
        color = mix(color, border_color, smoothstep(-0.5, 0.5, internal_distance));
    }

    // If there's no corner radius, we don't need to do any anti aliasing.
    if (radius <= 0.0) {
        return color;
    }

    // outer sdf
    let external_distance = sd_rounded_box(point, half_size, radius);

    // let a = 1.0 - smoothstep(-0.75, -0.1, external_distance);
    // let b = 1.0 - smoothstep(-0.1, 0.55, external_distance); // +0.65

    // color.a *= a; // The original is just multiplied by a.
    // color.a *= mix(a, b, b); // Repair the blank gap caused by antiasing.

    // let c = aa_c(external_distance);
    let s = aa_s(external_distance);

    color.a *= s;

    return color;
}
