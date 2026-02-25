use bevy_asset::Handle;
use bevy_color::Color;
use bevy_ecs::{component::Component, reflect::ReflectComponent};
use bevy_image::TRANSPARENT_IMAGE_HANDLE;
use bevy_reflect::{Reflect, prelude::ReflectDefault};

use crate::style::{ObjectFit, ObjectPosition};

use super::div::Div;

#[derive(Component, Clone, Debug, Reflect)]
#[require(Div)]
#[reflect(Component, Clone, Debug, Default)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    reflect(Serialize, Deserialize)
)]
pub struct Image {
    pub color: Color,
    pub handle: Handle<bevy_image::Image>,
    pub object_fit: ObjectFit,
    pub object_position: ObjectPosition,
}

impl Default for Image {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl Image {
    pub const DEFAULT: Self = Self {
        color: Color::WHITE,
        handle: TRANSPARENT_IMAGE_HANDLE,
        object_fit: ObjectFit::Cover,
        object_position: ObjectPosition::CENTER,
    };

    pub fn object_fit_fill(self) -> Self {
        Self {
            object_fit: ObjectFit::Fill,
            ..self
        }
    }

    pub fn object_fit_contain(self) -> Self {
        Self {
            object_fit: ObjectFit::Contain,
            ..self
        }
    }

    pub fn object_fit_cover(self) -> Self {
        Self {
            object_fit: ObjectFit::Cover,
            ..self
        }
    }

    pub fn object_fit_none(self) -> Self {
        Self {
            object_fit: ObjectFit::None,
            ..self
        }
    }

    pub fn object_fit_scale_down(self) -> Self {
        Self {
            object_fit: ObjectFit::ScaleDown,
            ..self
        }
    }

    pub fn object_position(self, position: ObjectPosition) -> Self {
        Self {
            object_position: position,
            ..self
        }
    }
}

pub fn img(handle: Handle<bevy_image::Image>) -> Image {
    Image {
        handle,
        ..Image::DEFAULT
    }
}
