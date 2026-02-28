#define_import_path bevy_moon::atlas

const FILL       = 0u;
const CONTAIN    = 1u;
const COVER      = 2u;
const SCALE_DOWN = 3u;

/// How an image should fit within its container.
///
/// ```text
/// uv = (uv - center) / scale + center
/// ```
///
/// <https://developer.mozilla.org/docs/Web/CSS/Reference/Properties/object-fit>
/// <https://developer.mozilla.org/docs/Web/CSS/Reference/Properties/object-position>
fn object_fit(uv: vec2<f32>, dst_size: vec2<f32>, src_size: vec2<f32>, center: vec2<f32>, mode: u32) -> vec2<f32> {
    let ratio = dst_size / src_size;
    var scale = ratio;

    switch mode {
        // None is by default
        default: {
            // scale = ratio;
            // do nothing
        }
        case FILL: {
            // scale = vec2(1.0);
            scale /= ratio;
        }
        case CONTAIN: {
            // scale = ratio / min_ratio
            scale /= min(ratio.x, ratio.y);
        }
        case COVER: {
           // scale = ratio / max_ratio
            scale /= max(ratio.x, ratio.y);
        }
        case SCALE_DOWN: {
            // scale = ratio / min(min_ratio, 1.0)
            scale /= min(min(ratio.x, ratio.y), 1.0);
        }
    }
 
    var out = (uv - center) * scale + center;
 
    // overflow handling
    if (any(out < vec2(0.0)) | any(out > vec2(1.0))) {
        return vec2(0.0);
    }

    return out;
}

/// Calculates a glyph tile's uv.
///
/// ```text
/// uv = (glyph_top_left + uv * glyph_size) / texture_size
/// uv = top_left / src_size + uv * scale
/// ```
fn glyph_tile_uv(uv: vec2<f32>, dst_size: vec2<f32>, src_size: vec2<f32>, top_left: vec2<f32>) -> vec2<f32> {
    let scale = dst_size / src_size;
    return uv * scale + top_left / src_size;
}

/// Flips a UV coordinate based on the flip vector.
fn flip_uv(uv: vec2<f32>, flip: vec2<u32>) -> vec2<f32> {
    return select(uv, vec2(1.0 - uv.x, 1.0 - uv.y), flip == vec2(1, 1));
}
