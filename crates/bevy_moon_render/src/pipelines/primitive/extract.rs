use std::ops::Range;

use bevy_asset::AssetId;
use bevy_color::LinearRgba;
use bevy_ecs::{component::Component, entity::Entity, resource::Resource};
use bevy_image::Image;
use bevy_math::{Affine3A, Vec2, Vec4};
use bevy_moon_core::prelude::Corners;
use bevy_render::{
    render_resource::{BindGroup, BufferUsages, RawBufferVec},
    sync_world::MainEntity,
};
use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct UiInstance {
    pub position: [f32; 3],
    pub size: [f32; 2],
    pub color: [f32; 4],
    pub corner_radii: [f32; 4],
    pub border_widths: [f32; 4],
    pub border_color: [f32; 4],
}

#[derive(Resource)]
pub struct UiMeta {
    pub instance_buffer: RawBufferVec<UiInstance>,
}

impl Default for UiMeta {
    fn default() -> Self {
        Self {
            instance_buffer: RawBufferVec::new(BufferUsages::VERTEX),
        }
    }
}

#[derive(Component, Debug)]
pub struct UiBatch {
    pub range: Range<u32>,
}

#[derive(Component)]
pub struct UiViewBindGroup {
    pub value: BindGroup,
}

pub struct ExtractedUiInstance {
    pub index: f32,
    pub image: AssetId<Image>,
    pub entity: (Entity, MainEntity),
    pub camera_entity: Entity,

    pub size: Vec2,
    pub color: LinearRgba,
    pub affine: Affine3A,
    pub corner_radii: Corners<f32>,
    pub border_widths: Vec4,
    pub border_color: LinearRgba,
}

#[derive(Resource, Default)]
pub struct ExtractedUiInstances {
    pub instances: Vec<ExtractedUiInstance>,
}
