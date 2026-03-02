use crate::pipelines::{ExtractedUiInstances, UiBatch, UiMeta, UiViewBindGroup};

use self::{extract::UiShadow, pipeline::UiShadowsPipeline};

mod draw;
mod extract;
mod pipeline;
mod render;
mod systems;

pub mod plugin;

pub(crate) type UiShadowMeta = UiMeta<UiShadow>;
pub(crate) type UiShadowBatch = UiBatch<UiShadow>;
pub(crate) type UiShadowViewBindGroup = UiViewBindGroup<UiShadowsPipeline>;
pub(crate) type ExtractedUiShadows = ExtractedUiInstances<UiShadow>;
