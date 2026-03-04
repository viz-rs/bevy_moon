use bevy_app::{App, Plugin};
use bevy_asset::embedded_asset;
use bevy_ecs::schedule::IntoScheduleConfigs;
use bevy_render::{
    ExtractSchedule, Render, RenderApp, RenderStartup, RenderSystems,
    render_phase::{AddRenderCommand, sort_phase_system},
    render_resource::SpecializedRenderPipelines,
};

use crate::{prelude::ExtractUiSystems, transparent::TransparentUi};

use super::{
    ExtractedUiAtlases, UiAtlasMeta,
    draw::DrawUiAtlas,
    pipeline::{UiAtlasPipeline, init_ui_atlas_pipeline},
    render::{prepare_atlases, prepare_view_bind_groups, queue_atlases},
    systems::{extract_images, extract_texts},
};

pub struct MoonAtlasRenderPlugin;

impl Plugin for MoonAtlasRenderPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "../../shaders/atlas.wgsl");

        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            .init_resource::<UiAtlasMeta>()
            .init_resource::<ExtractedUiAtlases>()
            .init_resource::<SpecializedRenderPipelines<UiAtlasPipeline>>()
            .add_render_command::<TransparentUi, DrawUiAtlas>()
            .add_systems(RenderStartup, init_ui_atlas_pipeline)
            .add_systems(
                ExtractSchedule,
                (
                    extract_images.in_set(ExtractUiSystems::Images),
                    extract_texts.in_set(ExtractUiSystems::Texts),
                ),
            )
            .add_systems(
                Render,
                (
                    queue_atlases
                        .in_set(RenderSystems::Queue)
                        .before(sort_phase_system::<TransparentUi>),
                    prepare_view_bind_groups.in_set(RenderSystems::PrepareBindGroups),
                    prepare_atlases.in_set(RenderSystems::PrepareBindGroups),
                ),
            );
    }
}
