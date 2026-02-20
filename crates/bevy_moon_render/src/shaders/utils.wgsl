#define_import_path bevy_moon::utils

const antialias_threshold: f32 = 0.5;

const grid_bases: vec4<u32> = vec4(0u, 1u, 2u, 0u);

// const quad_vertices: array<vec2<f32>, 6> = array(
//   vec2(-0.5, -0.5), // 0 - BottomLeft
//   vec2( 0.5,  0.5), // 1 - TopRight
//   vec2(-0.5,  0.5), // 2 - TopLeft
//   vec2(-0.5, -0.5), // 3 - BottomLeft
//   vec2( 0.5, -0.5), // 4 - BottomRight
//   vec2( 0.5,  0.5), // 5 - TopRight
// );
const quad_vertices: array<vec2<f32>, 4> = array(
    vec2(-0.5, -0.5), // 0 - BottomLeft
    vec2( 0.5, -0.5), // 1 - BottomRight
    vec2( 0.5,  0.5), // 2 - TopRight
    vec2(-0.5,  0.5), // 3 - TopLeft
);

fn get_vertex_by_index(index: u32) -> vec2<f32> {
    return quad_vertices[index];
}

fn get_corner_index(point: vec2<f32>) -> u32 {
    let s = sign(point);
    let c = select(grid_bases.x, grid_bases.y, s.x == s.y);
    let r = select(grid_bases.z, grid_bases.w, s.y == 1.0);
    return c + r;
}

fn get_inset_by_index(insets: vec4<f32>, index: u32) -> vec2<f32> {
    return array(insets.wx, insets.yx, insets.yz, insets.wz)[index];
}

fn is_xyzw_zero(v: vec4<f32>) -> bool {
    return all(v == vec4(0.0));
}

// anti-aliasing width without `fwidth`
fn antialias(distance: f32) -> f32 {
    return saturate(antialias_threshold - distance);
}

fn antialias_f(distance: f32) -> f32 {
    let aa_width = fwidth(distance);
    let t = 1.0 - smoothstep(-aa_width, aa_width, distance); 
    return t;
}

fn antialias_alpha(alpha: f32, distance: f32) -> f32 {
    let t = antialias(distance);
    // let t = antialias_f(distance);
    return saturate(alpha * t);
}

fn over(below: vec4<f32>, above: vec4<f32>) -> vec4<f32> {
    let d = 1.0 - above.a;
    let alpha = above.a + below.a * d;
    let color = (above.rgb * above.a + below.rgb * below.a * d) / alpha;
    return vec4(color, alpha);
}
