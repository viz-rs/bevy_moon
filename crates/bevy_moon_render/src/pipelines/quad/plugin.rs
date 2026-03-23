use bevy_app::Plugin;
use bevy_asset::embedded_asset;
use bevy_ecs::schedule::IntoScheduleConfigs;
use bevy_render::{
    ExtractSchedule, Render, RenderApp, RenderStartup, RenderSystems,
    render_phase::{AddRenderCommand, sort_phase_system},
    render_resource::SpecializedRenderPipelines,
};

use crate::{pipelines::ExtractUiSystems, transparent::TransparentUi};

use super::{
    ExtractedUiQuads, UiQuadMeta,
    draw::DrawUiQuad,
    pipeline::{UiQuadPipeline, init_ui_quad_pipeline},
    render::{prepare_quads, prepare_view_bind_groups, queue_quads},
    systems::extract_quads,
};

pub struct MoonQuadRenderPlugin;

impl Plugin for MoonQuadRenderPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        embedded_asset!(app, "../../shaders/quad.wgsl");

        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            .init_resource::<UiQuadMeta>()
            .init_resource::<ExtractedUiQuads>()
            .init_resource::<SpecializedRenderPipelines<UiQuadPipeline>>()
            .add_render_command::<TransparentUi, DrawUiQuad>()
            .add_systems(RenderStartup, init_ui_quad_pipeline)
            .add_systems(
                ExtractSchedule,
                extract_quads.in_set(ExtractUiSystems::Quads),
            )
            .add_systems(
                Render,
                (
                    queue_quads
                        .in_set(RenderSystems::Queue)
                        .before(sort_phase_system::<TransparentUi>),
                    prepare_view_bind_groups.in_set(RenderSystems::PrepareBindGroups),
                    prepare_quads.in_set(RenderSystems::PrepareBindGroups),
                ),
            );
    }
}
