use crate::pipelines::{UiBatch, UiMeta, UiViewBindGroup};

use self::{extract::UiQuad, pipeline::UiQuadPipeline};

mod draw;
mod extract;
mod pipeline;
mod plugin;
mod render;
mod systems;

pub(crate) type UiQuadMeta = UiMeta<UiQuad>;
pub(crate) type UiQuadBatch = UiBatch<UiQuad>;
pub(crate) type UiQuadViewBindGroup = UiViewBindGroup<UiQuadPipeline>;
pub(crate) type ExtractedUiQuads = super::ExtractedUiInstances<UiQuad>;

pub use plugin::MoonQuadRenderPlugin;
