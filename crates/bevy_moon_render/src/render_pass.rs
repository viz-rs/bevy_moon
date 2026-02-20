use bevy_app::SubApp;
use bevy_core_pipeline::{Core2d, Core2dSystems, Core3d, upscaling::upscaling};
use bevy_ecs::{prelude::Res, schedule::IntoScheduleConfigs, system::Query, world::World};
use bevy_render::{
    camera::ExtractedCamera,
    diagnostic::RecordDiagnostics,
    render_phase::ViewSortedRenderPhases,
    render_resource::RenderPassDescriptor,
    renderer::{RenderContext, ViewQuery},
    view::{ExtractedView, ViewTarget},
};

use crate::{
    transparent::TransparentUi,
    view::{MoonUiCameraView, MoonUiViewTarget},
};

// Adds moon ui pass the 2D/3D.
pub fn add_moon_ui_pass(render_app: &mut SubApp) {
    render_app.add_systems(
        Core2d,
        ui_pass.after(Core2dSystems::PostProcess).before(upscaling),
    );

    render_app.add_systems(
        Core3d,
        ui_pass.after(Core2dSystems::PostProcess).before(upscaling),
    );
}

pub fn ui_pass(
    world: &World,
    view: ViewQuery<&MoonUiCameraView>,
    ui_view_query: Query<(&ExtractedView, &MoonUiViewTarget)>,
    ui_view_target_query: Query<(&ViewTarget, &ExtractedCamera)>,
    transparent_render_phases: Res<ViewSortedRenderPhases<TransparentUi>>,
    mut ctx: RenderContext,
) {
    let ui_camera_view = view.into_inner();
    let ui_view_entity = ui_camera_view.0;

    let Ok((extracted_view, ui_view_target)) = ui_view_query.get(ui_view_entity) else {
        return;
    };

    let Ok((target, camera)) = ui_view_target_query.get(ui_view_target.0) else {
        return;
    };

    let Some(transparent_phase) =
        transparent_render_phases.get(&extracted_view.retained_view_entity)
    else {
        return;
    };

    if transparent_phase.items.is_empty() {
        return;
    }

    let diagnostics = ctx.diagnostic_recorder();
    let diagnostics = diagnostics.as_deref();

    let color_attachment = target.get_color_attachment(); // sample count 4
    // let color_attachment = target.get_unsampled_color_attachment(); // sample count 1
    let depth_stencil_attachment = None;

    let mut render_pass = ctx.begin_tracked_render_pass(RenderPassDescriptor {
        label: Some("moon ui"),
        color_attachments: &[Some(color_attachment)],
        depth_stencil_attachment,
        timestamp_writes: None,
        occlusion_query_set: None,
        multiview_mask: None,
    });

    let pass_span = diagnostics.pass_span(&mut render_pass, "moon ui");

    if let Some(viewport) = camera.viewport.as_ref() {
        render_pass.set_camera_viewport(viewport);
    }

    if let Err(err) = transparent_phase.render(&mut render_pass, world, ui_view_entity) {
        tracing::error!("Error encountered while rendering the ui phase {err:?}");
    }

    pass_span.end(&mut render_pass);
}
