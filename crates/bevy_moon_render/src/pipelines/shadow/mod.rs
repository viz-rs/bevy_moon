use crate::pipelines::{ExtractedUiInstances, UiBatch, UiMeta, UiViewBindGroup};

use self::{extract::UiShadow, pipeline::UiShadowsPipeline};

mod draw;
mod extract;
mod pipeline;
mod render;
mod systems;

pub mod plugin;

pub(self) type UiShadowsMeta = UiMeta<UiShadow>;
pub(self) type UiShadowsBatch = UiBatch<UiShadow>;
pub(self) type UiShadowsViewBindGroup = UiViewBindGroup<UiShadowsPipeline>;
pub(self) type ExtractedUiShadows = ExtractedUiInstances<UiShadow>;
