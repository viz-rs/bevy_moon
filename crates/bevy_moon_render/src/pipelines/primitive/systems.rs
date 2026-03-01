use std::ops::Mul;

use bevy_asset::AssetId;
use bevy_camera::visibility::InheritedVisibility;
use bevy_color::{Alpha, Color, ColorToComponents};
use bevy_ecs::{
    entity::Entity,
    prelude::Res,
    schedule::SystemSet,
    system::{Commands, Query, ResMut},
};
use bevy_image::TRANSPARENT_IMAGE_HANDLE;
use bevy_math::{Affine3A, Vec3};
use bevy_render::{Extract, sync_world::TemporaryRenderEntity};
use bevy_text::{ComputedTextBlock, GlyphAtlasInfo, PositionedGlyph, TextColor, TextLayoutInfo};
use bevy_transform::components::GlobalTransform;

use bevy_moon_core::{
    geometry::{FLIP_X, FLIP_Y},
    prelude::{ComputedLayout, Div, Image, Text, UiStackMap},
};

use crate::pipelines::ExtractedUiInstance;

use super::{ExtractedUiInstances, UiInstance};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum ExtractUiSystems {
    CameraViews,
    Shadows,
    Divs,
    Images,
    Texts,
}

pub fn extract_divs(
    mut commands: Commands,
    mut extracted_ui_instances: ResMut<ExtractedUiInstances>,
    ui_stack_map: Extract<Res<UiStackMap>>,
    div_query: Extract<
        Query<(
            Entity,
            &GlobalTransform,
            &InheritedVisibility,
            &ComputedLayout,
            &Div,
        )>,
    >,
) {
    extracted_ui_instances.instances.clear();

    for (&camera_entity, ui_stack) in ui_stack_map.iter() {
        for div in ui_stack
            .ranges
            .iter()
            .flat_map(|range| div_query.iter_many(&ui_stack.entities[range.clone()]))
        {
            extract_single_div(
                &mut commands,
                &mut extracted_ui_instances,
                div,
                camera_entity,
            );
        }
    }
}

fn extract_single_div(
    commands: &mut Commands,
    extracted_ui_instances: &mut ExtractedUiInstances,
    (entity, transform, inherited_visibility, computed_layout, div): (
        Entity,
        &GlobalTransform,
        &InheritedVisibility,
        &ComputedLayout,
        &Div,
    ),
    camera_entity: Entity,
) {
    if !inherited_visibility.get() {
        return;
    }

    let color = div.background.unwrap_or(Color::NONE);
    let border_color = div.border_color.unwrap_or(Color::NONE);

    if color.is_fully_transparent() && border_color.is_fully_transparent() {
        return;
    }

    let color = color.to_linear().to_f32_array();
    let border_color = border_color.to_linear().to_f32_array();

    let index = div.stack_index as f32;
    let main_entity = entity.into();

    let [x_axis, y_axis, z_axis, position] = transform.affine().to_cols_array_2d();

    let size = computed_layout.size.to_array();
    let corner_radii = div.corner_radii.to_array(); // should be computed_layout.corner_radii
    let border_widths = computed_layout.border_widths.to_array();

    let render_entity = commands.spawn(TemporaryRenderEntity).id();

    extracted_ui_instances.instances.push(ExtractedUiInstance {
        index,
        camera_entity,
        entity: (render_entity, main_entity),
        texture: AssetId::default(),

        instance: UiInstance {
            position,
            x_axis,
            y_axis,
            z_axis,
            color,
            size,
            flags: 0,
            corner_radii,
            border_color,
            border_widths,
            ..UiInstance::DEFAULT
        },
    });
}

pub fn extract_images(
    mut commands: Commands,
    mut extracted_ui_instances: ResMut<ExtractedUiInstances>,
    ui_stack_map: Extract<Res<UiStackMap>>,
    image_query: Extract<
        Query<(
            Entity,
            &GlobalTransform,
            &InheritedVisibility,
            &ComputedLayout,
            &Div,
            &Image,
        )>,
    >,
) {
    for (&camera_entity, ui_stack) in ui_stack_map.iter() {
        for div in ui_stack
            .ranges
            .iter()
            .flat_map(|range| image_query.iter_many(&ui_stack.entities[range.clone()]))
        {
            extract_single_image(
                &mut commands,
                &mut extracted_ui_instances,
                div,
                camera_entity,
            );
        }
    }
}

