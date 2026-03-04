use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct UiAtlas {
    /// A `[[f32; 4]; 4]` 3D array storing data in column major order (4Cx4R).
    ///
    /// Sees [`bevy_math::Affine3A::to_cols_array_2d`].
    pub matrix: [[f32; 4]; 4],

    pub color: [f32; 4],
    pub size: [f32; 2],
    pub corner_radii: [f32; 4],

    /// | Type  | Data                                                     |
    /// | ----- | -------------------------------------------------------- |
    /// | Glyph | `[flags, scale, left, top]`                              |
    /// | Image | `[flags, ObjectFit, ObjectPosition.x, ObjectPosition.y]` |
    pub extra: [f32; 4],
    pub flipped: [u32; 2],
}

impl Default for UiAtlas {
    fn default() -> Self {
        Self::IMAGE
    }
}

impl UiAtlas {
    /// The `image` instance.
    pub const IMAGE: Self = Self {
        matrix: [[0.0; 4]; 4],
        color: [0.0; 4],
        size: [0.0; 2],
        corner_radii: [0.0; 4],
        extra: [0.0; 4],
        flipped: [0; 2],
    };

    /// The `text` instance.
    pub const TEXT: Self = Self {
        extra: [1.0, 0.0, 0.0, 0.0],
        ..Self::IMAGE
    };
}
