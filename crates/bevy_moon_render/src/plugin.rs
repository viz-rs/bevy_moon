use bevy_app::Plugin;
use bevy_render::{RenderApp, extract_resource::ExtractResourcePlugin};
use bevy_shader::load_shader_library;

use bevy_moon_core::prelude::UiStackMap;

use crate::{pipelines::MoonInternalRenderPlugin, render_pass::add_moon_ui_pass};

pub struct MoonRenderPlugin;

impl Plugin for MoonRenderPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        load_shader_library!(app, "shaders/libs/flags.wgsl");
        load_shader_library!(app, "shaders/libs/quad.wgsl");
        load_shader_library!(app, "shaders/libs/maths.wgsl");
        load_shader_library!(app, "shaders/libs/corners.wgsl");
        load_shader_library!(app, "shaders/libs/rectangles.wgsl");
        load_shader_library!(app, "shaders/libs/atlas.wgsl");
        load_shader_library!(app, "shaders/libs/utils.wgsl");

        app.add_plugins(ExtractResourcePlugin::<UiStackMap>::default());

        app.add_plugins(MoonInternalRenderPlugin);

        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        add_moon_ui_pass(render_app);
    }
}
