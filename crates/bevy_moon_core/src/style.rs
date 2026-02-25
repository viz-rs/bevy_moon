use std::{fmt::Debug, ops::Deref};

use bevy_color::Color;
use bevy_math::Vec2;
use bevy_reflect::{Reflect, prelude::ReflectDefault};

/// Rounds the corners of an element's outer border edge.
///
/// <https://developer.mozilla.org/ocs/Web/CSS/Reference/Properties/border-radius>
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

    /// Converts `self` to `[top_left, top_right, bottom_right, bottom_left]`.
    #[inline]
    pub const fn to_array(self) -> [T; 4] {
        [
            self.top_left,
            self.top_right,
            self.bottom_right,
            self.bottom_left,
        ]
    }
}

impl Corners<f32> {
    pub const DEFAULT: Self = Self::all(0.0);

    const fn resolve_single(radius: f32, min_length: f32) -> f32 {
        radius.clamp(0.0, 0.5 * min_length)
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

/// Sets the box shadow of the element.
/// [Docs](https://tailwindcss.com/docs/box-shadow)
impl BoxShadow {
    pub const XS2: [Self; 1] = [Self {
        color: Color::srgba(0.0, 0.0, 0.0, 0.05),
        offset: Vec2::new(0.0, 1.0),
        blur_radius: 0.0,
        spread_radius: 0.0,
    }];

    pub const XS: [Self; 1] = [Self {
        color: Color::srgba(0.0, 0.0, 0.0, 0.05),
        offset: Vec2::new(0.0, 1.0),
        blur_radius: 2.0,
        spread_radius: 0.0,
    }];

    pub const SM: [Self; 2] = [
        Self {
            color: Color::srgba(0.0, 0.0, 0.0, 0.1),
            offset: Vec2::new(0.0, 1.0),
            blur_radius: 3.0,
            spread_radius: 0.0,
        },
        Self {
            color: Color::srgba(0.0, 0.0, 0.0, 0.1),
            offset: Vec2::new(0.0, 1.0),
            blur_radius: 2.0,
            spread_radius: -1.0,
        },
    ];

    pub const MD: [Self; 2] = [
        Self {
            color: Color::srgba(0.0, 0.0, 0.0, 0.1),
            offset: Vec2::new(0.0, 4.0),
            blur_radius: 6.0,
            spread_radius: -1.0,
        },
        Self {
            color: Color::srgba(0.0, 0.0, 0.0, 0.1),
            offset: Vec2::new(0.0, 2.0),
            blur_radius: 4.0,
            spread_radius: -2.0,
        },
    ];

    pub const LG: [Self; 2] = [
        Self {
            color: Color::srgba(0.0, 0.0, 0.0, 0.1),
            offset: Vec2::new(0.0, 10.0),
            blur_radius: 15.0,
            spread_radius: -3.0,
        },
        Self {
            color: Color::srgba(0.0, 0.0, 0.0, 0.1),
            offset: Vec2::new(0.0, 4.0),
            blur_radius: 6.0,
            spread_radius: -4.0,
        },
    ];

    pub const XL: [Self; 2] = [
        Self {
            color: Color::srgba(0.0, 0.0, 0.0, 0.1),
            offset: Vec2::new(0.0, 20.0),
            blur_radius: 25.0,
            spread_radius: -5.0,
        },
        Self {
            color: Color::srgba(0.0, 0.0, 0.0, 0.1),
            offset: Vec2::new(0.0, 8.0),
            blur_radius: 10.0,
            spread_radius: -6.0,
        },
    ];

    pub const XL2: [Self; 1] = [Self {
        color: Color::srgba(0.0, 0.0, 0.0, 0.25),
        offset: Vec2::new(0.0, 25.0),
        blur_radius: 50.0,
        spread_radius: -12.0,
    }];
}

/// How an image should fit within its container.
///
/// ```text
/// uv = (uv - center) / scale + center
/// ```
///
/// ## Examples
///
/// ```text
/// uv = (uv - TopCenter) / scale + TopCenter
/// ```
///
/// <https://developer.mozilla.org/docs/Web/CSS/Reference/Properties/object-fit>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
pub enum ObjectFit {
    /// Stretch to fill the container (ignores aspect ratio)
    Fill,
    /// Fit entirely within the container (maintains aspect ratio, may letterbox)
    Contain,
    /// Fill the container completely, cropping if necessary (maintains aspect ratio)
    #[default]
    Cover,
    /// Scale down only if larger than container (maintains aspect ratio)
    ScaleDown,
    /// No scaling, maintain its original size
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Default, Reflect)]
pub struct ObjectPosition(Vec2);

/// How to align an image within its container.
///
/// <https://developer.mozilla.org/docs/Web/CSS/Reference/Properties/object-position>
impl ObjectPosition {
    pub const TOP_LEFT: Self = Self(Vec2::ZERO);
    pub const TOP_CENTER: Self = Self(Vec2::new(0.5, 0.0));
    pub const TOP_RIGHT: Self = Self(Vec2::X);
    pub const CENTER_LEFT: Self = Self(Vec2::new(0.0, 0.5));
    pub const CENTER: Self = Self(Vec2::new(0.5, 0.5));
    pub const CENTER_RIGHT: Self = Self(Vec2::new(1.0, 0.5));
    pub const BOTTOM_LEFT: Self = Self(Vec2::Y);
    pub const BOTTOM_CENTER: Self = Self(Vec2::new(0.5, 1.0));
    pub const BOTTOM_RIGHT: Self = Self(Vec2::ONE);
}

impl From<Vec2> for ObjectPosition {
    fn from(value: Vec2) -> Self {
        Self(value)
    }
}

impl From<ObjectPosition> for Vec2 {
    fn from(value: ObjectPosition) -> Self {
        value.0
    }
}

impl Deref for ObjectPosition {
    type Target = Vec2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
