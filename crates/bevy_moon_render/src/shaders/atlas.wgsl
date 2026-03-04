#import bevy_render::view::View

#import bevy_moon::maths::{from_3x4_to_mat4x4}
#import bevy_moon::flags::{GLYPH, enabled}
#import bevy_moon::quad::{
    normalize_vertex_index,
    get_vertex_by_index, 
    get_corner_index,
    to_uv,
}
#import bevy_moon::utils::is_empty4
#import bevy_moon::utils::aa_s
#import bevy_moon::rectangles::sd_rounded_box
#import bevy_moon::atlas

@group(0) @binding(0) var<uniform> view: View;

@group(1) @binding(0) var atlas_texture: texture_2d<f32>;
@group(1) @binding(1) var atlas_sampler: sampler;

struct VertexInput {
    @builtin(vertex_index) vertex_id: u32,

    @location(0) x_axis: vec3<f32>,
    @location(1) y_axis: vec3<f32>,
    @location(2) z_axis: vec3<f32>,
    @location(3) translation: vec3<f32>,

    @location(4) color: vec4<f32>,
    @location(5) size: vec2<f32>,
    @location(6) flags: u32,
    @location(7) corner_radii: vec4<f32>,

    // glyph: [left, top, scale]
    // image: [ObjectPosition.x, ObjectPosition.y, ObjectFit]
    @location(8) extra: vec3<f32>,
    @location(9) flipped: vec2<u32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
 
    @location(0) uv: vec2<f32>,
    @location(1) local_position: vec2<f32>,

    @location(2) @interpolate(flat) color: vec4<f32>,
    @location(3) @interpolate(flat) size: vec2<f32>,
    @location(4) @interpolate(flat) flags: u32,
    @location(5) @interpolate(flat) corner_radii: vec4<f32>,
    @location(6) @interpolate(flat) extra: vec3<f32>,
    @location(7) @interpolate(flat) flipped: vec2<u32>,
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
        in.flags,
        in.corner_radii,
        in.extra,
        in.flipped
    );
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let src_size = vec2<f32>(textureDimensions(atlas_texture, 0));
    let dst_size = in.size;
    let position = in.extra.xy;
    
    var color = in.color;
    var uv = atlas::flip_uv(in.uv, in.flipped);

    if (in.flags == GLYPH) {
        let scale_factor = in.extra.z;
        let current_src_size = src_size * scale_factor;
        uv = atlas::glyph_tile_uv(uv, dst_size, current_src_size, position);
        // let a = textureSample(atlas_texture, atlas_sampler, uv).a;
        // color.a *= a;
        let d = textureSample(atlas_texture, atlas_sampler, uv);
        color *= d;
        return color;
    } else {
        let mode = u32(in.extra.z);
        uv = atlas::object_fit(uv, dst_size, src_size, position, mode);
        let d = textureSample(atlas_texture, atlas_sampler, uv);
        color *= d;
    }
    
    // fast path
    if (color.a <= 0.0) {
        discard;
    }

    let corner_radii = in.corner_radii;
    let unrounded = is_empty4(corner_radii);

    // fast path
    if unrounded {
        return color;
    }

    // position relative to the center of the box
    let point = in.local_position;
    let corner_index = get_corner_index(point);
    let half_size = in.size * 0.5;
    let radius = corner_radii[corner_index];

    // fast path
    if (radius <= 0.0) {
        return color;
    }

    // outer sdf
    let external_distance = sd_rounded_box(point, half_size, radius);

    let s = aa_s(external_distance);

    color.a *= s;

    return color;
}
