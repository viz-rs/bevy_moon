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
pub struct Image {
    pub color: Color,
    pub handle: Handle<bevy_image::Image>,
    pub object_fit: ObjectFit,
    pub object_position: ObjectPosition,
    pub flipped: [bool; 2],
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
        flipped: [false; 2],
    };

    pub fn object_fit_fill(mut self) -> Self {
        self.object_fit = ObjectFit::Fill;
        self
    }

    pub fn object_fit_contain(mut self) -> Self {
        self.object_fit = ObjectFit::Contain;
        self
    }

    pub fn object_fit_cover(mut self) -> Self {
        self.object_fit = ObjectFit::Cover;
        self
    }

    pub fn object_fit_none(mut self) -> Self {
        self.object_fit = ObjectFit::None;
        self
    }

    pub fn object_fit_scale_down(mut self) -> Self {
        self.object_fit = ObjectFit::ScaleDown;
        self
    }

    pub fn object_position(mut self, position: ObjectPosition) -> Self {
        self.object_position = position;
        self
    }

    pub fn flip_x(mut self) -> Self {
        self.flipped[0] ^= true;
        self
    }

    pub fn flip_y(mut self) -> Self {
        self.flipped[1] ^= true;
        self
    }
}

pub fn img(handle: Handle<bevy_image::Image>) -> Image {
    Image {
        handle,
        ..Image::DEFAULT
    }
}
