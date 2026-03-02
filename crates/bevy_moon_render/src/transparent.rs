use std::ops::Range;

use bevy_ecs::entity::{Entity, EntityHash};
use bevy_math::FloatOrd;
use bevy_render::{
    render_phase::{
        CachedRenderPipelinePhaseItem, DrawFunctionId, PhaseItem, PhaseItemExtraIndex,
        SortedPhaseItem, ViewSortedRenderPhases,
    },
    render_resource::CachedRenderPipelineId,
    sync_world::MainEntity,
    view::ExtractedView,
};
use indexmap::IndexMap;

/// Transparent UI [`SortedPhaseItem`]s.
pub struct TransparentUi {
    pub sort_key: FloatOrd,
    pub entity: (Entity, MainEntity),
    pub pipeline: CachedRenderPipelineId,
    pub draw_function: DrawFunctionId,
    pub batch_range: Range<u32>,
    pub extra_index: PhaseItemExtraIndex,
    pub indexed: bool,
    pub extracted_index: usize,
}

impl PhaseItem for TransparentUi {
    #[inline]
    fn entity(&self) -> Entity {
        self.entity.0
    }

    #[inline]
    fn main_entity(&self) -> MainEntity {
        self.entity.1
    }

    #[inline]
    fn draw_function(&self) -> DrawFunctionId {
        self.draw_function
    }

    #[inline]
    fn batch_range(&self) -> &Range<u32> {
        &self.batch_range
    }

    #[inline]
    fn batch_range_mut(&mut self) -> &mut Range<u32> {
        &mut self.batch_range
    }

    #[inline]
    fn extra_index(&self) -> PhaseItemExtraIndex {
        self.extra_index.clone()
    }

    #[inline]
    fn batch_range_and_extra_index_mut(&mut self) -> (&mut Range<u32>, &mut PhaseItemExtraIndex) {
        (&mut self.batch_range, &mut self.extra_index)
    }
}

impl SortedPhaseItem for TransparentUi {
    type SortKey = FloatOrd;

    #[inline]
    fn sort_key(&self) -> Self::SortKey {
        self.sort_key
    }

    #[inline]
    fn sort(items: &mut IndexMap<(Entity, MainEntity), Self, EntityHash>) {
        items.sort_by_key(|_, value| value.sort_key());
    }

    fn recalculate_sort_keys(
        _: &mut IndexMap<(Entity, MainEntity), Self, EntityHash>,
        _: &ExtractedView,
    ) {
        // Sort keys are precalculated for UI phase items.
    }

    #[inline]
    fn indexed(&self) -> bool {
        self.indexed
    }
}

impl CachedRenderPipelinePhaseItem for TransparentUi {
    #[inline]
    fn cached_pipeline(&self) -> CachedRenderPipelineId {
        self.pipeline
    }
}

/// A [`ViewSortedRenderPhases`] filter trait.
pub trait RenderPhasesFilter<I>
where
    I: SortedPhaseItem,
{
    /// Filters the render phases based on the given draw function id.
    fn filter(&mut self, draw_function_id: DrawFunctionId) -> impl Iterator<Item = &mut I>;
}

impl<I> RenderPhasesFilter<I> for ViewSortedRenderPhases<I>
where
    I: SortedPhaseItem,
{
    fn filter(&mut self, draw_function_id: DrawFunctionId) -> impl Iterator<Item = &mut I> {
        let filter = move |item: &&mut I| item.draw_function() == draw_function_id;

        self.values_mut()
            .flat_map(move |phase| phase.items.values_mut().filter(filter))
    }
}
