use bevy_app::Plugin;
use bevy_asset::embedded_asset;
use bevy_ecs::schedule::IntoScheduleConfigs;
use bevy_render::{
    ExtractSchedule, Render, RenderApp, RenderStartup, RenderSystems,
    extract_resource::extract_resource,
    render_phase::{AddRenderCommand, DrawFunctions, ViewSortedRenderPhases, sort_phase_system},
    render_resource::SpecializedRenderPipelines,
};

use bevy_moon_core::prelude::UiStackMap;
use bevy_shader::load_shader_library;

use crate::{transparent::TransparentUi, view::extract_camera_views};

use super::{
    draw::DrawUi,
    extract::{ExtractedUiInstances, UiMeta},
    pipeline::{UiPipeline, init_ui_pipeline},
    render::{prepare_div_view_bind_groups, prepare_divs, queue_divs},
    systems::{ExtractUiSystems, extract_divs, extract_images, extract_texts},
};

pub struct MoonPrimitiveRenderPlugin;

impl Plugin for MoonPrimitiveRenderPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        embedded_asset!(app, "../../shaders/primitive.wgsl");

        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            .init_resource::<SpecializedRenderPipelines<UiPipeline>>()
            .init_resource::<ViewSortedRenderPhases<TransparentUi>>()
            .init_resource::<DrawFunctions<TransparentUi>>()
            .add_render_command::<TransparentUi, DrawUi>();

        render_app
            .init_resource::<UiMeta>()
            .init_resource::<ExtractedUiInstances>();

        render_app.configure_sets(
            ExtractSchedule,
            (
                ExtractUiSystems::CameraViews,
                ExtractUiSystems::Divs,
                ExtractUiSystems::Images,
                ExtractUiSystems::Texts,
            )
                .chain()
                .after(extract_resource::<UiStackMap, ()>),
        );

        render_app.add_systems(RenderStartup, init_ui_pipeline);

        render_app.add_systems(
            ExtractSchedule,
            (
                // @TODO(fundon): set a correct subview index
                extract_camera_views::<{ u32::MAX }>.in_set(ExtractUiSystems::CameraViews),
                extract_divs.in_set(ExtractUiSystems::Divs),
                extract_images.in_set(ExtractUiSystems::Images),
                extract_texts.in_set(ExtractUiSystems::Texts),
            ),
        );

        render_app.add_systems(
            Render,
            (
                queue_divs.in_set(RenderSystems::Queue),
                sort_phase_system::<TransparentUi>.in_set(RenderSystems::PhaseSort),
                prepare_div_view_bind_groups.in_set(RenderSystems::PrepareBindGroups),
                prepare_divs.in_set(RenderSystems::PrepareBindGroups),
            ),
        );
    }
}
