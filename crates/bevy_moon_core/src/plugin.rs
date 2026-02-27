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
    prelude::Text,
    stack::UiStackMap,
    systems::{
        AmbiguousWithText, AmbiguousWithUpdateText2dLayout, UiSystems, measure_text_system,
        text_system, ui_layout_system, ui_stack_system,
    },
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
                ui_stack_system
                    .in_set(UiSystems::Stack)
                    // These systems don't care about stack index
                    .ambiguous_with(measure_text_system)
                    .in_set(AmbiguousWithText),
                ui_layout_system
                    .in_set(UiSystems::Layout)
                    .before(TransformSystems::Propagate)
                    // Text and Text2D operate on disjoint sets of entities
                    .ambiguous_with(bevy_sprite::update_text2d_layout)
                    .ambiguous_with(bevy_text::detect_text_needs_rerender::<bevy_sprite::Text2d>),
            ),
        );

        // text interop
        {
            app.add_systems(
                PostUpdate,
                (
                    (
                        bevy_text::detect_text_needs_rerender::<Text>,
                        measure_text_system,
                    )
                        .chain()
                        .after(bevy_text::load_font_assets_into_font_collection)
                        .in_set(UiSystems::Content)
                        // Text and Text2d are independent.
                        .ambiguous_with(
                            bevy_text::detect_text_needs_rerender::<bevy_sprite::Text2d>,
                        )
                        // Potential conflict: `Assets<Image>`
                        // Since both systems will only ever insert new [`Image`] assets,
                        // they will never observe each other's effects.
                        .ambiguous_with(bevy_sprite::update_text2d_layout),
                    text_system
                        .in_set(UiSystems::PostLayout)
                        .after(bevy_text::load_font_assets_into_font_collection)
                        .after(bevy_asset::AssetEventSystems)
                        // Text2d and bevy_moon ui text are entirely on separate entities
                        .ambiguous_with(
                            bevy_text::detect_text_needs_rerender::<bevy_sprite::Text2d>,
                        )
                        .ambiguous_with(bevy_sprite::update_text2d_layout)
                        .ambiguous_with(bevy_sprite::calculate_bounds_text2d),
                ),
            );

            // app.add_plugins(accessibilit::AccessibilityPlugin);

            app.configure_sets(PostUpdate, AmbiguousWithText.ambiguous_with(text_system));

            app.configure_sets(
                PostUpdate,
                AmbiguousWithUpdateText2dLayout.ambiguous_with(bevy_sprite::update_text2d_layout),
            );
        }

        app.world_mut()
            .register_component_hooks::<Div>()
            .on_add(add_visibility_class::<Div>);
    }
}