fn extract_single_image(
    commands: &mut Commands,
    extracted_ui_instances: &mut ExtractedUiInstances,
    (entity, transform, inherited_visibility, computed_layout, div, image): (
        Entity,
        &GlobalTransform,
        &InheritedVisibility,
        &ComputedLayout,
        &Div,
        &Image,
    ),
    camera_entity: Entity,
) {
    if !inherited_visibility.get() {
        return;
    }
    if image.handle == TRANSPARENT_IMAGE_HANDLE {
        return;
    }
    if image.color.is_fully_transparent() {
        return;
    }

    let index = div.stack_index as f32 + 0.01;
    let main_entity = entity.into();
    let size = computed_layout.size.to_array();
    let color = image.color.to_linear().to_f32_array();
    let corner_radii = div.corner_radii.to_array(); // should be computed_layout.corner_radii
    let border_color = Color::NONE.to_linear().to_f32_array(); // ignore border color
    let border_widths = computed_layout.border_widths.to_array();
    let extra = (*image.object_position)
        .extend(image.object_fit as isize as f32)
        .to_array();
    let flip = image.flip.map(Into::into);

    let render_entity = commands.spawn(TemporaryRenderEntity).id();

    let [x_axis, y_axis, z_axis, position] = transform.affine().to_cols_array_2d();

    extracted_ui_instances.instances.push(ExtractedUiInstance {
        index,
        camera_entity,
        entity: (render_entity, main_entity),
        texture: image.handle.id(),

        instance: UiInstance {
            position,
            x_axis,
            y_axis,
            z_axis,
            color,
            size,
            flags: 1,
            corner_radii,
            border_color,
            border_widths,
            extra,
            flip,
            ..UiInstance::DEFAULT
        },
    });
}

pub fn extract_texts(
    mut commands: Commands,
    mut extracted_ui_instances: ResMut<ExtractedUiInstances>,
    ui_stack_map: Extract<Res<UiStackMap>>,
    text_query: Extract<
        Query<(
            Entity,
            &GlobalTransform,
            &InheritedVisibility,
            &ComputedLayout,
            &Div,
            &Text,
            &TextColor,
            &TextLayoutInfo,
            &ComputedTextBlock,
        )>,
    >,
    text_colors: Extract<Query<&TextColor>>,
) {
    for (&camera_entity, ui_stack) in ui_stack_map.iter() {
        for div in ui_stack
            .ranges
            .iter()
            .flat_map(|range| text_query.iter_many(&ui_stack.entities[range.clone()]))
        {
            extract_single_text(
                &mut commands,
                &mut extracted_ui_instances,
                div,
                &text_colors,
                camera_entity,
            );
        }
    }
}

fn extract_single_text(
    commands: &mut Commands,
    extracted_ui_instances: &mut ExtractedUiInstances,
    (
        entity,
        transform,
        inherited_visibility,
        computed_layout,
        div,
        _text,
        text_color,
        text_layout_info,
        computed_text_block,
    ): (
        Entity,
        &GlobalTransform,
        &InheritedVisibility,
        &ComputedLayout,
        &Div,
        &Text,
        &TextColor,
        &TextLayoutInfo,
        &ComputedTextBlock,
    ),
    text_colors: &Extract<Query<&TextColor>>,
    camera_entity: Entity,
) {
    if !inherited_visibility.get() {
        return;
    }

    let offset = computed_layout.size.mul(FLIP_X * 0.5).extend(0.0);

    let scale_factor = text_layout_info.scale_factor;
    let scale_factor_recip = scale_factor.recip();
    let scale_factor_affine = Affine3A::from_scale(Vec3::splat(scale_factor));
    let affine = transform
        .affine()
        .mul(Affine3A::from_translation(offset))
        .mul(scale_factor_affine.inverse());

    let index = div.stack_index as f32 + 0.06;
    let main_entity = entity.into();
    let corner_radii = div.corner_radii.to_array();

    let mut color = text_color.to_linear();
    let mut current_span = usize::MAX;

    for &PositionedGlyph {
        position,
        span_index,
        atlas_info: GlyphAtlasInfo { texture, rect, .. },
        ..
    } in text_layout_info.glyphs.iter()
    {
        if span_index != current_span {
            color = text_colors
                .get(
                    computed_text_block
                        .entities()
                        .get(span_index)
                        .map(|t| t.entity)
                        .unwrap_or(Entity::PLACEHOLDER),
                )
                .map(|text_color| text_color.0.to_linear())
                .unwrap_or_default();
            current_span = span_index;
        }

        let color = color.to_f32_array();
        let top_left = rect.min * scale_factor_recip;
        let size = rect.size().mul(scale_factor_recip).to_array();
        let extra = [top_left.x, top_left.y, scale_factor_recip]; // glyph tile's top-left position
        let position_flipped = position.mul(FLIP_Y).extend(0.0);

        let [x_axis, y_axis, z_axis, position] = affine
            .mul(Affine3A::from_translation(position_flipped))
            .mul(scale_factor_affine)
            .to_cols_array_2d();

        let render_entity = commands.spawn(TemporaryRenderEntity).id();

        extracted_ui_instances.instances.push(ExtractedUiInstance {
            index,
            camera_entity,
            entity: (render_entity, main_entity),
            texture,

            instance: UiInstance {
                position,
                x_axis,
                y_axis,
                z_axis,
                color,
                size,
                flags: 3,
                corner_radii,
                extra,
                ..UiInstance::DEFAULT
            },
        });
    }
}
