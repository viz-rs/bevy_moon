use crate::pipelines::{ExtractedUiInstances, UiBatch, UiMeta, UiViewBindGroup};

use self::{extract::UiShadow, pipeline::UiShadowsPipeline};

mod draw;
mod extract;
mod pipeline;
mod render;
mod systems;

pub mod plugin;

pub(self) type UiShadowMeta = UiMeta<UiShadow>;
pub(self) type UiShadowBatch = UiBatch<UiShadow>;
pub(self) type UiShadowViewBindGroup = UiViewBindGroup<UiShadowsPipeline>;
pub(self) type ExtractedUiShadows = ExtractedUiInstances<UiShadow>;
