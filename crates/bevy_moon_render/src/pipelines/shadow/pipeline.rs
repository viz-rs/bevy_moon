use bevy_asset::{AssetServer, Handle, load_embedded_asset};
use bevy_ecs::{
    resource::Resource,
    system::{Commands, Res},
};
use bevy_image::BevyDefault;
use bevy_mesh::{PrimitiveTopology, VertexBufferLayout, VertexFormat};
use bevy_render::{
    render_resource::{
        BindGroupLayoutDescriptor, BindGroupLayoutEntries, BlendState, ColorTargetState,
        ColorWrites, FragmentState, FrontFace, MultisampleState, PolygonMode, PrimitiveState,
        RenderPipelineDescriptor, ShaderStages, SpecializedRenderPipeline, TextureFormat,
        VertexState, VertexStepMode, binding_types::uniform_buffer,
    },
    view::{ViewTarget, ViewUniform},
};
use bevy_shader::{Shader, ShaderDefVal};
use bevy_sprite_render::Mesh2dPipelineKey;
use bevy_utils::default;

#[derive(Resource, Clone)]
pub struct UiShadowPipeline {
    pub view_layout: BindGroupLayoutDescriptor,
    pub shader: Handle<Shader>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct UiShadowPipelineKey {
    pub mesh_key: Mesh2dPipelineKey,
    /// Number of samples, a higher value results in better quality shadows.
    pub samples: u32,
}

impl SpecializedRenderPipeline for UiShadowPipeline {
    type Key = UiShadowPipelineKey;

    fn specialize(&self, key: Self::Key) -> RenderPipelineDescriptor {
        let shader_defs = vec![ShaderDefVal::UInt(
            "SHADOW_SAMPLES".to_string(),
            key.samples,
        )];

        let mesh_key = key.mesh_key;

        let format = match mesh_key.contains(Mesh2dPipelineKey::HDR) {
            true => ViewTarget::TEXTURE_FORMAT_HDR,
            false => TextureFormat::bevy_default(),
        };
        let count = mesh_key.msaa_samples();

        let layout = vec![self.view_layout.clone()];

        let vertex_layout = VertexBufferLayout::from_vertex_formats(
            VertexStepMode::Instance,
            vec![
                // x_axis
                VertexFormat::Float32x4,
                // y_axis
                VertexFormat::Float32x4,
                // z_axis
                VertexFormat::Float32x4,
                // translation
                VertexFormat::Float32x4,
                // color
                VertexFormat::Float32x4,
                // size
                VertexFormat::Float32x2,
                // corner_radii
                VertexFormat::Float32x4,
                // blur_radius
                VertexFormat::Float32,
            ],
        );

        RenderPipelineDescriptor {
            vertex: VertexState {
                shader: self.shader.clone(),
                shader_defs: shader_defs.clone(),
                buffers: vec![vertex_layout],
                ..default()
            },
            fragment: Some(FragmentState {
                shader: self.shader.clone(),
                shader_defs,
                targets: vec![Some(ColorTargetState {
                    format,
                    blend: Some(BlendState::ALPHA_BLENDING),
                    write_mask: ColorWrites::ALL,
                })],
                ..default()
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleStrip,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: PolygonMode::Fill,
                conservative: false,
                unclipped_depth: false,
            },
            multisample: MultisampleState {
                count,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            layout,
            label: Some("moon_ui_shadows_pipeline".into()),
            ..default()
        }
    }
}

pub fn init_ui_shadow_pipeline(mut commands: Commands, asset_server: Res<AssetServer>) {
    let view_layout = BindGroupLayoutDescriptor::new(
        "moon_ui_shadow_view_layout",
        &BindGroupLayoutEntries::single(ShaderStages::VERTEX, uniform_buffer::<ViewUniform>(true)),
    );

    commands.insert_resource(UiShadowPipeline {
        view_layout,
        shader: load_embedded_asset!(asset_server.as_ref(), "../../shaders/shadow.wgsl"),
    });
}
