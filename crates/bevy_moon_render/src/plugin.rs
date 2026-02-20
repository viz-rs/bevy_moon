use bevy_app::Plugin;
use bevy_render::{
    RenderApp, extract_resource::ExtractResourcePlugin, sync_world::SyncToRenderWorld,
};
use bevy_shader::load_shader_library;

use bevy_moon_core::prelude::{Div, UiStackMap};

use crate::{
    pipelines::primitive::plugin::MoonPrimitiveRenderPlugin, render_pass::add_moon_ui_pass,
};

pub struct MoonRenderPlugin;

impl Plugin for MoonRenderPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        load_shader_library!(app, "shaders/corners.wgsl");
        load_shader_library!(app, "shaders/utils.wgsl");

        app.register_required_components::<Div, SyncToRenderWorld>();

        app.add_plugins(ExtractResourcePlugin::<UiStackMap>::default());

        app.add_plugins(MoonPrimitiveRenderPlugin);

        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        add_moon_ui_pass(render_app);
    }
}
