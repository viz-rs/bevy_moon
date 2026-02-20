#define_import_path bevy_moon::utils

#import bevy_moon::corners::{sd_corner}

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

fn sd_rounded_box(point: vec2<f32>, half_size: vec2<f32>, radius: f32) -> f32 {
    let corner_to_point = abs(point) - half_size;
    let q = corner_to_point + radius;
    
    // distance to sides
    if min(q.x, q.y) < 0.0 || radius == 0.0 {
        return max(q.x, q.y) - radius;
    }
    
    // rotate 45 degrees, offset by r and scale by r*sqrt(0.5) to canonical corner coordinates
    let uv = vec2(abs(q.x - q.y), q.x + q.y - radius) / radius;
    
    // distance to corner
    let d = sd_corner(uv, 0u) ;
    
    // undo scale and return
    return d * radius * sqrt(0.5);
}

fn sd_inset_rounded_box(point: vec2<f32>, half_size: vec2<f32>, radius: f32, tl: vec2<f32>, br: vec2<f32>, cb: vec2<f32>) -> f32 {
    let inner_half_size = half_size - 0.5 * (tl + br);
    let inner_center = tl + inner_half_size - half_size;
    let inner_point = point - inner_center * vec2(1.0, -1.0); // Flip Y
    let min_size = min(inner_half_size.x, inner_half_size.y);
    
    var r = radius;
    r -= max(cb.x, cb.y);
    r = min(max(r, 0.0), min_size);
    
    return sd_rounded_box(inner_point, inner_half_size, r);
}

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
