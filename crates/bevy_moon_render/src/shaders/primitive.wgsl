#import bevy_render::view::View

#import bevy_moon::flags::{TEXTURED, GLYPH, enabled}
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

@group(1) @binding(0) var sprite_texture: texture_2d<f32>;
@group(1) @binding(1) var sprite_sampler: sampler;

struct VertexInput {
    @builtin(vertex_index) vertex_id: u32,

    @location(0) position: vec3<f32>,
    @location(1) color: vec4<f32>,
 
    @location(2) size: vec2<f32>,
    @location(3) flags: u32,
    @location(4) corner_radii: vec4<f32>,
    @location(5) border_color: vec4<f32>,
    @location(6) border_widths: vec4<f32>,
    // glyph: [left, top, scale]
    // image: [ObjectPosition.x, ObjectPosition.y, ObjectFit]
    @location(7) extra: vec3<f32>,
    @location(8) flip: vec2<u32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
 
    @location(0) uv: vec2<f32>,
    @location(1) local_position: vec2<f32>,

    @location(2) @interpolate(flat) color: vec4<f32>,
    @location(3) @interpolate(flat) size: vec2<f32>,
    @location(4) @interpolate(flat) flags: u32,
    @location(5) @interpolate(flat) corner_radii: vec4<f32>,
    @location(6) @interpolate(flat) border_color: vec4<f32>,
    @location(7) @interpolate(flat) border_widths: vec4<f32>,
    @location(8) @interpolate(flat) extra: vec3<f32>,
    @location(9) @interpolate(flat) flip: vec2<u32>,
};

@vertex
fn vertex(in: VertexInput) -> VertexOutput {
    let vertex_index = normalize_vertex_index(in.vertex_id);
    let vertex = get_vertex_by_index(vertex_index);

    let uv = to_uv(vertex_index);
    let local_position = vertex * in.size;
    let world_position = in.position.xyz + vec3(local_position, 0.0);
    let clip_position = view.clip_from_world * vec4(world_position, 1.0);

    return VertexOutput(
        clip_position,
        uv,
        local_position,
        in.color,
        in.size,
        in.flags,
        in.corner_radii,
        in.border_color,
        in.border_widths,
        in.extra,
        in.flip
    );
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    var color = in.color;

    // should split this into a standalone pipeline
    if (enabled(in.flags, TEXTURED)) {
        var uv = atlas::flip_uv(in.uv, in.flip);

        let src_size = vec2<f32>(textureDimensions(sprite_texture, 0));
        let dst_size = in.size;
        let position = in.extra.xy;

        if (in.flags == GLYPH) {
            let scale_factor = in.extra.z;
            // In rust side, `in.position` and `in.size` have been applied a `scale_factor`
            let current_src_size = src_size * scale_factor;
            uv = atlas::glyph_tile_uv(uv, dst_size, current_src_size, position);
            let a = textureSample(sprite_texture, sprite_sampler, uv).a;
            color.a *= a;
        } else {
            let mode = u32(in.extra.z);
            uv = atlas::object_fit(uv, dst_size, src_size, position, mode);
            let d = textureSample(sprite_texture, sprite_sampler, uv);
            color *= d;
        }
    }

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
