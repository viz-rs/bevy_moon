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
    sync_world::{MainEntity, MainEntityHashMap},
    view::{ExtractedView, ViewUniforms},
};

use crate::{
    transparent::{RenderPhasesFilter, TransparentUi},
    view::{MoonUiCameraView, MoonUiOptions, MoonUiViewTarget},
};

use super::{
    ExtractedUiShadows, UiShadowBatch, UiShadowMeta, UiShadowViewBindGroup,
    draw::DrawShadows,
    pipeline::{UiShadowsPipeline, UiShadowsPipelineKey},
};

pub fn queue_shadows(
    render_targets: Query<(MainEntity, &MoonUiCameraView, &MoonUiOptions)>,
    render_views: Query<&ExtractedView, With<MoonUiViewTarget>>,
    extracted_ui_instances: Res<ExtractedUiShadows>,
    ui_stack_map: Res<UiStackMap>,
    ui_pipeline: Res<UiShadowsPipeline>,
    pipeline_cache: Res<PipelineCache>,
    draw_functions: Res<DrawFunctions<TransparentUi>>,
    mut pipelines: ResMut<SpecializedRenderPipelines<UiShadowsPipeline>>,
    mut render_phases: ResMut<ViewSortedRenderPhases<TransparentUi>>,
) {
    let Some(draw_function) = draw_functions.read().get_id::<DrawShadows>() else {
        return;
    };

    for (extracted_index, div) in extracted_ui_instances.instances.iter().enumerate() {
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
            &ui_pipeline,
            UiShadowsPipelineKey {
                mesh_key,
                samples: 4,
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

pub fn prepare_div_view_bind_groups(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    pipeline_cache: Res<PipelineCache>,
    ui_shadows_pipeline: Res<UiShadowsPipeline>,
    view_uniforms: Res<ViewUniforms>,
    views: Query<Entity, (With<ExtractedView>, With<MoonUiViewTarget>)>,
) {
    let Some(view_binding) = view_uniforms.uniforms.binding() else {
        return;
    };

    for entity in &views {
        let value = render_device.create_bind_group(
            "moon_ui_view_bind_group",
            &pipeline_cache.get_bind_group_layout(&ui_shadows_pipeline.view_layout),
            &BindGroupEntries::single(view_binding.clone()),
        );

        commands
            .entity(entity)
            .insert(UiShadowViewBindGroup::new(value));
    }
}

pub fn prepare_shadows(
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
    draw_functions: Res<DrawFunctions<TransparentUi>>,
    mut commands: Commands,
    mut ui_meta: ResMut<UiShadowMeta>,
    mut extracted_ui_instances: ResMut<ExtractedUiShadows>,
    mut render_phases: ResMut<ViewSortedRenderPhases<TransparentUi>>,
    // maps `main entity` to `render entity`
    mut live_entities: Local<MainEntityHashMap<Entity>>,
    mut cached_draw_function: Local<Option<DrawFunctionId>>,
) {
    ui_meta.instance_buffer.clear();

    let draw_function =
        *cached_draw_function.get_or_insert_with(|| draw_functions.read().id::<DrawShadows>());

    let mut batches = EntityHashMap::<UiShadowBatch>::with_capacity(live_entities.capacity());

    for (item, instance) in render_phases.filter(draw_function).filter_map(|item| {
        extracted_ui_instances
            .instances
            .get(item.extracted_index)
            // SAFETY: if remove the filter
            // .filter(|extracted_ui_instance| extracted_ui_instance.entity.0 == item.entity())
            .map(|extracted_ui_instance| (item, extracted_ui_instance.instance))
    }) {
        let render_entity = *live_entities
            .entry(item.main_entity())
            .or_insert_with(|| item.entity());

        let index = ui_meta.instance_buffer.push(instance) as u32;

        batches
            .entry(render_entity)
            .and_modify(|batch| {
                batch.range.end = index + 1;
            })
            .or_insert_with(|| {
                // only the first phase needs to be updated
                // phases under the same entity will be batch processed
                item.batch_range_mut().end += 1;
                UiShadowBatch::new(index..index + 1)
            });
    }

    ui_meta
        .instance_buffer
        .write_buffer(&render_device, &render_queue);

    commands.try_insert_batch(batches);

    extracted_ui_instances.instances.clear();
    live_entities.clear();
}
