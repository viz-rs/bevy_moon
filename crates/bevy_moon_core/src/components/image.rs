use bevy_asset::Handle;
use bevy_color::Color;
use bevy_ecs::{component::Component, reflect::ReflectComponent};
use bevy_image::TRANSPARENT_IMAGE_HANDLE;
use bevy_reflect::{Reflect, prelude::ReflectDefault};

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
    };
}

pub fn image(handle: Handle<bevy_image::Image>) -> Image {
    Image {
        handle,
        ..Image::DEFAULT
    }
}
