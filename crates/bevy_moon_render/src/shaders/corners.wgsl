// The MIT License
// Copyright © 2024 Inigo Quilez
// 
// <https://iquilezles.org/articles/roundedboxes/>

#define_import_path bevy_moon::corners

#import bevy_render::maths::PI_2

// Selects a corner shape by a kind
//
// <https://www.shadertoy.com/view/4cG3R1>
fn sd_corner(uv: vec2<f32>, kind: u32) -> f32 {
    switch kind {
        case 1: { return sd_corner_parabola(uv); }
        case 2: { return sd_corner_cosine(uv); }
        case 3: { return sd_corner_cubic(uv); }
        default: { return sd_corner_circle(uv); }
    }
}

// Traditional Circle rounded box
fn sd_corner_circle(uv: vec2<f32>) -> f32 {
    return length(uv - vec2(0.0, -1.0)) - sqrt(2.0);
}

// New Parabolic rounded box
//
// <https://www.shadertoy.com/view/ws3GD7>
fn sd_corner_parabola(uv: vec2<f32>) -> f32 {
    let y = (0.5 + uv.y) * (2.0 / 3.0);
    let h = uv.x * uv.x + y * y * y;
    let w = pow(uv.x + sqrt(abs(h)), 1.0 / 3.0); // note I allow a tiny error in the very interior of the shape so that I don't have to branch into the 3 root solution
    let x = w - y / w;
    let q = vec2(x, 0.5 * (1.0 - x * x));
    return length(uv - q) * sign(uv.y - q.y);
}

// New Cosine rounded box
//
// <https://www.shadertoy.com/view/3t23WG>
fn sd_corner_cosine(uv: vec2<f32>) -> f32 {
    var ta = 0.0; 
    var tb = PI_2 / 4.0;
    let p = uv * tb;

    for (var i = 0; i < 8; i++ ) {
        let t = 0.5 * (ta + tb);
        let y = t - p.x + (p.y - cos(t)) * sin(t);
        if (y < 0.0) {
          ta = t;
        } else {
          tb = t;
        }
    }
    
    let qa = vec2(ta, cos(ta));
    let qb = vec2(tb, cos(tb));
    let pa = p - qa;
    let di = qb - qa;
    let h = clamp(dot(pa, di) / dot(di, di), 0.0, 1.0);
    return length(pa - di * h) * sign(pa.y * di.x - pa.x * di.y) * (4.0 / PI_2);
}

// New Cubic rounded box
//
// <https://www.shadertoy.com/view/4fyGz1>
fn sd_corner_cubic(uv: vec2<f32>) -> f32 {
    var ta = 0.0; 
    var tb = 1.0;
    
    for (var i = 0; i < 12; i++ ) {
        let t = 0.5 * (ta + tb);
        let c = (t * t * (t - 3.0) + 2.0) / 3.0;
        let dc = t * (t - 2.0);
        let y = (uv.x - t) + (uv.y - c) * dc;
        if (y > 0.0 ) {
          ta = t;
        } else {
          tb = t;
        }
    }
    
    let qa = vec2(ta, (ta * ta * (ta - 3.0) + 2.0) / 3.0);
    let qb = vec2(tb, (tb * tb * (tb - 3.0) + 2.0) / 3.0);
    let pa = uv - qa;
    let di = qb - qa;
    let h = clamp(dot(pa, di) / dot(di, di), 0.0, 1.0);
    return length(pa - di * h) * sign(pa.y * di.x - pa.x * di.y);
}
