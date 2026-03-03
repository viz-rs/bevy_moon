use std::fmt::Debug;

use bevy_derive::{Deref, DerefMut};
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

pub trait Measure: Send + Sync + DynClone + 'static {
    /// Calculates the size of the node given the constraints.
    fn measure(&mut self, args: MeasureArgs<'_>, style: &Style) -> Vec2;

    /// Gets the text buffer for the text node.
    fn get_text_buffer<'a>(
        &mut self,
        _: &'a mut Query<&mut ComputedTextBlock>,
    ) -> Option<&'a mut ComputedTextBlock> {
        None
    }
}

impl Clone for Box<dyn Measure> {
    fn clone(&self) -> Self {
        dyn_clone::clone_box(&**self)
    }
}

#[derive(Clone, Deref, DerefMut)]
pub struct NodeContext(StackSafe<Box<dyn Measure>>);

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

/// Always returns `fixed size` for the node.
#[derive(Clone)]
pub struct FixedMeasure {
    pub size: Vec2,
}

impl Measure for FixedMeasure {
    fn measure(&mut self, _: MeasureArgs, _: &Style) -> Vec2 {
        self.size
    }
}
