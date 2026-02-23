use crate::pipelines::{UiBatch, UiMeta, UiViewBindGroup};

use self::{extract::UiInstance, pipeline::UiPipeline};

mod draw;
mod extract;
mod pipeline;
mod render;

pub mod plugin;
pub mod systems;

pub(self) type UiInstanceMeta = UiMeta<UiInstance>;
pub(self) type UiInstanceBatch = UiBatch<UiInstance>;
pub(self) type UiInstanceViewBindGroup = UiViewBindGroup<UiPipeline>;
pub(self) type ExtractedUiInstances = super::ExtractedUiInstances<UiInstance>;
