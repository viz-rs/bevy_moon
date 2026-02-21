#import bevy_render::maths::PI

// A standard gaussian function, used for weighting samples
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

// fn compute_erf7(x: f64) -> f64 {
//     let x = x * std::f64::consts::FRAC_2_SQRT_PI;
//     let xx = x * x;
//     let x = x + (0.24295 + (0.03395 + 0.0104 * xx) * xx) * (x * xx);
//     x / (1.0 + x * x).sqrt()
// }

fn horizontal_rounded_box_shadow(point: vec2<f32>, half_size: vec2<f32>, radius: f32, blur_radius: f32) -> f32 {
    let x = point.x;
    let y = point.y;
    let d = min(half_size.y - radius - abs(y), 0.0);
    let c = half_size.x - radius + sqrt(max(0.0, radius * radius - d * d));
    let integral = 0.5 + 0.5 * erf((x + vec2(-c, c)) * (sqrt(0.5) / blur_radius));
    return integral.y - integral.x;
}

fn rounded_box_shadow(
    lower: vec2<f32>,
    upper: vec2<f32>,
    point: vec2<f32>,
    blur: f32,
    radius: vec4<f32>,
) -> f32 {
    let center = (lower + upper) * 0.5;
    let half_size = (upper - lower) * 0.5;
    let p = point - center;
    let low = p.y - half_size.y;
    let high = p.y + half_size.y;
    let start = clamp(-3. * blur, low, high);
    let end = clamp(3. * blur, low, high);
    let step = (end - start) / f32(SAMPLES);
    var y = start + step * 0.5;
    var value: f32 = 0.0;
    for (var i = 0; i < SAMPLES; i++) {
        value += horizontalRoundedBoxShadow(p.x, p.y - y, half_size, radius, blur) * gaussian(y, blur) * step;
        y += step;
    }
    return value;
}
