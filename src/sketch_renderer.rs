use std::{num::NonZeroU64, sync::Arc};

use eframe::{egui, egui_wgpu, CreationContext};
use wgpu::util::DeviceExt;

//mostly adapted from https://gist.github.com/zicklag/b9c1be31ec599fd940379cecafa1751b
//Needs to be adapted to https://github.com/emilk/egui/blob/master/crates/egui_demo_app/src/apps/custom3d_wgpu.rs
pub struct SketchRenderer{
    angle: f32
}

impl SketchRenderer{
    pub fn new(cc: & CreationContext) -> Self{
        let render_state = cc.wgpu_render_state.as_ref().expect("WGPU should be enabled");
        let device = &render_state.device;

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(include_str!("./triangle_shader.wgsl").into()),
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: NonZeroU64::new(16),
                },
                count: None,
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
                compilation_options: Default::default()
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                compilation_options: Default::default(),
                targets: &[Some(render_state.target_format.into())],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None
        });

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("custom3d"),
            contents: bytemuck::cast_slice(&[0.0f32; 4]), //16 bytes aligned
            usage: wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::UNIFORM,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        // Because the graphics pipeline must have the same lifetime as the egui render pass,
        // instead of storing the pipeline in our `Custom3D` struct, we insert it into the
        // `paint_callback_resources` type map, which is stored alongside the render pass.
        render_state
            .renderer.write().callback_resources
            .insert(TriangleRenderResources {
                pipeline,
                bind_group,
                uniform_buffer,
            });

        Self { angle: 0.0 }
    }

    pub fn custom_painting(&mut self, ui: &mut egui::Ui) {
        let (rect, response) =
            ui.allocate_exact_size(egui::Vec2::splat(300.0), egui::Sense::drag());

        self.angle += response.drag_delta().x * 0.01;

        // The callback function for WGPU is in two stages: prepare, and paint.
        //
        // The prepare callback is called every frame before paint and is given access to the wgpu
        // Device and Queue, which can be used, for instance, to update buffers and uniforms before
        // rendering.
        //
        // The paint callback is called after prepare and is given access to the render pass, which
        // can be used to issue draw commands.
        struct TrianglePaintCallback{
            angle: f32
        }

        impl egui_wgpu::CallbackTrait for TrianglePaintCallback{
            fn prepare(
                    &self,
                    device: &wgpu::Device,
                    queue: &wgpu::Queue,
                    _screen_descriptor: &egui_wgpu::ScreenDescriptor,
                    _egui_encoder: &mut wgpu::CommandEncoder,
                    callback_resources: &mut egui_wgpu::CallbackResources,
                ) -> Vec<wgpu::CommandBuffer> {
                    let resources: &TriangleRenderResources = callback_resources.get().unwrap();
                    resources.prepare(device, queue, self.angle);
                    Default::default()
            }

            fn paint(
                &self,
                info: egui::PaintCallbackInfo,
                render_pass: &mut wgpu::RenderPass<'static>,
                callback_resources: &egui_wgpu::CallbackResources,
            ) {
                let resources: &TriangleRenderResources = callback_resources.get().as_ref().unwrap();
                resources.paint(render_pass);
            }
        }
        let cb =  TrianglePaintCallback{angle: self.angle};

        let callback = egui::PaintCallback {
            rect,
            callback: Arc::new(cb),
        };

        ui.painter().add(callback);
    }
}

struct TriangleRenderResources {
    pipeline: wgpu::RenderPipeline,
    bind_group: wgpu::BindGroup,
    uniform_buffer: wgpu::Buffer,
}

impl TriangleRenderResources {
    fn prepare(&self, _device: &wgpu::Device, queue: &wgpu::Queue, angle: f32) {
        // Update our uniform buffer with the angle from the UI
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[angle]));
    }

    fn paint<'rpass, 'encoder>(&'rpass self, rpass: &mut wgpu::RenderPass<'encoder>) {
        // Draw our triangle!
        rpass.set_pipeline(&self.pipeline);
        rpass.set_bind_group(0, &self.bind_group, &[]);
        rpass.draw(0..3, 0..1);
    }
}