use bevy_ecs::{
    query::ROQueryItem,
    system::{
        SystemParamItem,
        lifetimeless::{Read, SRes},
    },
};
use bevy_render::{
    render_phase::{
        PhaseItem, RenderCommand, RenderCommandResult, SetItemPipeline, TrackedRenderPass,
    },
    view::ViewUniformOffset,
};

use super::{UiQuadBatch, UiQuadMeta, UiQuadViewBindGroup};

pub type DrawUiQuad = (SetItemPipeline, SetUiViewBindGroup<0>, DrawUiQuadBatch);

pub struct SetUiViewBindGroup<const I: usize>;

impl<P: PhaseItem, const I: usize> RenderCommand<P> for SetUiViewBindGroup<I> {
    type Param = ();
    type ViewQuery = (Read<ViewUniformOffset>, Read<UiQuadViewBindGroup>);
    type ItemQuery = ();

    #[inline]
    fn render<'w>(
        _item: &P,
        (view_uniform, ui_view_bind_group): ROQueryItem<'w, '_, Self::ViewQuery>,
        _entity: Option<ROQueryItem<'w, '_, Self::ItemQuery>>,
        _param: SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        pass.set_bind_group(I, &ui_view_bind_group.value, &[view_uniform.offset]);
        RenderCommandResult::Success
    }
}

pub struct DrawUiQuadBatch;

impl<P: PhaseItem> RenderCommand<P> for DrawUiQuadBatch {
    type Param = SRes<UiQuadMeta>;
    type ViewQuery = ();
    type ItemQuery = Read<UiQuadBatch>;

    #[inline]
    fn render<'w>(
        _item: &P,
        _view: ROQueryItem<'w, '_, Self::ViewQuery>,
        batch: Option<ROQueryItem<'w, '_, Self::ItemQuery>>,
        ui_meta: SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        let Some(batch) = batch else {
            return RenderCommandResult::Skip;
        };

        let UiQuadMeta { instance_buffer } = ui_meta.into_inner();

        let Some(instances) = instance_buffer.buffer() else {
            return RenderCommandResult::Failure("missing vertices to draw ui");
        };

        pass.set_vertex_buffer(0, instances.slice(..));
        pass.draw(0..4, batch.range.clone());

        RenderCommandResult::Success
    }
}
