// The MIT License
// Copyright © Viktor Chlumský
// <https://github.com/Chlumsky/msdfgen>
// 
// The Apache-2.0 License
// Copyright © Amit Patel
// <https://www.redblobgames.com/articles/sdf-fonts/>

// Computation of the median value using minima and maxima.
fn median(r: f32, g: f32, b: f32) -> f32 {
    return max(min(r, g), min(max(r, g), b));
}

fn contour(d: f32, w: f32) -> f32 {
    return smoothstep(0.5 - w, 0.5 + w, d);
}

// float3 sample = atlas.sample(colorSampler, in.texCoord).rgb;
// float msdf = median3(sample.r, sample.g, sample.b);
// float2 screenTexSize = 1.0f / fwidth(in.texCoord);
// float screenPxRange = max(0.5f * dot(uni.unitRange, screenTexSize), 1.0f);
// float screenPxDistance = screenPxRange * (msdf - 0.5f);
// float alphaFill = clamp(screenPxDistance + 0.5f, 0.0f, 1.0f);
// float4 color = uni.textColor;
// color.a *= alphaFill;
// return color;

 // mix(msdf_texel, texture(u_atlas, v_st + u_glow_offset).a, u_sdf_in_alpha_channel);

 // thickness
 // 
// plain
// outline
// shadow
// outline shadow
// outer glow
// inner glow
// both glow
// striped
// landscape
// overload

//fn main() {
//    // Bilinear sampling of the distance field
//    let s = textureSample(atlas_texture, atlas_sampler).rgb;
//    // Acquiring the signed distance
//    let d = median(s.r, s.g, s.b) - 0.5;
//    // The anti-aliased measure of how "inside" the fragment lies
//    let w = clamp(d / fwidth(d) + 0.5, 0.0, 1.0);
//    // Combining the two colors
//    let final_color = mix(outside_color, inside_color, w);
//    return final_color;
//}
