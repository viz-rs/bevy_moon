mod plugin;

pub mod prelude {
    pub use crate::plugin::MoonPlugin;

    pub use bevy_moon_core::prelude::*;
    pub use bevy_moon_render::prelude::*;
}
