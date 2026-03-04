use crate::pipelines::{ExtractedUiInstances, UiBatch, UiMeta, UiViewBindGroup};

use self::{extract::UiAtlas, pipeline::UiAtlasPipeline};

mod draw;
mod extract;
mod pipeline;
mod plugin;
mod render;
mod systems;

pub(crate) type UiAtlasMeta = UiMeta<UiAtlas>;
pub(crate) type UiAtlasBatch = UiBatch<UiAtlas>;
pub(crate) type UiAtlasViewBindGroup = UiViewBindGroup<UiAtlasPipeline>;
pub(crate) type ExtractedUiAtlases = ExtractedUiInstances<UiAtlas>;

pub use plugin::MoonAtlasRenderPlugin;
