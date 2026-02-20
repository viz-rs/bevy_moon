#import bevy_render::maths::PI

// A standard gaussian function, used for weighting samples
fn gaussian(x: f32, sigma: f32) -> f32 {
    return exp(-(x * x) / (2.0 * sigma * sigma)) / (sqrt(2.0 * PI) * sigma);
}

// This approximates the error function, needed for the gaussian integral
// 
// https://en.wikipedia.org/wiki/Error_function
fn erf(v: vec2<f32>) -> vec2<f32> {
    let s = sign(v);
    let a = abs(v);
    // fourth degree polynomial approximation for erf
    var r = 1.0 + (0.278393 + (0.230389 + (0.000972 + 0.078108 * a) * a) * a) * a;
    r *= r;
    r *= r;
    return s - s / r;
}
