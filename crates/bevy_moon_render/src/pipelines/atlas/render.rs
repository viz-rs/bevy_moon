use bevy_asset::{AssetEvent, AssetId};
use bevy_ecs::{
    entity::{Entity, EntityHashMap},
    query::With,
    system::{Commands, Local, Query, Res, ResMut},
};
use bevy_math::FloatOrd;
use bevy_moon_core::prelude::UiStackMap;
use bevy_render::{
    render_asset::RenderAssets,
    render_phase::{
        DrawFunctionId, DrawFunctions, PhaseItem, PhaseItemExtraIndex, ViewSortedRenderPhases,
    },
    render_resource::{BindGroupEntries, PipelineCache, SpecializedRenderPipelines},
    renderer::{RenderDevice, RenderQueue},
    sync_world::MainEntity,
    texture::GpuImage,
    view::{ExtractedView, ViewUniforms},
};
use bevy_sprite_render::SpriteAssetEvents;
use indexmap::IndexMap;

use crate::{
    pipelines::{UiTextureBindGroups, atlas::ExtractedUiAtlases},
    transparent::{RenderPhasesFilter, TransparentUi},
    view::{MoonUiCameraView, MoonUiOptions, MoonUiViewTarget},
};

use super::{
    UiAtlasBatch, UiAtlasMeta, UiAtlasViewBindGroup,
    draw::DrawUiAtlas,
    pipeline::{UiAtlasPipeline, UiAtlasPipelineKey},
};

pub fn queue_atlases(
    render_targets: Query<(MainEntity, &MoonUiCameraView, &MoonUiOptions)>,
    render_views: Query<&ExtractedView, With<MoonUiViewTarget>>,
    extracted_ui_atlases: Res<ExtractedUiAtlases>,
    ui_stack_map: Res<UiStackMap>,
    ui_atlas_pipeline: Res<UiAtlasPipeline>,
    pipeline_cache: Res<PipelineCache>,
    draw_functions: Res<DrawFunctions<TransparentUi>>,
    mut pipelines: ResMut<SpecializedRenderPipelines<UiAtlasPipeline>>,
    mut render_phases: ResMut<ViewSortedRenderPhases<TransparentUi>>,
) {
    let draw_function = draw_functions.read().id::<DrawUiAtlas>();

    for (extracted_index, div) in extracted_ui_atlases.instances.iter().enumerate() {
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
            &ui_atlas_pipeline,
            UiAtlasPipelineKey { mesh_key },
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
    ui_atlas_pipeline: Res<UiAtlasPipeline>,
    view_uniforms: Res<ViewUniforms>,
    views: Query<Entity, (With<ExtractedView>, With<MoonUiViewTarget>)>,
) {
    let Some(view_binding) = view_uniforms.uniforms.binding() else {
        return;
    };

    for entity in &views {
        let value = render_device.create_bind_group(
            "moon_ui_atlas_view_layout",
            &pipeline_cache.get_bind_group_layout(&ui_atlas_pipeline.view_layout),
            &BindGroupEntries::single(view_binding.clone()),
        );

        commands
            .entity(entity)
            .insert(UiAtlasViewBindGroup::new(value));
    }
}

pub fn prepare_atlases(
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
    pipeline_cache: Res<PipelineCache>,
    ui_atlas_pipeline: Res<UiAtlasPipeline>,
    gpu_images: Res<RenderAssets<GpuImage>>,
    events: Res<SpriteAssetEvents>,
    draw_functions: Res<DrawFunctions<TransparentUi>>,
    mut commands: Commands,
    mut ui_atlas_meta: ResMut<UiAtlasMeta>,
    mut extracted_ui_atlases: ResMut<ExtractedUiAtlases>,
    mut texture_bind_groups: ResMut<UiTextureBindGroups>,
    mut render_phases: ResMut<ViewSortedRenderPhases<TransparentUi>>,
    // maps `main entity - texture` to `render entity`
    mut live_entities: Local<IndexMap<(MainEntity, AssetId<bevy_image::Image>), Entity>>,
    mut cached_draw_function: Local<Option<DrawFunctionId>>,
) {
    // If an image has changed, the GpuImage has (probably) changed
    for event in &events.images {
        match event {
          AssetEvent::Added { .. } |
          AssetEvent::Unused { .. } |
          // Images don't have dependencies
          AssetEvent::LoadedWithDependencies { .. } => {}
          AssetEvent::Modified { id } | AssetEvent::Removed { id } => {
              texture_bind_groups.values.remove(id);
          }
      };
    }

    ui_atlas_meta.instance_buffer.clear();

    let draw_function =
        *cached_draw_function.get_or_insert_with(|| draw_functions.read().id::<DrawUiAtlas>());

    let mut batches = EntityHashMap::<UiAtlasBatch>::with_capacity(live_entities.capacity());

    for (item, (instance, texture)) in render_phases.filter(draw_function).filter_map(|item| {
        extracted_ui_atlases
            .instances
            .get(item.extracted_index)
            // SAFETY: if remove the filter
            // .filter(|extracted_ui_instance| extracted_ui_instance.entity.0 == item.entity())
            .map(|extracted_ui_instance| {
                (
                    item,
                    (
                        extracted_ui_instance.instance,
                        extracted_ui_instance.texture,
                    ),
                )
            })
    }) {
        let Some(gpu_image) = (texture != AssetId::invalid())
            .then(|| gpu_images.get(texture))
            .flatten()
        else {
            continue;
        };

        let index = ui_atlas_meta.instance_buffer.push(instance) as u32;

        texture_bind_groups
            .values
            .entry(texture)
            .or_insert_with(|| {
                render_device.create_bind_group(
                    "ui_atlas_texture_bind_group",
                    &pipeline_cache.get_bind_group_layout(&ui_atlas_pipeline.texture_layout),
                    &BindGroupEntries::sequential((&gpu_image.texture_view, &gpu_image.sampler)),
                )
            });

        let key = (item.main_entity(), texture);

        // Clears the live entities when a new texture is assigned and keeps its capacity.
        // This is also used to handle multiple spans with multiple fonts in same text container.
        //
        // An example for textures order: [A, A, B, A, C, B, B, B, C]
        //                                 ↓     ↓     ↓  ↓        ↓
        if !live_entities.is_empty() && !live_entities.contains_key(&key) {
            live_entities.clear();
        }

        let render_entity = live_entities.entry(key).or_insert_with(|| item.entity());

        batches
            .entry(*render_entity)
            .and_modify(|batch| {
                batch.range.end = index + 1;
                // updates it with real texture
                if batch.texture == texture {
                    return;
                }
                batch.texture = texture;
            })
            .or_insert_with(|| {
                // only the first phase needs to be updated
                // phases under the same entity will be batch processed
                item.batch_range_mut().end += 1;
                UiAtlasBatch::new(index..index + 1).with_texture(texture)
            });
    }

    ui_atlas_meta
        .instance_buffer
        .write_buffer(&render_device, &render_queue);

    commands.try_insert_batch(batches);

    extracted_ui_atlases.instances.clear();
    live_entities.clear();
}
