use crate::pipelines::{ExtractedUiInstances, UiBatch, UiMeta, UiViewBindGroup};

use self::{extract::UiShadow, pipeline::UiShadowPipeline};

mod draw;
mod extract;
mod pipeline;
mod plugin;
mod render;
mod systems;

pub(crate) type UiShadowMeta = UiMeta<UiShadow>;
pub(crate) type UiShadowBatch = UiBatch<UiShadow>;
pub(crate) type UiShadowViewBindGroup = UiViewBindGroup<UiShadowPipeline>;
pub(crate) type ExtractedUiShadows = ExtractedUiInstances<UiShadow>;

pub use plugin::MoonShadowRenderPlugin;
