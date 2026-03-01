use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct UiShadow {
    // TODO(@fundon): sets them to [f32; 4] if need to align
    pub position: [f32; 3],
    pub x_axis: [f32; 3],
    pub y_axis: [f32; 3],
    pub z_axis: [f32; 3],

    pub color: [f32; 4],
    pub size: [f32; 2],
    pub corner_radii: [f32; 4],
    pub blur_radius: f32,
}

impl Default for UiShadow {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl UiShadow {
    pub const DEFAULT: Self = Self {
        position: [0.0; 3],
        x_axis: [0.0; 3],
        y_axis: [0.0; 3],
        z_axis: [0.0; 3],
        color: [0.0; 4],
        size: [0.0; 2],
        corner_radii: [0.0; 4],
        blur_radius: 0.0,
    };
}
