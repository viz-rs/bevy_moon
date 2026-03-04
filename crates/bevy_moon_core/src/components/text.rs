use bevy_asset::Assets;
use bevy_camera::Camera;
use bevy_derive::{Deref, DerefMut};
use bevy_ecs::{
    change_detection::{DetectChanges, Mut},
    component::Component,
    entity::Entity,
    query::With,
    reflect::ReflectComponent,
    schedule::SystemSet,
    system::{Query, Res, ResMut},
    world::Ref,
};
use bevy_math::Vec2;
use bevy_reflect::{Reflect, prelude::ReflectDefault};
use bevy_text::{
    ComputedTextBlock, Font, FontAtlasSet, FontCx, FontHinting, LayoutCx, LineBreak, LineHeight,
    RemSize, ScaleCx, TextBounds, TextColor, TextError, TextFont, TextLayout, TextLayoutInfo,
    TextMeasureInfo, TextPipeline, TextReader, TextRoot, TextSpanAccess,
};

use crate::{
    components::{computed::ComputedTargetInfo, content_size::ContentSize},
    measure::{FixedMeasure, Measure, MeasureArgs},
    prelude::ComputedLayout,
    stack::UiStackMap,
};

use super::div::Div;

#[derive(Component, Debug, Default, Clone, Deref, DerefMut, Reflect, PartialEq)]
#[reflect(Component, Default, Debug, PartialEq, Clone)]
#[require(
    Div,
    ContentSize,
    LineHeight,
    TextColor,
    TextFlags,
    TextFont,
    TextLayout,
    FontHinting::Enabled
)]
pub struct Text(pub String);

impl Text {
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

pub fn text(value: impl Into<String>) -> Text {
    Text::new(value)
}

#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component, Default, Debug, Clone)]
pub struct TextFlags {
    pub(crate) needs_measure_fn: bool,
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

/// Syncs [`TextMeasureInfo`] for measuring with text buffer.
#[derive(Clone, Copy)]
pub struct TextMeasure {
    pub min: Vec2,
    pub max: Vec2,
    pub entity: Entity,
    pub scale_factor: f32,
}

impl TextMeasure {
    #[inline]
    pub const fn needs_buffer(height: Option<f32>, available_width: taffy::AvailableSpace) -> bool {
        height.is_none() && matches!(available_width, taffy::AvailableSpace::Definite(_))
    }
}

impl Measure for TextMeasure {
    fn measure(&mut self, args: MeasureArgs<'_>, _style: &taffy::Style) -> Vec2 {
        let MeasureArgs {
            known_dimensions: taffy::Size { width, height },
            available_space:
                taffy::Size {
                    width: available_width,
                    ..
                },
            font_system,
            text_buffer,
        } = args;

        // it has been scaled in text layout engine
        let min = self.min;
        let max = self.max;
        let scale_factor = self.scale_factor;

        let scale_up_fn = |v| v * scale_factor;
        let scale_down_fn = |v| v * scale_factor.recip();

        // scales up when there are from taffy layout engine
        let width = width.map(scale_up_fn);
        let height = height.map(scale_up_fn);

        let x = match width {
            Some(x) => x,
            None => match available_width {
                taffy::AvailableSpace::MinContent => min.x,
                taffy::AvailableSpace::MaxContent => max.x,
                taffy::AvailableSpace::Definite(x) => {
                    // It is possible for the "min content width" to be larger than
                    // the "max content width" when soft-wrapping right-aligned text
                    // and possibly other situations.

                    scale_up_fn(x).max(min.x).min(max.x)
                }
            },
        };

        let size = match height {
            Some(y) => Vec2::new(x, y),
            None => match available_width {
                taffy::AvailableSpace::MinContent => Vec2::new(x, min.y),
                taffy::AvailableSpace::MaxContent => Vec2::new(x, max.y),
                taffy::AvailableSpace::Definite(_) => match text_buffer {
                    Some(buffer) => TextMeasureInfo {
                        min,
                        max,
                        entity: self.entity,
                    }
                    .compute_size(
                        TextBounds::new_horizontal(x),
                        buffer,
                        font_system,
                    ),
                    None => {
                        tracing::error!("text measure failed, buffer is missing");
                        Vec2::ZERO
                    }
                },
            },
        };

        // scales down when it be returned to taffy layout engine
        scale_down_fn(size).ceil()
    }

