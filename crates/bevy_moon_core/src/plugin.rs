use bevy_app::{AnimationSystems, Plugin, PostUpdate};
use bevy_camera::{
    CameraUpdateSystems,
    visibility::{Visibility, VisibilityClass, add_visibility_class},
};
use bevy_ecs::schedule::IntoScheduleConfigs;
use bevy_transform::TransformSystems;

use crate::{
    components::div::Div,
    layout::UiLayoutTree,
    stack::UiStackMap,
    systems::{UiSystems, ui_layout_system, ui_stack_system},
};

pub struct MoonCorePlugin;

impl Plugin for MoonCorePlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.register_required_components::<Div, Visibility>()
            .register_required_components::<Div, VisibilityClass>();

        app.init_resource::<UiStackMap>()
            .init_resource::<UiLayoutTree>();

        app.configure_sets(
            PostUpdate,
            (
                CameraUpdateSystems,
                UiSystems::Prepare.after(AnimationSystems),
                UiSystems::Content,
                UiSystems::Stack,
                UiSystems::Layout,
                UiSystems::PostLayout,
            )
                .chain(),
        );

        app.add_systems(
            PostUpdate,
            (
                ui_stack_system.in_set(UiSystems::Stack),
                ui_layout_system
                    .in_set(UiSystems::Layout)
                    .before(TransformSystems::Propagate),
            ),
        );

        app.world_mut()
            .register_component_hooks::<Div>()
            .on_add(add_visibility_class::<Div>);
    }
}
