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
    ExtractedUiShadows, UiShadowMeta,
    draw::DrawUiShadow,
    pipeline::{UiShadowPipeline, init_ui_shadow_pipeline},
    render::{prepare_shadows, prepare_view_bind_groups, queue_shadows},
    systems::extract_shadows,
};

pub struct MoonShadowRenderPlugin;

impl Plugin for MoonShadowRenderPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "../../shaders/shadow.wgsl");

        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            .init_resource::<UiShadowMeta>()
            .init_resource::<ExtractedUiShadows>()
            .init_resource::<SpecializedRenderPipelines<UiShadowPipeline>>()
            .add_render_command::<TransparentUi, DrawUiShadow>()
            .add_systems(RenderStartup, init_ui_shadow_pipeline)
            .add_systems(
                ExtractSchedule,
                extract_shadows.in_set(ExtractUiSystems::Shadows),
            )
            .add_systems(
                Render,
                (
                    queue_shadows
                        .in_set(RenderSystems::Queue)
                        .before(sort_phase_system::<TransparentUi>),
                    prepare_view_bind_groups.in_set(RenderSystems::PrepareBindGroups),
                    prepare_shadows.in_set(RenderSystems::PrepareBindGroups),
                ),
            );
    }
}
