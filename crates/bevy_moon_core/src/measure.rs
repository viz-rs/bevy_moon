use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use bevy_ecs::system::Query;
use bevy_math::Vec2;
use bevy_text::{ComputedTextBlock, FontCx};
use dyn_clone::DynClone;
use stacksafe::StackSafe;
use taffy::{AvailableSpace, Size, Style};

pub struct MeasureArgs<'a> {
    pub known_dimensions: Size<Option<f32>>,
    pub available_space: Size<AvailableSpace>,
    pub font_system: &'a mut FontCx,
    pub text_buffer: Option<&'a mut ComputedTextBlock>,
}

/// A `Measure` is used to compute the size of a ui node
/// when the size of that node is based on its content.
pub trait Measure: Send + Sync + DynClone + 'static {
    /// Calculate the size of the node given the constraints.
    fn measure(&mut self, args: MeasureArgs<'_>, style: &Style) -> Vec2;

    /// Calculate the text buffer for the text node.
    fn get_text_buffer<'a>(
        &mut self,
        _: &'a mut Query<&mut ComputedTextBlock>,
    ) -> Option<&'a mut ComputedTextBlock> {
        None
    }
}

pub struct NodeContext(StackSafe<Box<dyn Measure>>);

impl Deref for NodeContext {
    type Target = StackSafe<Box<dyn Measure>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for NodeContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Clone for NodeContext {
    fn clone(&self) -> Self {
        dyn_clone::clone(self)
    }
}

impl Debug for NodeContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NodeContxt").finish()
    }
}

impl NodeContext {
    pub fn new<M>(measure: M) -> Self
    where
        M: Measure + Send + Sync + 'static,
    {
        Self(StackSafe::new(Box::new(measure)))
    }
}
