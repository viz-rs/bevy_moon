use std::{
    ops::{Deref, DerefMut, Range},
    sync::Arc,
};

use bevy_ecs::{
    entity::{Entity, EntityHashMap},
    resource::Resource,
};
use bevy_render::extract_resource::ExtractResource;
use fixedbitset::FixedBitSet;
use smallvec::SmallVec;

#[derive(Default, Clone, Debug)]
pub struct UiStack {
    pub bitset: FixedBitSet,
    pub roots: SmallVec<[Entity; 8]>,
    pub entities: SmallVec<[Entity; 24]>,
    pub ranges: SmallVec<[Range<usize>; 16]>,
}

impl UiStack {
    pub fn clear(&mut self) {
        self.bitset.clear();
        self.roots.clear();
        self.entities.clear();
        self.ranges.clear();
    }
}

#[derive(Resource, Clone, Debug)]
pub struct UiStackMap(Arc<EntityHashMap<UiStack>>);

impl Default for UiStackMap {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl AsMut<EntityHashMap<UiStack>> for UiStackMap {
    fn as_mut(&mut self) -> &mut EntityHashMap<UiStack> {
        Arc::make_mut(&mut self.0)
    }
}

impl Deref for UiStackMap {
    type Target = EntityHashMap<UiStack>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for UiStackMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

impl ExtractResource for UiStackMap {
    type Source = Self;

    fn extract_resource(source: &Self::Source) -> Self {
        Self(Arc::clone(&source.0))
    }
}

impl UiStackMap {
    pub fn clear(&mut self) {
        self.as_mut().clear();
    }
}
