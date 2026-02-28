use std::ops::{Deref, DerefMut};

use bevy_ecs::{component::Component, reflect::ReflectComponent};
use bevy_math::Vec2;
use bevy_reflect::{Reflect, prelude::ReflectDefault};

use crate::measure::{FixedMeasure, Measure, NodeContext};

/// A node with a `ContentSize` component is a node where its size
/// is based on its content.
#[derive(Component, Reflect, Default)]
#[reflect(Component, Default)]
pub struct ContentSize {
    /// The `Measure` used to compute the intrinsic size
    #[reflect(ignore, clone)]
    pub(crate) measure: Option<NodeContext>,
}

impl Deref for ContentSize {
    type Target = Option<NodeContext>;

    fn deref(&self) -> &Self::Target {
        &self.measure
    }
}

impl DerefMut for ContentSize {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.measure
    }
}

impl ContentSize {
    /// Set a `Measure` for the UI node entity with this component
    pub fn set<M>(&mut self, measure: M)
    where
        M: Measure + Send + Sync + Clone + 'static,
    {
        self.measure = Some(NodeContext::new(measure));
    }

    /// Creates a `ContentSize` with a `Measure` that always returns given `size` argument, regardless of the UI layout's constraints.
    pub fn fixed_size(size: Vec2) -> Self {
        Self {
            measure: Some(NodeContext::new(FixedMeasure { size })),
        }
    }

    /// Take the `Measure` from the `ContentSize` component.
    pub fn take(&mut self) -> Option<NodeContext> {
        self.measure.take()
    }
}
