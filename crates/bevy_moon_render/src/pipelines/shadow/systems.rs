use std::ops::Mul;

use bevy_asset::AssetId;
use bevy_camera::visibility::InheritedVisibility;
use bevy_color::{Alpha, ColorToComponents};
use bevy_ecs::{
    entity::Entity,
    prelude::Res,
    system::{Commands, Query, ResMut},
};
use bevy_math::{Affine3A, vec2};
use bevy_render::{Extract, sync_world::TemporaryRenderEntity};
use bevy_transform::components::GlobalTransform;

use bevy_moon_core::{
    geometry::FLIP_Y,
    prelude::{ComputedLayout, Div, UiStackMap},
};

use crate::pipelines::{
    ExtractedUiInstance,
    shadow::{ExtractedUiShadows, extract::UiShadow},
};

pub fn extract_shadows(
    mut commands: Commands,
    mut extracted_ui_instances: ResMut<ExtractedUiShadows>,
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
    extracted_ui_instances: &mut ExtractedUiShadows,
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

    let Some(shadows) = &div.box_shadow else {
        return;
    };

    if shadows.is_empty() {
        return;
    }

    let index = div.stack_index as f32 - 0.1;
    let affine = transform.affine();
    let main_entity = entity.into();
    let size = computed_layout.size;
    let spread_ratio = size.y / size.x;
    let corner_radii = div.corner_radii.to_array(); // should be computed_layout.corner_radii

    let [x_axis, y_axis, z_axis, _] = affine.to_cols_array_2d();

    for shadow in shadows {
        if shadow.color.is_fully_transparent() {
            continue;
        }

        let spread_radius = shadow.spread_radius;
        let spread = vec2(spread_radius, spread_radius * spread_ratio);
        let offset = shadow.offset * FLIP_Y;

        // expands bounds for shadow
        let shadow_size = size - spread * 2.0;

        let blur_radius = shadow.blur_radius;
        let color = shadow.color.to_linear().to_f32_array();

        let position = affine
            .mul(Affine3A::from_translation(offset.extend(0.0)))
            .translation
            .to_array();

        let render_entity = commands.spawn(TemporaryRenderEntity).id();

        extracted_ui_instances.instances.push(ExtractedUiInstance {
            index,
            camera_entity,
            entity: (render_entity, main_entity),
            texture: AssetId::default(),

            instance: UiShadow {
                position,
                x_axis,
                y_axis,
                z_axis,
                color,
                corner_radii,
                blur_radius,
                size: shadow_size.to_array(),
                ..UiShadow::DEFAULT
            },
        });
    }
}
