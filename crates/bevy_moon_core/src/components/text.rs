use bevy_derive::{Deref, DerefMut};
use bevy_ecs::{component::Component, reflect::ReflectComponent};
use bevy_reflect::{Reflect, prelude::ReflectDefault};
use bevy_text::{
    FontHinting, LineHeight, TextColor, TextFont, TextLayout, TextRoot, TextSpanAccess,
};

use super::div::Div;

#[derive(Component, Debug, Default, Clone, Deref, DerefMut, Reflect, PartialEq)]
#[reflect(Component, Default, Debug, PartialEq, Clone)]
#[require(
    Div,
    TextLayout,
    TextFont,
    TextColor,
    LineHeight,
    TextFlags,
    // ContentSize,
    // Hinting is enabled by default as UI text is normally pixel.
    FontHinting::Enabled
)]
pub struct Text(pub String);

impl Text {
    /// Makes a new text component.
    pub fn new(text: impl Into<String>) -> Self {
        Self(text.into())
    }
}

impl TextRoot for Text {}

impl TextSpanAccess for Text {
    fn read_span(&self) -> &str {
        self.as_str()
    }
    fn write_span(&mut self) -> &mut String {
        &mut *self
    }
}

impl From<&str> for Text {
    fn from(value: &str) -> Self {
        Self(String::from(value))
    }
}

impl From<String> for Text {
    fn from(value: String) -> Self {
        Self(value)
    }
}

/// UI text system flags.
///
/// Used internally by [`measure_text_system`] and [`text_system`] to schedule text for processing.
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component, Default, Debug, Clone)]
pub struct TextFlags {
    /// If set then a new measure function for the text node will be created.
    pub(crate) needs_measure_fn: bool,
    /// If set then the text will be recomputed.
    pub(crate) needs_recompute: bool,
}

impl Default for TextFlags {
    fn default() -> Self {
        Self {
            needs_measure_fn: true,
            needs_recompute: true,
        }
    }
}

pub fn text(value: impl Into<String>) -> Text {
    Text::new(value)
}
