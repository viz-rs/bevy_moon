use bevy_ecs::{component::Component, prelude::ReflectComponent};
use bevy_math::{Affine3A, Vec2, Vec4};
use bevy_reflect::{Reflect, prelude::ReflectDefault};
use taffy::Layout;

/// Provides the computed size and layout properties of the node.
#[derive(Component, Debug, Copy, Clone, PartialEq, Reflect)]
#[reflect(Component, Default, Debug, PartialEq, Clone)]
pub struct ComputedLayout {
    /// The top-left corner of the node.
    pub location: Vec2,

    /// The width and height of the node.
    pub size: Vec2,

    /// The border widths of the node.
    pub border_widths: Vec4,

    /// This is the affine of the node relative to its parent.
    /// Stores it for inversion.
    pub affine: Affine3A,
}

impl Default for ComputedLayout {
    fn default() -> Self {
        Self {
            location: Vec2::ZERO,
            size: Vec2::ZERO,
            border_widths: Vec4::ZERO,

            affine: Affine3A::IDENTITY,
        }
    }
}

impl ComputedLayout {
    pub fn update(&mut self, layout: Layout) {
        use crate::geometry::Convert;

        let Layout {
            location,
            size,
            border,
            ..
        } = layout;

        self.location = location.convert();
        self.size = size.convert();
        self.border_widths = border.convert();
    }
}
