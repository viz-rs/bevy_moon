use bevy_asset::AssetId;
use bevy_camera::visibility::InheritedVisibility;
use bevy_color::{Alpha, ColorToComponents, LinearRgba};
use bevy_ecs::{
    entity::Entity,
    prelude::Res,
    system::{Query, ResMut},
};
use bevy_math::Vec2;
use bevy_moon_core::prelude::{ComputedLayout, Div, UiStackMap};
use bevy_render::{Extract, sync_world::RenderEntity};
use bevy_transform::components::GlobalTransform;

use crate::pipelines::{
    ExtractedUiInstance,
    shadow::{ExtractedUiShadows, extract::UiShadow},
};

pub fn extract_shadows(
    mut extracted_ui_instances: ResMut<ExtractedUiShadows>,
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
    extracted_ui_instances: &mut ExtractedUiShadows,
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

    let Some(shadows) = &div.box_shadow else {
        return;
    };

    if shadows.is_empty() {
        return;
    }

    let index = div.stack_index as f32;
    let main_entity = entity.into();
    let affine = transform.affine();
    let size = computed_layout.size;
    let spread_ratio = size.y / size.x;
    let corner_radii = div.corner_radii.into(); // should be computed_layout.corner_radii

    for shadow in shadows {
        if shadow.color.is_fully_transparent() {
            continue;
        }

        let spread_radius = shadow.spread_radius;
        let spread = Vec2::new(spread_radius, spread_radius * spread_ratio);

        let color = LinearRgba::from(shadow.color).to_f32_array();
        let offset = shadow.offset * Vec2::new(1.0, -1.0);
        let blur_radius = shadow.blur_radius;
        let shadow_size = size - spread * 2.0;
        let position = affine.translation.to_vec3() + offset.extend(0.0);

        extracted_ui_instances.instances.push(ExtractedUiInstance {
            index,
            camera_entity,
            entity: (render_entity, main_entity),
            image: AssetId::default(),

            instance: UiShadow {
                color,
                corner_radii,
                blur_radius,
                size: shadow_size.into(),
                position: position.into(),
            },
        });
    }
}