    fn get_text_buffer<'a>(
        &mut self,
        query: &'a mut Query<&mut ComputedTextBlock>,
    ) -> Option<&'a mut ComputedTextBlock> {
        query.get_mut(self.entity).map(Mut::into_inner).ok()
    }
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub(crate) struct AmbiguousWithText;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub(crate) struct AmbiguousWithUpdateText2dLayout;

pub fn measure_text_system(
    camera_query: Query<Ref<ComputedTargetInfo>, With<Camera>>,
    ui_stack_map: Res<UiStackMap>,
    fonts: Res<Assets<Font>>,
    rem_size: Res<RemSize>,
    mut text_query: Query<
        (
            Entity,
            Ref<Text>,
            Ref<TextLayout>,
            Ref<ComputedLayout>,
            &mut ContentSize,
            &mut TextFlags,
            &mut ComputedTextBlock,
        ),
        With<Div>,
    >,
    mut text_reader: TextReader<Text>,
    mut text_pipeline: ResMut<TextPipeline>,
    mut font_system: ResMut<FontCx>,
    mut layout_cx: ResMut<LayoutCx>,
) {
    for (
        entity,
        text,
        text_layout,
        computed_layout,
        mut content_size,
        mut text_flags,
        mut computed_text_block,
    ) in &mut text_query
    {
        let Some(target_info) = ui_stack_map
            .iter()
            .find_map(|stack| {
                stack
                    .1
                    .bitset
                    .contains(entity.index_u32() as usize)
                    .then_some(*stack.0)
            })
            .iter()
            .find_map(|&camera_entity| camera_query.get(camera_entity).ok())
        else {
            continue;
        };

        let is_changed = computed_text_block
            .needs_rerender(computed_layout.is_changed(), rem_size.is_changed())
            || text.is_changed()
            || content_size.is_changed()
            || text_flags.needs_measure_fn;

        if !is_changed {
            continue;
        }

        let scale_factor = target_info.scale_factor;
        let physical_size = target_info.physical_size;

        match text_pipeline.create_text_measure(
            entity,
            fonts.as_ref(),
            text_reader.iter(entity),
            scale_factor,
            &text_layout,
            computed_text_block.as_mut(),
            &mut font_system,
            &mut layout_cx,
            physical_size,
            rem_size.0,
        ) {
            Ok(measure) => {
                if text_layout.linebreak == LineBreak::NoWrap {
                    content_size.set(FixedMeasure { size: measure.max });
                } else {
                    content_size.set(TextMeasure {
                        min: measure.min,
                        max: measure.max,
                        entity: measure.entity,
                        scale_factor,
                    });
                }

                // Text measure func created successfully, so set `TextFlags` to schedule a recompute
                text_flags.needs_measure_fn = false;
                text_flags.needs_recompute = true;
            }
            Err(
                TextError::NoSuchFont
                | TextError::NoSuchFontFamily(_)
                | TextError::DegenerateScaleFactor,
            ) => {
                // Try again next frame
                text_flags.needs_measure_fn = true;
            }
            Err(
                e @ (TextError::FailedToAddGlyph(_)
                | TextError::FailedToGetGlyphImage(_)
                | TextError::MissingAtlasLayout
                | TextError::MissingAtlasTexture
                | TextError::InconsistentAtlasState),
            ) => {
                panic!("Fatal error when processing text: {e}.");
            }
        };
    }
}

pub fn text_system(
    camera_query: Query<Ref<ComputedTargetInfo>, With<Camera>>,
    ui_stack_map: Res<UiStackMap>,
    mut textures: ResMut<Assets<bevy_image::Image>>,
    mut font_atlas_set: ResMut<FontAtlasSet>,
    mut text_pipeline: ResMut<TextPipeline>,
    mut text_query: Query<(
        Entity,
        Ref<Div>,
        Ref<ComputedLayout>,
        Ref<FontHinting>,
        Ref<TextLayout>,
        &mut TextLayoutInfo,
        &mut TextFlags,
        &mut ComputedTextBlock,
    )>,
    mut scale_cx: ResMut<ScaleCx>,
) {
    for (
        entity,
        div,
        computed_layout,
        hinting,
        text_layout,
        mut text_layout_info,
        mut text_flags,
        mut computed_text_block,
    ) in &mut text_query
    {
        let Some(target_info) = ui_stack_map
            .iter()
            .find_map(|stack| {
                stack
                    .1
                    .bitset
                    .contains(entity.index_u32() as usize)
                    .then_some(*stack.0)
            })
            .iter()
            .find_map(|&camera_entity| camera_query.get(camera_entity).ok())
        else {
            continue;
        };

        let is_changed = div.is_changed()
            || target_info.is_changed()
            || hinting.is_changed()
            || text_flags.needs_recompute;

        if !is_changed {
            continue;
        }

        // Skip the text node if it is waiting for a new measure func
        if text_flags.needs_measure_fn {
            continue;
        }

        let scale_factor = target_info.scale_factor;

        let physical_node_size = if text_layout.linebreak == LineBreak::NoWrap {
            // With `NoWrap` set, no constraints are placed on the width of the text.
            TextBounds::UNBOUNDED
        } else {
            // We currently don't compute the size of the node with a scale factor,
            // and should apply the scale factor to the text layout engine.
            TextBounds::from(computed_layout.size * scale_factor)
        };

        match text_pipeline.update_text_layout_info(
            &mut text_layout_info,
            &mut font_atlas_set,
            &mut textures,
            &mut computed_text_block,
            &mut scale_cx,
            physical_node_size,
            text_layout.justify,
            *hinting,
        ) {
            Err(
                TextError::NoSuchFont
                | TextError::NoSuchFontFamily(_)
                | TextError::DegenerateScaleFactor,
            ) => {
                // There was an error processing the text layout, try again next frame
                text_flags.needs_recompute = true;
            }
            Err(e @ TextError::FailedToGetGlyphImage(_)) => {
                bevy_log::warn_once!("{e}.");
                text_flags.needs_recompute = false;
                text_layout_info.clear();
            }
            Err(
                e @ (TextError::FailedToAddGlyph(_)
                | TextError::MissingAtlasLayout
                | TextError::MissingAtlasTexture
                | TextError::InconsistentAtlasState),
            ) => {
                panic!("Fatal error when processing text: {e}.");
            }
            Ok(()) => {
                text_layout_info.scale_factor = scale_factor;
                text_layout_info.size *= scale_factor;
                text_flags.needs_recompute = false;
            }
        }
    }
}
