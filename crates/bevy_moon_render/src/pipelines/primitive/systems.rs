use bevy_asset::AssetId;
use bevy_camera::visibility::InheritedVisibility;
use bevy_color::{Alpha, Color, ColorToComponents, LinearRgba};
use bevy_ecs::{
    entity::Entity,
    prelude::Res,
    schedule::SystemSet,
    system::{Query, ResMut},
};
use bevy_render::{Extract, sync_world::RenderEntity};

use bevy_moon_core::prelude::{ComputedLayout, Div, UiStackMap};
use bevy_transform::components::GlobalTransform;

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

pub fn extract_images(ui_stack_map: Extract<Res<UiStackMap>>) {}

pub fn extract_texts(ui_stack_map: Extract<Res<UiStackMap>>) {}

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
    let Some(color) = div.background.filter(|c| !c.is_fully_transparent()) else {
        return;
    };

    let index = div.stack_index as f32;
    let main_entity = entity.into();
    let affine = transform.affine();
    let position = affine.translation.into();
    let size = computed_layout.size.into();
    let color = LinearRgba::from(color).to_f32_array();
    let corner_radii = div.corner_radii.into(); // should be computed_layout.corner_radii
    let border_color = LinearRgba::from(div.border_color.unwrap_or(Color::NONE)).to_f32_array();
    let border_widths = computed_layout.border_widths.to_array();

    extracted_ui_instances.instances.push(ExtractedUiInstance {
        index,
        camera_entity,
        entity: (render_entity, main_entity),
        image: AssetId::default(),

        instance: UiInstance {
            size,
            position,
            color,
            corner_radii,
            border_color,
            border_widths,
        },
    });
}
