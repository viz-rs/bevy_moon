use bevy_derive::{Deref, DerefMut};
use bevy_ecs::{component::Component, reflect::ReflectComponent};
use bevy_reflect::{Reflect, prelude::ReflectDefault};

use crate::measure::{Measure, NodeContext};

/// Defines or measure size for the contents of the node.
#[derive(Component, Reflect, Default, Deref, DerefMut)]
#[reflect(Component, Default)]
pub struct ContentSize {
    /// The `Measure` used to compute the intrinsic size.
    #[reflect(ignore, clone)]
    pub(crate) measure: Option<NodeContext>,
}

impl ContentSize {
    /// Sets a `Measure` for the current node.
    #[inline]
    pub fn set<M>(&mut self, measure: M)
    where
        M: Measure + Send + Sync + Clone + 'static,
    {
        self.measure = Some(NodeContext::new(measure));
    }
}
