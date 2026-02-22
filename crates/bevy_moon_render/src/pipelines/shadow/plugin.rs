use bevy_app::{App, Plugin};
use bevy_asset::embedded_asset;
use bevy_ecs::schedule::IntoScheduleConfigs;
use bevy_render::{
    ExtractSchedule, Render, RenderApp, RenderStartup, RenderSystems,
    render_phase::AddRenderCommand, render_resource::SpecializedRenderPipelines,
};

use crate::{prelude::ExtractUiSystems, transparent::TransparentUi};

use super::{
    ExtractedUiShadows, UiShadowsMeta,
    draw::DrawShadows,
    pipeline::{UiShadowsPipeline, init_shadows_pipeline},
    render::{prepare_div_view_bind_groups, prepare_shadows, queue_shadows},
    systems::extract_shadows,
};

pub struct MoonShadowRenderPlugin;

impl Plugin for MoonShadowRenderPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "../../shaders/shadows.wgsl");

        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            .add_render_command::<TransparentUi, DrawShadows>()
            .init_resource::<UiShadowsMeta>()
            .init_resource::<ExtractedUiShadows>()
            .init_resource::<SpecializedRenderPipelines<UiShadowsPipeline>>()
            .add_systems(RenderStartup, init_shadows_pipeline)
            .add_systems(
                ExtractSchedule,
                extract_shadows.in_set(ExtractUiSystems::Shadows),
            )
            .add_systems(
                Render,
                (
                    queue_shadows.in_set(RenderSystems::Queue),
                    prepare_div_view_bind_groups.in_set(RenderSystems::PrepareBindGroups),
                    prepare_shadows.in_set(RenderSystems::PrepareBindGroups),
                ),
            );
    }
}
