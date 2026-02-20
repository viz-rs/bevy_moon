use std::{any::TypeId, ops::DerefMut};

use bevy_camera::{Camera, visibility::VisibleEntities};
use bevy_ecs::{
    change_detection::{DetectChanges, DetectChangesMut},
    entity::Entity,
    hierarchy::{ChildOf, Children},
    lifecycle::RemovedComponents,
    query::{Changed, With, Without},
    schedule::SystemSet,
    system::{Local, Query, Res, ResMut},
    world::Ref,
};
use bevy_math::{UVec2, Vec2};
use bevy_text::{ComputedTextBlock, FontCx};
use bevy_transform::components::{GlobalTransform, Transform};
use fixedbitset::FixedBitSet;
use smallvec::SmallVec;
use taffy::NodeId;

use crate::{
    components::computed::ComputedLayout,
    layout::UiLayoutTree,
    prelude::Div,
    stack::{UiStack, UiStackMap},
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum UiSystems {
    Prepare,
    Content,
    Stack,
    Layout,
    PostLayout,
}

pub fn ui_stack_system(
    render_targets: Query<(Entity, &VisibleEntities), With<Camera>>,
    root_div_query: Query<
        (Entity, &GlobalTransform, Option<&Children>),
        (With<Div>, Without<ChildOf>),
    >,
    div_query: Query<(Entity, &GlobalTransform, Option<&Children>), With<Div>>,
    mut update_query: Query<&mut Div>,
    mut ui_stack_map: ResMut<UiStackMap>,
    mut view_entities: Local<FixedBitSet>,
) {
    ui_stack_map.clear();

    for (camera_entity, visiable_entities) in render_targets {
        view_entities.clear();
        view_entities.extend(
            visiable_entities
                .get(TypeId::of::<Div>())
                .iter()
                .map(|e| e.index_u32() as usize),
        );

        if view_entities.is_clear() {
            continue;
        }

        let ui_stack = ui_stack_map
            .as_mut()
            .deref_mut()
            .entry(camera_entity)
            .or_default();

        ui_stack.bitset.union_with(&view_entities);

        // Only filter root divs.
        let divs = root_div_query
            .iter()
            .filter(|entity| ui_stack.bitset.contains(entity.0.index_u32() as usize))
            .collect::<Vec<_>>();

        ui_stack.roots.extend(divs.iter().map(|e| e.0));

        // Make sure ui transparency phases' `sort_key` is correct.
        let mut depth = 0;

        update_ui_stack_recursive(
            &div_query,
            &mut update_query,
            ui_stack,
            &mut depth,
            divs,
            camera_entity,
        );
    }
}

fn update_ui_stack_recursive(
    div_query: &Query<(Entity, &GlobalTransform, Option<&Children>), With<Div>>,
    update_query: &mut Query<&mut Div>,
    ui_stack: &mut UiStack,
    depth: &mut usize,
    mut sorted_divs: Vec<(Entity, &GlobalTransform, Option<&Children>)>,
    camera_entity: Entity,
) {
    if sorted_divs.is_empty() {
        return;
    }

    // back to front
    radsort::sort_by_key(&mut sorted_divs, |e| (e.1.translation().z, e.0.index_u32()));

    tracing::debug!(
        "camera: {} {} {:?}",
        camera_entity,
        &depth,
        sorted_divs
            .iter()
            .map(|e| (e.0, e.1.translation().z))
            .collect::<Vec<_>>()
    );

    let start = ui_stack
        .ranges
        .last()
        .map(|range| range.end)
        .unwrap_or_default();
    let end = start + sorted_divs.len();
    ui_stack.ranges.push(start..end);

    for (entity, _transform, children) in sorted_divs {
        if let Ok(mut div) = update_query.get_mut(entity) {
            div.bypass_change_detection().stack_index = *depth;
        }

        ui_stack.entities.push(entity);

        *depth += 1;

        let Some(children) = children.filter(|c| !c.is_empty()) else {
            continue;
        };

        let divs = div_query.iter_many(children).collect::<Vec<_>>();

        update_ui_stack_recursive(
            &div_query,
            update_query,
            ui_stack,
            depth,
            divs,
            camera_entity,
        );
    }
}

pub fn ui_layout_system(
    camera_query: Query<&Camera>,
    root_div_query: Query<(Entity, Ref<Div>, Option<&Children>), Without<ChildOf>>,
    div_query: Query<(Entity, Ref<Div>, Option<&Children>), With<ChildOf>>,
    ui_stack_map: Res<UiStackMap>,
    mut ui_layout_tree: ResMut<UiLayoutTree>,
    mut layouts: Local<SmallVec<[taffy::NodeId; 4]>>,

    mut removed_children: RemovedComponents<Children>,
    mut removed_div: RemovedComponents<Div>,

    changed_children_query: Query<(), (Changed<Children>, With<Div>)>,

    mut text_block_query: Query<&mut ComputedTextBlock>,
    mut font_system: ResMut<FontCx>,
    mut update_div_query: Query<(&mut Transform, &mut ComputedLayout), With<Div>>,
) {
    for (_camera_entity, ui_stack) in ui_stack_map.as_ref().iter() {
        for item in root_div_query.iter_many(&ui_stack.roots) {
            update_ui_layout_recursive(
                &div_query,
                &changed_children_query,
                &mut ui_layout_tree,
                &mut layouts,
                item,
            );
        }

        layouts.clear();
    }

    {
        // Updates and remove children.
        ui_layout_tree.remove_nodes_children(
            removed_children
                .read()
                .filter(|&entity| div_query.contains(entity)),
        );
        // Cleans up removed divs after syncing children.
        ui_layout_tree.remove_nodes(
            removed_div
                .read()
                .filter(|&entity| !div_query.contains(entity)),
        );
    }

    for (&camera_entity, ui_stack) in ui_stack_map.as_ref().iter() {
        let Ok(camera) = camera_query.get(camera_entity) else {
            continue;
        };
        let physical_size = camera.physical_viewport_size().unwrap_or(UVec2::ZERO);

        for item in root_div_query.iter_many(&ui_stack.roots) {
            ui_layout_tree.compute_layout(
                item.0,
                physical_size,
                &mut text_block_query,
                &mut font_system,
            );

            update_ui_geometry_recursive(
                &div_query,
                &mut update_div_query,
                &mut ui_layout_tree,
                item,
                None,
            );
        }

        layouts.clear();
    }

    ui_layout_tree.print_tree();
}

fn update_ui_layout_recursive(
    div_query: &Query<(Entity, Ref<Div>, Option<&Children>), With<ChildOf>>,
    changed_children_query: &Query<(), (Changed<Children>, With<Div>)>,
    ui_layout_tree: &mut UiLayoutTree,
    layouts: &mut SmallVec<[NodeId; 4]>,
    (entity, div, children): (Entity, Ref<Div>, Option<&Children>),
) {
    // Stores current node's layout id and index.
    let mut node = Option::<(NodeId, usize)>::None;

    let is_changed = div.is_added()
        || div.is_changed()
        || changed_children_query.contains(entity)
        || !ui_layout_tree.contains(entity);

    if is_changed {
        let node_id = ui_layout_tree.upsert_node(entity, div.style.clone(), div.measure.clone());

        layouts.push(node_id);

        node = Some((node_id, layouts.len()));
    }

    if let Some(children) = children {
        for div in div_query.iter_many(children) {
            update_ui_layout_recursive(
                div_query,
                changed_children_query,
                ui_layout_tree,
                layouts,
                div,
            );
        }
    }

    // The current node is created, so we can update its children.
    let Some((node_id, node_children)) = node
        .map(|(node_id, index)| (node_id, layouts.drain(index..)))
        .filter(|(_, node_children)| node_children.len() > 0)
    else {
        return;
    };

    ui_layout_tree.set_node_children(node_id, &node_children.collect::<Vec<_>>());
}

fn update_ui_geometry_recursive(
    div_query: &Query<(Entity, Ref<Div>, Option<&Children>), With<ChildOf>>,
    update_div_query: &mut Query<(&mut Transform, &mut ComputedLayout), With<Div>>,
    ui_layout_tree: &mut UiLayoutTree,
    (entity, _, children): (Entity, Ref<Div>, Option<&Children>),
    mut maybe_inherited: Option<(Transform, Vec2)>,
) {
    let (Ok(layout), Ok((mut transform, mut computed_layout))) = (
        ui_layout_tree.get_layout(entity),
        update_div_query.get_mut(entity),
    ) else {
        return;
    };

    {
        let bypass_computed_layout = computed_layout.bypass_change_detection();
        let prev_location = bypass_computed_layout.location;
        let prev_size = bypass_computed_layout.size;

        bypass_computed_layout.update(layout);
        // bypass_computed_node.set_corner_radii(style.corner_radii);

        // if let Some(outline) = style.outline {
        //     bypass_computed_node.set_outline(outline);
        // }

        if prev_location != computed_layout.location || prev_size != computed_layout.size {
            computed_layout.set_changed();
        }
    }

    if let Some((_parent_transform, parent_size)) = maybe_inherited {
        // @TODO(fundon): scrolling
        let local_center = computed_layout.location + 0.5 * (computed_layout.size - parent_size);
        let local_center_flipped = local_center * Vec2::new(1.0, -1.0);

        let mut local_affine = computed_layout.affine;
        if local_center_flipped != local_affine.translation.truncate() {
            // extracts transform without layout translation
            let base_affine = transform.compute_affine() * local_affine.inverse();

            // updates layout translation
            local_affine.translation.x = local_center_flipped.x;
            local_affine.translation.y = local_center_flipped.y;

            // applies new layout translation
            let new_affine = base_affine * local_affine;

            transform.translation.x = new_affine.translation.x;
            transform.translation.y = new_affine.translation.y;

            computed_layout.affine = local_affine;
        }
    }

    if let Some(children) = children {
        // Updates its children.
        maybe_inherited = Some((*transform, computed_layout.size));

        for item in div_query.iter_many(children) {
            update_ui_geometry_recursive(
                div_query,
                update_div_query,
                ui_layout_tree,
                item,
                maybe_inherited,
            );
        }
    }
}
