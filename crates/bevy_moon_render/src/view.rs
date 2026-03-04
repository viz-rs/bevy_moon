use bevy_ecs::{component::Component, entity::Entity};
use bevy_sprite_render::Mesh2dPipelineKey;

/// A render-world component that lives on the main render target view and
/// specifies the corresponding moon ui view.
///
/// For example, if moon ui is being rendered to a 3D camera, this component lives on
/// the 3D camera and contains the entity corresponding to the moon ui view.
///
/// Entity id of the temporary render entity with the corresponding extracted moon ui view.
#[derive(Component, Debug)]
pub struct MoonUiCameraView(pub Entity);

/// A render-world component that lives on the moon ui view and specifies the
/// corresponding main render target view.
///
/// For example, if moon ui is being rendered to a 3D camera, this component
/// lives on the moon ui view and contains the entity corresponding to the 3D camera.
///
/// This is the inverse of [`MoonUiCameraView`].
#[derive(Component, Debug)]
pub struct MoonUiViewTarget(pub Entity);

/// Caches the mesh key for the moon ui view.
#[derive(Component, Debug)]
pub struct MoonUiOptions(pub Mesh2dPipelineKey);
