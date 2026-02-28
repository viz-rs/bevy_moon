#define_import_path bevy_moon::quad

// const QUAD_VERTICES: array<vec2<f32>, 6> = array(
//     vec2(-0.5, -0.5), // 0 - BottomLeft
//     vec2( 0.5, -0.5), // 1 - BottomRight
//     vec2(-0.5,  0.5), // 2 - TopLeft
//     vec2( 0.5,  0.5), // 3 - TopRight
//     vec2(-0.5, -0.5), // 4 - BottomLeft
//     vec2( 0.5, -0.5), // 5 - BottomRight
// );
const QUAD_VERTICES: array<vec2<f32>, 4> = array(
    vec2(-0.5, -0.5), // 0 - BottomLeft
    vec2( 0.5, -0.5), // 1 - BottomRight
    vec2(-0.5,  0.5), // 2 - TopLeft
    vec2( 0.5,  0.5), // 3 - TopRight
);

fn normalize_vertex_index(index: u32) -> u32 {
    return index % 4;
}

fn get_vertex_by_index(index: u32) -> vec2<f32> {
    return QUAD_VERTICES[index];
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
