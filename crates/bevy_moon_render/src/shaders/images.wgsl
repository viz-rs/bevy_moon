#define_import_path bevy_moon::images

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
fn object_fit(
    uv: vec2<f32>,
    dst_size: vec2<f32>,
    src_size: vec2<f32>,
    object_position: vec2<f32>,
    mode: u32
) -> vec2<f32> {
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
    
    let out = (uv - object_position) * scale + object_position;
    
    // clear the overflow
    // TODO(@fundon): improve overflow handling
    if any(out < vec2(0.0)) || any(out > vec2(1.0)) {
        return vec2(0.0);
    }
    
    return out;
}
