use std::{any::TypeId, ops::DerefMut};

use bevy_asset::Assets;
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
use bevy_text::{
    ComputedTextBlock, Font, FontAtlasSet, FontCx, FontHinting, LayoutCx, LineBreak, RemSize,
    ScaleCx, TextBounds, TextError, TextLayout, TextLayoutInfo, TextPipeline, TextReader,
};
use bevy_transform::components::{GlobalTransform, Transform};
use fixedbitset::FixedBitSet;
use smallvec::SmallVec;
use taffy::NodeId;

use crate::{
    components::{computed::ComputedLayout, text::TextFlags},
    layout::UiLayoutTree,
    prelude::{Div, Text},
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

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub(super) struct AmbiguousWithText;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub(super) struct AmbiguousWithUpdateText2dLayout;

pub fn measure_text_system(
    fonts: Res<Assets<Font>>,
    mut text_query: Query<
        (
            Entity,
            Ref<TextLayout>,
            Ref<FontHinting>,
            // &mut ContentSize,
            &mut TextFlags,
            &mut ComputedTextBlock,
            // Ref<ComputedUiRenderTargetInfo>,
            &ComputedLayout,
        ),
        With<Div>,
    >,
    mut text_reader: TextReader<Text>,
    mut text_pipeline: ResMut<TextPipeline>,
    mut font_system: ResMut<FontCx>,
    mut layout_cx: ResMut<LayoutCx>,
    rem_size: Res<RemSize>,
) {
    for (
        entity,
        block,
        font_hinting,
        // mut content_size,
        mut text_flags,
        mut computed_text_block,
        // computed_target,
        computed_layout,
    ) in &mut text_query
    {
        // Note: the ComputedTextBlock::needs_rerender bool is cleared in create_text_measure().
        // 1e-5 epsilon to ignore tiny scale factor float errors
        // if !(1e-5
        //     < (computed_target.scale_factor() - computed_node.inverse_scale_factor.recip()).abs()
        //     || computed.needs_rerender(computed_target.is_changed(), rem_size.is_changed())
        //     || text_flags.needs_measure_fn
        //     || content_size.is_added())
        // {
        //     continue;
        // }

        match text_pipeline.create_text_measure(
            entity,
            fonts.as_ref(),
            text_reader.iter(entity),
            1.0,
            // computed_target.scale_factor,
            &block,
            computed_text_block.as_mut(),
            &mut font_system,
            &mut layout_cx,
            // computed_target.logical_size(),
            Vec2::new(1024.0, 1024.0) * 2.0,
            rem_size.0,
        ) {
            Ok(measure) => {
                if block.linebreak == LineBreak::NoWrap {
                    // content_size.set(NodeMeasure::Fixed(FixedMeasure { size: measure.max }));
                } else {
                    // content_size.set(NodeMeasure::Text(TextMeasure { info: measure }));
                }

                // Text measure func created successfully, so set `TextNodeFlags` to schedule a recompute
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
    mut textures: ResMut<Assets<bevy_image::Image>>,
    mut font_atlas_set: ResMut<FontAtlasSet>,
    mut text_pipeline: ResMut<TextPipeline>,
    mut text_query: Query<(
        Ref<ComputedLayout>,
        &TextLayout,
        &mut TextLayoutInfo,
        &mut TextFlags,
        &mut ComputedTextBlock,
        Ref<FontHinting>,
    )>,
    mut scale_cx: ResMut<ScaleCx>,
) {
    for (node, block, mut text_layout_info, mut text_flags, mut computed, hinting) in
        &mut text_query
    {
        if node.is_changed() || text_flags.needs_recompute || hinting.is_changed() {
            // Skip the text node if it is waiting for a new measure func
            if text_flags.needs_measure_fn {
                continue;
            }

            // let physical_node_size = if block.linebreak == LineBreak::NoWrap {
            //     // With `NoWrap` set, no constraints are placed on the width of the text.
            //     TextBounds::UNBOUNDED
            // } else {
            //     // `scale_factor` is already multiplied by `UiScale`
            //     TextBounds::new(node.unrounded_size.x, node.unrounded_size.y)
            // };
            let physical_node_size = TextBounds::UNBOUNDED;

            match text_pipeline.update_text_layout_info(
                &mut text_layout_info,
                &mut font_atlas_set,
                &mut textures,
                &mut computed,
                &mut scale_cx,
                physical_node_size,
                block.justify,
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
                    // text_layout_info.scale_factor = node.inverse_scale_factor().recip();
                    // text_layout_info.size *= node.inverse_scale_factor();
                    text_flags.needs_recompute = false;
                }
            }
        }
    }
}
