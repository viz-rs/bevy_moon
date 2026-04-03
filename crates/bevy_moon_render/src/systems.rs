use bevy_asset::AssetEvent;
use bevy_camera::{Camera, Camera2d, Camera3d, Hdr};
use bevy_ecs::{
    entity::Entity,
    query::{Has, Or, With},
    system::{Commands, Local, Query, Res, ResMut},
};
use bevy_math::{URect, UVec4};
use bevy_platform::collections::HashSet;
use bevy_render::{
    Extract,
    render_phase::ViewSortedRenderPhases,
    sync_world::{RenderEntity, TemporaryRenderEntity},
    view::{ExtractedView, Msaa, RetainedViewEntity},
};
use bevy_sprite_render::{Mesh2dPipelineKey, SpriteAssetEvents};
use bevy_transform::components::GlobalTransform;

use crate::{
    pipelines::UiTextureBindGroups,
    transparent::TransparentUi,
    view::{MoonUiCameraView, MoonUiOptions, MoonUiViewTarget},
};

pub fn extract_sprite_events(
    events: Res<SpriteAssetEvents>,
    mut texture_bind_groups: ResMut<UiTextureBindGroups>,
) {
    // If an image has changed, the GpuImage has (probably) changed
    for event in &events.images {
        match event {
          AssetEvent::Added { .. } |
          // Images don't have dependencies
          AssetEvent::LoadedWithDependencies { .. } => {}
          AssetEvent::Unused { id } | AssetEvent::Modified { id } | AssetEvent::Removed { id } => {
              texture_bind_groups.values.remove(id);
          }
      };
    }
}

/// Extracts all moon ui with a camera into the render world.
///
/// | Camera        | Subview Index |
/// | ------------- | ------------- |
/// | Main 2D or 3D | 0             |
/// | UI            | 1             |
/// | egui          | 2095931312    |
/// | Moon UI       | u32::MAX      |
pub fn extract_camera_views<const CAMERA_SUBVIEW: u32>(
    cameras: Extract<
        Query<
            (
                Entity,
                RenderEntity,
                &GlobalTransform,
                &Camera,
                // options
                (Has<Hdr>, &Msaa),
            ),
            Or<(With<Camera2d>, With<Camera3d>)>,
        >,
    >,
    mut commands: Commands,
    mut render_phases: ResMut<ViewSortedRenderPhases<TransparentUi>>,
    mut live_entities: Local<HashSet<RetainedViewEntity>>,
) {
    live_entities.clear();

    for (main_entity, render_entity, &transform, camera, (hdr, msaa)) in &cameras {
        // Ignore inactive cameras.
        if !camera.is_active {
            commands
                .get_entity(render_entity)
                .expect("Camera entity wasn't synced.")
                .remove::<MoonUiCameraView>();
            continue;
        }

        let (
            Some(URect {
                min: viewport_origin,
                ..
            }),
            Some(viewport_size),
        ) = (
            camera.physical_viewport_rect(),
            camera.physical_viewport_size(),
        )
        else {
            continue;
        };

        let retained_view_entity =
            RetainedViewEntity::new(main_entity.into(), None, CAMERA_SUBVIEW);

        let mesh_key =
            Mesh2dPipelineKey::from_hdr(hdr) | Mesh2dPipelineKey::from_msaa_samples(msaa.samples());

        // Creates the UI view.
        let ui_camera_view = commands
            .spawn((
                ExtractedView {
                    retained_view_entity,
                    clip_from_view: camera.clip_from_view(),
                    world_from_view: transform,
                    clip_from_world: None,
                    viewport: UVec4::new(
                        viewport_origin.x,
                        viewport_origin.y,
                        viewport_size.x,
                        viewport_size.y,
                    ),
                    hdr,
                    invert_culling: false,
                    compositing_space: None,
                    color_grading: Default::default(),
                },
                // Link to the main camera view.
                MoonUiViewTarget(render_entity),
                TemporaryRenderEntity,
            ))
            .id();

        commands
            .get_entity(render_entity)
            .expect("Camera entity wasn't synced.")
            // Link from the main 2D/3D camera view to the moon ui view.
            .insert(MoonUiCameraView(ui_camera_view))
            .insert(MoonUiOptions(mesh_key));

        render_phases.prepare_for_new_frame(retained_view_entity);
        live_entities.insert(retained_view_entity);
    }

    render_phases.retain(|camera_entity, _| live_entities.contains(camera_entity));
}
