use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct UiInstance {
    // TODO(@fundon): sets them to [f32; 4] if need to align
    pub position: [f32; 3],
    pub x_axis: [f32; 3],
    pub y_axis: [f32; 3],
    pub z_axis: [f32; 3],

    pub color: [f32; 4],
    pub size: [f32; 2],
    pub flags: u32,
    pub corner_radii: [f32; 4],
    pub border_color: [f32; 4],
    pub border_widths: [f32; 4],

    // glyph: [left, top, scale]
    // image: [ObjectPosition.x, ObjectPosition.y, ObjectFit]
    pub extra: [f32; 3],
    pub flip: [u32; 2],
}

impl Default for UiInstance {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl UiInstance {
    pub const DEFAULT: Self = Self {
        position: [0.0; 3],
        x_axis: [0.0; 3],
        y_axis: [0.0; 3],
        z_axis: [0.0; 3],
        color: [0.0; 4],
        size: [0.0; 2],
        flags: 0,
        corner_radii: [0.0; 4],
        border_color: [0.0; 4],
        border_widths: [0.0; 4],
        extra: [0.0; 3],
        flip: [0; 2],
    };
}
