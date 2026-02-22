use bevy_ecs::{
    entity::{Entity, EntityHashMap},
    query::With,
    system::{Commands, Local, Query, Res, ResMut},
};
use bevy_math::FloatOrd;
use bevy_moon_core::prelude::UiStackMap;
use bevy_render::{
    render_phase::{DrawFunctions, PhaseItem, PhaseItemExtraIndex, ViewSortedRenderPhases},
    render_resource::{BindGroupEntries, PipelineCache, SpecializedRenderPipelines},
    renderer::{RenderDevice, RenderQueue},
    sync_world::MainEntity,
    view::{ExtractedView, ViewUniforms},
};

use crate::{
    transparent::{PipelineFilter, TransparentUi},
    view::{MoonUiCameraView, MoonUiOptions, MoonUiViewTarget},
};

use super::{
    ExtractedUiShadows, UiShadowsBatch, UiShadowsMeta, UiShadowsViewBindGroup,
    draw::DrawShadows,
    pipeline::{UI_SHADOWS_PIPELINE_KEY, UiShadowsPipeline, UiShadowsPipelineKey},
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
    let draw_function = draw_functions.read().id::<DrawShadows>();

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
        let sort_key = FloatOrd(div.index - 0.001);

        render_phase.add(TransparentUi {
            pipeline,
            draw_function,
            extracted_index,
            entity,
            sort_key,
            indexed: true,
            batch_range: 0..0,
            extra_index: PhaseItemExtraIndex::None,
            pipeline_key: UI_SHADOWS_PIPELINE_KEY,
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
            .insert(UiShadowsViewBindGroup::new(value));
    }
}

pub fn prepare_shadows(
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
    mut commands: Commands,
    mut ui_meta: ResMut<UiShadowsMeta>,
    mut extracted_ui_instances: ResMut<ExtractedUiShadows>,
    mut render_phases: ResMut<ViewSortedRenderPhases<TransparentUi>>,
    mut previous_len: Local<usize>,
) {
    ui_meta.instance_buffer.clear();

    let mut batches: EntityHashMap<UiShadowsBatch> = EntityHashMap::with_capacity(*previous_len);

    for item in render_phases.filter(UI_SHADOWS_PIPELINE_KEY) {
        let Some(extracted_ui_instance) = extracted_ui_instances
            .instances
            .get(item.extracted_index)
            .filter(|extracted_ui_instance| extracted_ui_instance.entity.0 == item.entity())
        else {
            continue;
        };

        let instance = extracted_ui_instance.instance;

        let index = ui_meta.instance_buffer.push(instance) as u32;

        batches
            .entry(item.entity())
            .and_modify(|batch| {
                batch.range.end = index + 1;
            })
            .or_insert_with(|| UiShadowsBatch::new(index..index + 1));

        item.batch_range_mut().end += 1;
    }

    ui_meta
        .instance_buffer
        .write_buffer(&render_device, &render_queue);

    *previous_len = batches.len();
    commands.try_insert_batch(batches);

    extracted_ui_instances.instances.clear();
}
