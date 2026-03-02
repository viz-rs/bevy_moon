use crate::pipelines::{UiBatch, UiMeta, UiViewBindGroup};

use self::{extract::UiInstance, pipeline::UiPipeline};

mod draw;
mod extract;
mod pipeline;
mod render;

pub mod plugin;
pub mod systems;

pub(crate) type UiInstanceMeta = UiMeta<UiInstance>;
pub(crate) type UiInstanceBatch = UiBatch<UiInstance>;
pub(crate) type UiInstanceViewBindGroup = UiViewBindGroup<UiPipeline>;
pub(crate) type ExtractedUiInstances = super::ExtractedUiInstances<UiInstance>;
