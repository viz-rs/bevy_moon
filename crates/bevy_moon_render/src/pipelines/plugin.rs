use bevy_app::Plugin;
use bevy_ecs::schedule::IntoScheduleConfigs;
use bevy_render::{
    ExtractSchedule, Render, RenderApp, RenderSystems,
    extract_resource::extract_resource,
    render_phase::{DrawFunctions, ViewSortedRenderPhases, sort_phase_system},
};

use bevy_moon_core::prelude::UiStackMap;

use crate::{
    pipelines::{ExtractUiSystems, UiTextureBindGroups},
    transparent::TransparentUi,
    view::extract_camera_views,
};

use super::{
    atlas::MoonAtlasRenderPlugin, quad::MoonQuadRenderPlugin, shadow::MoonShadowRenderPlugin,
};

pub struct MoonInternalRenderPlugin;

impl Plugin for MoonInternalRenderPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            .init_resource::<UiTextureBindGroups>()
            .init_resource::<ViewSortedRenderPhases<TransparentUi>>()
            .init_resource::<DrawFunctions<TransparentUi>>();

        render_app.configure_sets(
            ExtractSchedule,
            (
                ExtractUiSystems::CameraViews,
                ExtractUiSystems::Shadows,
                ExtractUiSystems::Divs,
                ExtractUiSystems::Images,
                ExtractUiSystems::Texts,
            )
                .chain()
                .after(extract_resource::<UiStackMap, ()>),
        );

        render_app.add_systems(
            ExtractSchedule,
            // @TODO(fundon): set a correct subview index
            extract_camera_views::<{ u32::MAX }>.in_set(ExtractUiSystems::CameraViews),
        );

        render_app.add_systems(
            Render,
            sort_phase_system::<TransparentUi>.in_set(RenderSystems::PhaseSort),
        );

        app.add_plugins((
            MoonShadowRenderPlugin,
            MoonQuadRenderPlugin,
            MoonAtlasRenderPlugin,
        ));
    }
}
