use std::ops::Mul;

use bevy_camera::visibility::InheritedVisibility;
use bevy_color::{Alpha, ColorToComponents};
use bevy_ecs::{
    entity::Entity,
    prelude::Res,
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

use crate::pipelines::{ExtractedUiInstance, atlas::ExtractedUiAtlases};

use super::UiAtlas;

pub fn extract_images(
    mut commands: Commands,
    mut extracted_ui_instances: ResMut<ExtractedUiAtlases>,
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
    extracted_ui_atlases: &mut ExtractedUiAtlases,
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
    if computed_layout.is_empty() {
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
    let extra = (*image.object_position)
        .extend(image.object_fit as isize as f32)
        .to_array();
    let flipped = image.flipped.map(Into::into);

    let matrix = transform.affine().to_cols_array_2d();

    let render_entity = commands.spawn(TemporaryRenderEntity).id();

    extracted_ui_atlases.instances.push(ExtractedUiInstance {
        index,
        camera_entity,
        entity: (render_entity, main_entity),
        texture: image.handle.id(),

        instance: UiAtlas {
            matrix,
            color,
            size,
            corner_radii,
            extra,
            flipped,
            ..UiAtlas::IMAGE
        },
    });
}

pub fn extract_texts(
    mut commands: Commands,
    mut extracted_ui_atlases: ResMut<ExtractedUiAtlases>,
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
                &mut extracted_ui_atlases,
                div,
                &text_colors,
                camera_entity,
            );
        }
    }
}

fn extract_single_text(
    commands: &mut Commands,
    extracted_ui_atlases: &mut ExtractedUiAtlases,
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
    if computed_layout.is_empty() {
        return;
    }

    let scale_factor = text_layout_info.scale_factor;
    let scale_factor_recip = scale_factor.recip();
    let scale_factor_affine = Affine3A::from_scale(Vec3::splat(scale_factor));

    let offset = computed_layout.size.mul(FLIP_X * 0.5).extend(0.0);
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
        let extra = top_left.extend(scale_factor_recip).to_array(); // glyph tile's top-left position
        let size = rect.size().mul(scale_factor_recip).to_array();
        let position_flipped = position.mul(FLIP_Y).extend(0.0);

        let matrix = affine
            .mul(Affine3A::from_translation(position_flipped))
            .mul(scale_factor_affine)
            .to_cols_array_2d();

        let render_entity = commands.spawn(TemporaryRenderEntity).id();

        extracted_ui_atlases.instances.push(ExtractedUiInstance {
            index,
            camera_entity,
            entity: (render_entity, main_entity),
            texture,

            instance: UiAtlas {
                matrix,
                color,
                size,
                corner_radii,
                extra,
                ..UiAtlas::TEXT
            },
        });
    }
}
