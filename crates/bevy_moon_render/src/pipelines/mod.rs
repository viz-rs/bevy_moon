use std::{marker::PhantomData, ops::Range};

use bevy_asset::AssetId;
use bevy_ecs::{component::Component, entity::Entity, resource::Resource};
use bevy_image::Image;
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
    _marker: PhantomData<T>,
}

impl<T> UiBatch<T> {
    pub fn new(range: Range<u32>) -> Self {
        Self {
            range,
            _marker: PhantomData,
        }
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

pub struct ExtractedUiInstance<T> {
    pub index: f32,
    pub image: AssetId<Image>,
    pub entity: (Entity, MainEntity),
    pub camera_entity: Entity,

    pub instance: T,
}

#[derive(Resource, Default)]
pub struct ExtractedUiInstances<T> {
    pub instances: Vec<ExtractedUiInstance<T>>,
}
