use std::ops::{Deref, DerefMut};

use bevy_ecs::{component::Component, reflect::ReflectComponent};
use bevy_reflect::{Reflect, prelude::ReflectDefault};

use crate::measure::{Measure, NodeContext};

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
}
