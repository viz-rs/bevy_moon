use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Default)]
pub struct UiInstance {
    pub position: [f32; 3],
    pub color: [f32; 4],
    pub size: [f32; 2],
    pub flags: u32,
    pub corner_radii: [f32; 4],
    pub border_widths: [f32; 4],
    pub border_color: [f32; 4],
}
