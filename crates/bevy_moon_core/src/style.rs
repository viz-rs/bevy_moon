use std::fmt::Debug;

use bevy_color::Color;
use bevy_math::Vec2;
use bevy_reflect::{Reflect, prelude::ReflectDefault};

#[derive(Clone, Copy, Debug, Default, PartialEq, Reflect)]
#[reflect(Clone, Default, PartialEq)]
pub struct Corners<T>
where
    T: Clone + Copy + Debug + Default + PartialEq + Reflect,
{
    pub top_left: T,
    pub top_right: T,
    pub bottom_right: T,
    pub bottom_left: T,
}

impl<T> Corners<T>
where
    T: Clone + Copy + Debug + Default + PartialEq + Reflect,
{
    #[inline]
    pub const fn all(value: T) -> Self {
        Self {
            top_left: value,
            top_right: value,
            bottom_right: value,
            bottom_left: value,
        }
    }

    #[inline]
    pub const fn top_left(self, value: T) -> Self {
        Self {
            top_left: value,
            ..self
        }
    }

    #[inline]
    pub const fn top_right(self, value: T) -> Self {
        Self {
            top_right: value,
            ..self
        }
    }

    #[inline]
    pub const fn bottom_right(self, value: T) -> Self {
        Self {
            bottom_right: value,
            ..self
        }
    }

    #[inline]
    pub const fn bottom_left(self, value: T) -> Self {
        Self {
            bottom_left: value,
            ..self
        }
    }

    #[inline]
    pub fn map<R, F>(self, f: F) -> Corners<R>
    where
        R: Clone + Copy + Debug + Default + PartialEq + Reflect,
        F: Fn(T) -> R,
    {
        Corners {
            top_left: f(self.top_left),
            top_right: f(self.top_right),
            bottom_right: f(self.bottom_right),
            bottom_left: f(self.bottom_left),
        }
    }
}

impl Corners<f32> {
    pub const DEFAULT: Self = Self::all(0.0);

    const fn resolve_single(radius: f32, min_length: f32) -> f32 {
        radius.clamp(0.0, 0.5 * min_length)
    }
}

impl From<Corners<f32>> for [f32; 4] {
    #[inline]
    fn from(value: Corners<f32>) -> Self {
        [
            value.top_left,
            value.top_right,
            value.bottom_right,
            value.bottom_left,
        ]
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Reflect)]
#[reflect(Clone, Default, PartialEq)]
pub struct BoxShadow {
    pub color: Color,
    pub offset: Vec2,
    pub blur_radius: f32,
    pub spread_radius: f32,
}
