use bevy_asset::AssetId;
use bevy_camera::visibility::InheritedVisibility;
use bevy_color::{Alpha, Color, ColorToComponents, LinearRgba};
use bevy_ecs::{
    entity::Entity,
    prelude::Res,
    schedule::SystemSet,
    system::{Query, ResMut},
};
use bevy_image::TRANSPARENT_IMAGE_HANDLE;
use bevy_render::{Extract, sync_world::RenderEntity};
use bevy_transform::components::GlobalTransform;

use bevy_moon_core::prelude::{ComputedLayout, Div, Image, UiStackMap};

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
    mut extracted_ui_instances: ResMut<ExtractedUiInstances>,
    ui_stack_map: Extract<Res<UiStackMap>>,
    div_query: Extract<
        Query<(
            Entity,
            RenderEntity,
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
            extract_single_div(&mut extracted_ui_instances, div, camera_entity);
        }
    }
}

fn extract_single_div(
    extracted_ui_instances: &mut ExtractedUiInstances,
    (entity, render_entity, transform, inherited_visibility, computed_layout, div): (
        Entity,
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
    let affine = transform.affine();
    let position = affine.translation.into();
    let size = computed_layout.size.into();
    let corner_radii = div.corner_radii.into(); // should be computed_layout.corner_radii
    let border_widths = computed_layout.border_widths.to_array();

    extracted_ui_instances.instances.push(ExtractedUiInstance {
        index,
        camera_entity,
        entity: (render_entity, main_entity),
        texture: AssetId::default(),

        instance: UiInstance {
            position,
            color,
            size,
            flags: 0,
            corner_radii,
            border_color,
            border_widths,
        },
    });
}

pub fn extract_images(
    mut extracted_ui_instances: ResMut<ExtractedUiInstances>,
    ui_stack_map: Extract<Res<UiStackMap>>,
    div_query: Extract<
        Query<(
            Entity,
            RenderEntity,
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
            .flat_map(|range| div_query.iter_many(&ui_stack.entities[range.clone()]))
        {
            extract_single_image(&mut extracted_ui_instances, div, camera_entity);
        }
    }
}

fn extract_single_image(
    extracted_ui_instances: &mut ExtractedUiInstances,
    (entity, render_entity, transform, inherited_visibility, computed_layout, div, image): (
        Entity,
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
    let affine = transform.affine();
    let position = affine.translation.into();
    let size = computed_layout.size.into();
    let color = image.color.to_linear().to_f32_array();
    let corner_radii = div.corner_radii.into(); // should be computed_layout.corner_radii
    let border_color = LinearRgba::NONE.to_f32_array(); // ignore border color
    let border_widths = computed_layout.border_widths.to_array();

    extracted_ui_instances.instances.push(ExtractedUiInstance {
        index,
        camera_entity,
        entity: (render_entity, main_entity),
        texture: image.handle.id(),

        instance: UiInstance {
            position,
            color,
            size,
            flags: 1,
            corner_radii,
            border_color,
            border_widths,
        },
    });
}

pub fn extract_texts(ui_stack_map: Extract<Res<UiStackMap>>) {}
