use bevy_ecs::{
    entity::{Entity, EntityHashMap},
    query::With,
    system::{Commands, Local, Query, Res, ResMut},
};
use bevy_math::FloatOrd;
use bevy_moon_core::prelude::UiStackMap;
use bevy_render::{
    render_phase::{
        DrawFunctionId, DrawFunctions, PhaseItem, PhaseItemExtraIndex, ViewSortedRenderPhases,
    },
    render_resource::{BindGroupEntries, PipelineCache, SpecializedRenderPipelines},
    renderer::{RenderDevice, RenderQueue},
    sync_world::MainEntity,
    view::{ExtractedView, ViewUniforms},
};
use indexmap::IndexMap;

use crate::{
    transparent::{RenderPhasesFilter, TransparentUi},
    view::{MoonUiCameraView, MoonUiOptions, MoonUiViewTarget},
};

use super::{
    ExtractedUiQuads, UiQuadBatch, UiQuadMeta, UiQuadViewBindGroup,
    draw::DrawUiQuad,
    pipeline::{UiQuadPipeline, UiQuadPipelineKey},
};

pub fn queue_quads(
    render_targets: Query<(MainEntity, &MoonUiCameraView, &MoonUiOptions)>,
    render_views: Query<&ExtractedView, With<MoonUiViewTarget>>,
    extracted_ui_quads: Res<ExtractedUiQuads>,
    ui_stack_map: Res<UiStackMap>,
    ui_quad_pipeline: Res<UiQuadPipeline>,
    pipeline_cache: Res<PipelineCache>,
    draw_functions: Res<DrawFunctions<TransparentUi>>,
    mut pipelines: ResMut<SpecializedRenderPipelines<UiQuadPipeline>>,
    mut render_phases: ResMut<ViewSortedRenderPhases<TransparentUi>>,
) {
    let draw_function = draw_functions.read().id::<DrawUiQuad>();

    for (extracted_index, div) in extracted_ui_quads.instances.iter().enumerate() {
        let Some(ui_stack) = ui_stack_map.get(&div.camera_entity) else {
            return;
        };
        let Some((_camera_entity, &MoonUiCameraView(ui_camera_view), &MoonUiOptions(mesh_key))) =
            render_targets.iter().find(|r| r.0 == div.camera_entity)
        else {
            continue;
        };
        let Some(extracted_view) = render_views.get(ui_camera_view).ok() else {
            continue;
        };
        let Some(render_phase) = render_phases.get_mut(&extracted_view.retained_view_entity) else {
            continue;
        };

        let pipeline = pipelines.specialize(
            &pipeline_cache,
            &ui_quad_pipeline,
            UiQuadPipelineKey {
                mesh_key,
                // @TODO(fundon): add an `UiAntiAlias` option
                // anti_alias: true,
            },
        );

        let view_index = div.entity.1.index_u32() as usize;
        if !ui_stack.bitset.contains(view_index) {
            continue;
        }

        let entity = div.entity;
        let sort_key = FloatOrd(div.index);

        render_phase.add_transient(TransparentUi {
            pipeline,
            draw_function,
            extracted_index,
            entity,
            sort_key,
            indexed: true,
            batch_range: 0..0,
            extra_index: PhaseItemExtraIndex::None,
        });
    }
}

pub fn prepare_view_bind_groups(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    pipeline_cache: Res<PipelineCache>,
    ui_quad_pipeline: Res<UiQuadPipeline>,
    view_uniforms: Res<ViewUniforms>,
    views: Query<Entity, (With<ExtractedView>, With<MoonUiViewTarget>)>,
) {
    let Some(view_binding) = view_uniforms.uniforms.binding() else {
        return;
    };

    for entity in &views {
        let value = render_device.create_bind_group(
            "moon_ui_quad_view_bind_group",
            &pipeline_cache.get_bind_group_layout(&ui_quad_pipeline.view_layout),
            &BindGroupEntries::single(view_binding.clone()),
        );

        commands
            .entity(entity)
            .insert(UiQuadViewBindGroup::new(value));
    }
}

pub fn prepare_quads(
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
    draw_functions: Res<DrawFunctions<TransparentUi>>,
    mut commands: Commands,
    mut ui_quad_meta: ResMut<UiQuadMeta>,
    mut extracted_ui_quads: ResMut<ExtractedUiQuads>,
    mut render_phases: ResMut<ViewSortedRenderPhases<TransparentUi>>,
    // maps `main entity` to `render entity`
    mut live_entities: Local<IndexMap<MainEntity, Entity>>,
    mut cached_draw_function: Local<Option<DrawFunctionId>>,
) {
    ui_quad_meta.instance_buffer.clear();

    let draw_function =
        *cached_draw_function.get_or_insert_with(|| draw_functions.read().id::<DrawUiQuad>());

    let mut batches = EntityHashMap::<UiQuadBatch>::with_capacity(live_entities.capacity());

    for (item, instance) in render_phases.filter(draw_function).filter_map(|item| {
        extracted_ui_quads
            .instances
            .get(item.extracted_index)
            .map(|extracted_ui_instance| (item, extracted_ui_instance.instance))
    }) {
        let index = ui_quad_meta.instance_buffer.push(instance) as u32;

        let render_entity = live_entities
            .entry(item.main_entity())
            .or_insert_with(|| item.entity());

        batches
            .entry(*render_entity)
            .and_modify(|batch| {
                batch.range.end = index + 1;
            })
            .or_insert_with(|| {
                // only the first phase needs to be updated
                // phases under the same entity will be batch processed
                item.batch_range_mut().end += 1;
                UiQuadBatch::new(index..index + 1)
            });
    }

    ui_quad_meta
        .instance_buffer
        .write_buffer(&render_device, &render_queue);

    commands.try_insert_batch(batches);

    extracted_ui_quads.instances.clear();
    live_entities.clear();
}
