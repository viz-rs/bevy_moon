use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct UiQuad {
    /// A `[[f32; 4]; 4]` 3D array storing data in column major order (4Cx4R).
    ///
    /// Sees [`bevy_math::Affine3A::to_cols_array_2d`].
    pub matrix: [[f32; 4]; 4],

    pub color: [f32; 4],
    pub size: [f32; 2],
    pub corner_radii: [f32; 4],
    pub border_color: [f32; 4],
    pub border_widths: [f32; 4],
}

impl Default for UiQuad {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl UiQuad {
    /// The default value of the quad instance.
    pub const DEFAULT: Self = Self {
        matrix: [[0.0; 4]; 4],
        color: [0.0; 4],
        size: [0.0; 2],
        corner_radii: [0.0; 4],
        border_color: [0.0; 4],
        border_widths: [0.0; 4],
    };
}
