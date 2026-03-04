use bevy_asset::AssetId;
use bevy_camera::visibility::InheritedVisibility;
use bevy_color::{Alpha, Color, ColorToComponents};
use bevy_ecs::{
    entity::Entity,
    prelude::Res,
    system::{Commands, Query, ResMut},
};
use bevy_math::Mat4;
use bevy_render::{Extract, sync_world::TemporaryRenderEntity};
use bevy_transform::components::GlobalTransform;

use bevy_moon_core::prelude::{ComputedLayout, Div, UiStackMap};

use crate::pipelines::ExtractedUiInstance;

use super::{ExtractedUiQuads, UiQuad};

pub fn extract_quads(
    mut commands: Commands,
    mut extracted_ui_quads: ResMut<ExtractedUiQuads>,
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
    extracted_ui_quads.instances.clear();

    for (&camera_entity, ui_stack) in ui_stack_map.iter() {
        for div in ui_stack
            .ranges
            .iter()
            .flat_map(|range| div_query.iter_many(&ui_stack.entities[range.clone()]))
        {
            extract_quad(&mut commands, &mut extracted_ui_quads, div, camera_entity);
        }
    }
}

fn extract_quad(
    commands: &mut Commands,
    extracted_ui_quads: &mut ExtractedUiQuads,
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
    if computed_layout.is_empty() {
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

    let size = computed_layout.size.to_array();
    let corner_radii = div.corner_radii.to_array(); // should be computed_layout.corner_radii
    let border_widths = computed_layout.border_widths.to_array();

    let matrix = Mat4::from(transform.affine()).to_cols_array_2d();

    let render_entity = commands.spawn(TemporaryRenderEntity).id();

    extracted_ui_quads.instances.push(ExtractedUiInstance {
        index,
        camera_entity,
        entity: (render_entity, main_entity),
        texture: AssetId::default(),

        instance: UiQuad {
            matrix,
            color,
            size,
            corner_radii,
            border_color,
            border_widths,
        },
    });
}
