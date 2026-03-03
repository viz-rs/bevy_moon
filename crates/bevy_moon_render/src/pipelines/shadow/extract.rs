use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct UiShadow {
    /// A `[[f32; 3]; 4]` 3D array storing data in column major order (3Cx4R).
    ///
    /// Sees [`bevy_math::Affine3A::to_cols_array_2d`].
    pub matrix: [[f32; 3]; 4],

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
        matrix: [[0.0; 3]; 4],
        color: [0.0; 4],
        size: [0.0; 2],
        corner_radii: [0.0; 4],
        blur_radius: 0.0,
    };
}
