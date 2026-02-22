use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Default)]
pub struct UiShadow {
    pub position: [f32; 3],
    pub size: [f32; 2],
    pub color: [f32; 4],
    pub corner_radii: [f32; 4],
    pub blur_radius: f32,
}
