#define_import_path bevy_moon::rectangles

#import bevy_moon::corners::{sd_corner}

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
    let d = sd_corner(uv, 0u);
    
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
