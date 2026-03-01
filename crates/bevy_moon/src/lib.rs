// Copyright © Fangdun Tsai <fundon@pindash.io>
// SPDX-License-Identifier: Apache-2.0 OR MIT

mod plugin;

pub mod prelude {
    pub use crate::plugin::MoonPlugin;

    pub use bevy_moon_core::prelude::*;
    pub use bevy_moon_render::prelude::*;
}
