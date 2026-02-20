use bevy_app::Plugin;
use bevy_moon_core::prelude::MoonCorePlugin;
use bevy_moon_render::prelude::MoonRenderPlugin;

pub struct MoonPlugin;

impl Plugin for MoonPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.add_plugins((MoonCorePlugin, MoonRenderPlugin));
    }
}
