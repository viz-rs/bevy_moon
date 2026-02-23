use std::{marker::PhantomData, ops::Range};

use bevy_asset::AssetId;
use bevy_ecs::{component::Component, entity::Entity, resource::Resource};
use bevy_image::Image;
use bevy_platform::collections::HashMap;
use bevy_render::{
    render_resource::{BindGroup, BufferUsages, RawBufferVec},
    sync_world::MainEntity,
};
use bytemuck::{NoUninit, Pod, Zeroable};

pub mod primitive;
pub mod shadow;

#[derive(Resource)]
pub struct UiMeta<T>
where
    T: NoUninit + Pod + Zeroable,
{
    pub instance_buffer: RawBufferVec<T>,
}

impl<T> Default for UiMeta<T>
where
    T: NoUninit + Pod + Zeroable,
{
    fn default() -> Self {
        Self {
            instance_buffer: RawBufferVec::new(BufferUsages::VERTEX),
        }
    }
}

#[derive(Component, Debug)]
pub struct UiBatch<T> {
    pub range: Range<u32>,
    pub texture: AssetId<Image>,
    _marker: PhantomData<T>,
}

impl<T> UiBatch<T> {
    pub fn new(range: Range<u32>) -> Self {
        Self {
            range,
            texture: AssetId::default(),
            _marker: PhantomData,
        }
    }

    pub fn with_texture(mut self, texture: AssetId<Image>) -> Self {
        self.texture = texture;
        self
    }
}

#[derive(Component)]
pub struct UiViewBindGroup<T> {
    pub value: BindGroup,
    _marker: PhantomData<T>,
}

impl<T> UiViewBindGroup<T> {
    pub fn new(value: BindGroup) -> Self {
        Self {
            value,
            _marker: PhantomData,
        }
    }
}

#[derive(Resource, Default)]
pub struct UiTextureBindGroups {
    pub values: HashMap<AssetId<Image>, BindGroup>,
}

pub struct ExtractedUiInstance<T> {
    pub index: f32,
    pub entity: (Entity, MainEntity),
    pub camera_entity: Entity,

    pub texture: AssetId<Image>,
    pub instance: T,
}

#[derive(Resource, Default)]
pub struct ExtractedUiInstances<T> {
    pub instances: Vec<ExtractedUiInstance<T>>,
}
