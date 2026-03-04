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

use crate::pipelines::UiTextureBindGroups;

use super::{UiAtlasBatch, UiAtlasMeta, UiAtlasViewBindGroup};

pub type DrawUiAtlas = (
    SetItemPipeline,
    SetUiViewBindGroup<0>,
    SetUiTextureBindGroup<1>,
    DrawUiAtlasBatch,
);

pub struct SetUiViewBindGroup<const I: usize>;

impl<P: PhaseItem, const I: usize> RenderCommand<P> for SetUiViewBindGroup<I> {
    type Param = ();
    type ViewQuery = (Read<ViewUniformOffset>, Read<UiAtlasViewBindGroup>);
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

pub struct SetUiTextureBindGroup<const I: usize>;

impl<P: PhaseItem, const I: usize> RenderCommand<P> for SetUiTextureBindGroup<I> {
    type Param = SRes<UiTextureBindGroups>;
    type ViewQuery = ();
    type ItemQuery = Read<UiAtlasBatch>;

    #[inline]
    fn render<'w>(
        _item: &P,
        _view: ROQueryItem<'w, '_, Self::ViewQuery>,
        batch: Option<ROQueryItem<'w, '_, Self::ItemQuery>>,
        texture_bind_groups: SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        let Some(batch) = batch else {
            return RenderCommandResult::Skip;
        };

        let Some(texture) = texture_bind_groups.into_inner().values.get(&batch.texture) else {
            return RenderCommandResult::Failure("missing texture to draw ui");
        };

        pass.set_bind_group(I, texture, &[]);

        RenderCommandResult::Success
    }
}

pub struct DrawUiAtlasBatch;

impl<P: PhaseItem> RenderCommand<P> for DrawUiAtlasBatch {
    type Param = SRes<UiAtlasMeta>;
    type ViewQuery = ();
    type ItemQuery = Read<UiAtlasBatch>;

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

        let UiAtlasMeta { instance_buffer } = ui_meta.into_inner();

        let Some(instances) = instance_buffer.buffer() else {
            return RenderCommandResult::Failure("missing vertices to draw ui");
        };

        pass.set_vertex_buffer(0, instances.slice(..));
        pass.draw(0..4, batch.range.clone());

        RenderCommandResult::Success
    }
}
