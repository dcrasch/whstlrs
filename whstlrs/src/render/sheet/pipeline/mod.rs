mod instance_data;

use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::time::Duration;

use resvg::usvg;
use usvg::Color;
use wgpu_jumpstart::wgpu::util::DeviceExt;
use wgpu_jumpstart::wgpu::BindGroup;
use wgpu_jumpstart::{
    wgpu, Gpu, Instances, RenderPipelineBuilder, Shape, TransformUniform, Uniform,
};

use bytemuck::{Pod, Zeroable};

use lyon::math::Point;
use lyon::path::PathEvent;
use lyon::tessellation::geometry_builder::*;
use lyon::tessellation::{self, FillOptions, FillTessellator, StrokeOptions, StrokeTessellator};
pub const FALLBACK_COLOR: usvg::Color = usvg::Color {
    red: 0,
    green: 0,
    blue: 0,
};

pub struct SheetPipeline {
    render_pipeline: wgpu::RenderPipeline,
    mesh: Mesh,
    uniform: MyUniform,
    notes: HashMap<String, Vec<usize>>,
    primitives: Vec<GpuPrimitive>,
}

pub struct Mesh {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub indices_len: u32,
}

impl Mesh {
    pub fn new(device: &wgpu::Device, mesh: &VertexBuffers<GpuVertex, u32>) -> Self {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Mesh vertices"),
            contents: bytemuck::cast_slice(&mesh.vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Mesh indices"),
            contents: bytemuck::cast_slice(&mesh.indices),
            usage: wgpu::BufferUsages::INDEX,
        });
        Self {
            vertex_buffer,
            index_buffer,
            indices_len: mesh.indices.len() as u32,
        }
    }
}

struct MyUniform {
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
    pub prims_ssbo: wgpu::Buffer,
    pub transforms_ssbo: wgpu::Buffer,
    pub globals_ubo: wgpu::Buffer,
}

impl MyUniform {
    pub fn new(
        device: &wgpu::Device,
        primitives: &[GpuPrimitive],
        transforms: &[GpuTransform],
    ) -> Self {
        let prim_buffer_byte_size = (primitives.len() * std::mem::size_of::<GpuPrimitive>()) as u64;
        let transform_buffer_byte_size =
            (transforms.len() * std::mem::size_of::<GpuTransform>()) as u64;
        let globals_buffer_byte_size = std::mem::size_of::<GpuGlobals>() as u64;

        let prims_ssbo = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Prims ssbo"),
            size: prim_buffer_byte_size,
            usage: wgpu::BufferUsages::VERTEX
                | wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let transforms_ssbo = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Transforms ssbo"),
            size: transform_buffer_byte_size,
            usage: wgpu::BufferUsages::VERTEX
                | wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let globals_ubo = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Globals ubo"),
            size: globals_buffer_byte_size,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Bind group layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(globals_buffer_byte_size),
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(prim_buffer_byte_size),
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(transform_buffer_byte_size),
                    },
                    count: None,
                },
            ],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Bind group"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(globals_ubo.as_entire_buffer_binding()),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Buffer(prims_ssbo.as_entire_buffer_binding()),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Buffer(
                        transforms_ssbo.as_entire_buffer_binding(),
                    ),
                },
            ],
        });

        Self {
            bind_group,
            bind_group_layout,
            globals_ubo,
            prims_ssbo,
            transforms_ssbo,
        }
    }
}

fn collect_paths(parent: &usvg::Group, paths: &mut Vec<(usvg::Path, String)>, id_attr: &str) {
    for node in parent.children() {
        if let usvg::Node::Group(ref group) = node {
            let id_attr = if group.id().is_empty() {
                id_attr
            } else {
                group.id()
            };
            collect_paths(group, paths, id_attr);
        } else if let usvg::Node::Path(ref p) = node {
            paths.push((*p.to_owned(), id_attr.to_string()));
        }
    }
}

impl<'a> SheetPipeline {
    pub fn new(gpu: &Gpu, transform_uniform: &Uniform<TransformUniform>) -> Self {
        // SVG

        let mut fill_tess = FillTessellator::new();
        let mut stroke_tess = StrokeTessellator::new();
        let mut mesh: VertexBuffers<_, u32> = VertexBuffers::new();
        let mut notes: HashMap<String, Vec<usize>> = HashMap::new();
        let fontdb = usvg::fontdb::Database::new();
        let opt = usvg::Options::default();
        //let file_data = std::fs::read(filename).unwrap();
        let file_data =
            include_bytes!("../../../../../contrib/starofthecountydown/starofthecountydown.svg");
        let rtree = usvg::Tree::from_data(file_data, &opt, &fontdb).unwrap();
        let mut transforms = Vec::new();
        let mut primitives = Vec::new();

        let mut prev_transform = usvg::Transform {
            sx: f32::NAN,
            kx: f32::NAN,
            ky: f32::NAN,
            sy: f32::NAN,
            tx: f32::NAN,
            ty: f32::NAN,
        };
        let view_box = rtree.view_box();
        let mut paths: Vec<(usvg::Path, String)> = Vec::new();
        collect_paths(rtree.root(), &mut paths, "");
        for (p, id_attr) in paths {
            let t = p.abs_transform();
            if t != prev_transform {
                transforms.push(GpuTransform {
                    data0: [t.sx, t.kx, t.ky, t.sy],
                    data1: [t.tx, t.ty, 0.0, 0.0],
                });
            }
            prev_transform = t;

            let transform_idx = transforms.len() as u32 - 1;

            if let Some(ref fill) = p.fill() {
                // fall back to always use color fill
                // no gradients (yet?)
                let color: usvg::Color = match fill.paint() {
                    usvg::Paint::Color(c) => c.clone(),
                    _ => FALLBACK_COLOR,
                };

                primitives.push(GpuPrimitive::new(
                    transform_idx,
                    color,
                    fill.opacity().get(),
                ));

                if !id_attr.is_empty() {
                    let prim_id = primitives.len() - 1;
                    (*notes.entry(id_attr).or_default()).push(prim_id);
                }

                fill_tess
                    .tessellate(
                        convert_path(&p),
                        &FillOptions::tolerance(0.01),
                        &mut BuffersBuilder::new(
                            &mut mesh,
                            VertexCtor {
                                prim_id: primitives.len() as u32 - 1,
                            },
                        ),
                    )
                    .expect("Error during tessellation!");
            }

            if let Some(ref stroke) = p.stroke() {
                let (stroke_color, stroke_opts) = convert_stroke(stroke);
                primitives.push(GpuPrimitive::new(
                    transform_idx,
                    stroke_color,
                    stroke.opacity().get(),
                ));
                let _ = stroke_tess.tessellate(
                    convert_path(&p),
                    &stroke_opts.with_tolerance(0.01),
                    &mut BuffersBuilder::new(
                        &mut mesh,
                        VertexCtor {
                            prim_id: primitives.len() as u32 - 1,
                        },
                    ),
                );
            }
        }

        let myuniform = MyUniform::new(&gpu.device, &primitives, &transforms);

        let shader = &gpu
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("RectanglePipeline::shader"),
                source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!(
                    "./shader.wgsl"
                ))),
            });

        let pipeline_layout = &gpu
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                bind_group_layouts: &[&myuniform.bind_group_layout],
                push_constant_ranges: &[],
                label: Some("pipeline layout"),
            });

        let target = //wgpu_jumpstart::default_color_target_state(gpu.texture_format);
	wgpu::ColorTargetState {
                    format: gpu.texture_format,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                };
        let render_pipeline = wgpu::RenderPipelineDescriptor::builder(
            pipeline_layout,
            wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<GpuVertex>() as u64,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[
                        wgpu::VertexAttribute {
                            offset: 0,
                            format: wgpu::VertexFormat::Float32x2,
                            shader_location: 0,
                        },
                        wgpu::VertexAttribute {
                            offset: 8,
                            format: wgpu::VertexFormat::Uint32,
                            shader_location: 1,
                        },
                    ],
                }],
            },
        )
        .fragment("fs_main", &shader, &[Some(target)])
        .create_render_pipeline(&gpu.device);

        let mesh = Mesh::new(&gpu.device, &mesh);
        let _ = &gpu.queue.write_buffer(
            &myuniform.transforms_ssbo,
            0,
            bytemuck::cast_slice(&transforms),
        );
        let _ =
            &gpu.queue
                .write_buffer(&myuniform.prims_ssbo, 0, bytemuck::cast_slice(&primitives));

        const WINDOW_SIZE: f32 = 800.0;
        let vb_width = view_box.rect.size().width() as f32;
        let vb_height = view_box.rect.size().height() as f32;
        let scale = vb_width / vb_height;

        let (width, height) = if scale < 1.0 {
            (WINDOW_SIZE, WINDOW_SIZE * scale)
        } else {
            (WINDOW_SIZE, WINDOW_SIZE / scale)
        };

        let pan = [vb_width / -2.0, vb_height / -2.0];
        let zoom = 2.0 / f32::max(vb_width, vb_height);

        let _ = &gpu.queue.write_buffer(
            &myuniform.globals_ubo,
            0,
            bytemuck::cast_slice(&[GpuGlobals {
                aspect_ratio: width as f32 / height as f32,
                zoom: [zoom, zoom],
                pan: pan,
                _pad: 0.0,
            }]),
        );

        Self {
            render_pipeline,
            mesh,
            uniform: myuniform,
            notes,
            primitives,
        }
    }

    pub fn update_time(&mut self, gpu: &mut Gpu, delta: Duration) {
        let d = delta.as_secs_f32();
        let mut notes = self
            .notes
            .iter()
            .map(|x| {
                let idx: Vec<u32> = x.0.split('-').flat_map(str::parse).collect();

                (idx[0], idx[1], x.1.clone())
            })
            .collect::<Vec<(u32, u32, Vec<usize>)>>();
        notes.sort();
        let notes_count = notes.len();
        let idx = d as usize % notes_count;
        let note = &notes[idx];
        let prim_id = note.2[0];
        let mut prims = self.primitives.clone();
        let color: usvg::Color = Color::new_rgb(255, 0, 0);

        prims[prim_id] = GpuPrimitive::new(prims[prim_id].transform, color, 0.0);
        let _ = &gpu
            .queue
            .write_buffer(&self.uniform.prims_ssbo, 0, bytemuck::cast_slice(&prims));
    }

    pub fn render(
        &'a self,
        transform_uniform: &'a Uniform<TransformUniform>,
        render_pass: &mut wgpu::RenderPass<'a>,
    ) {
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.uniform.bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.mesh.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);

        render_pass.draw_indexed(0..(self.mesh.indices_len as u32), 0, 0..1);
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GpuVertex {
    pub position: [f32; 2],
    pub prim_id: u32,
}

// A 2x3 matrix (last two members of data1 unused).
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GpuTransform {
    pub data0: [f32; 4],
    pub data1: [f32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GpuPrimitive {
    pub transform: u32,
    pub color: u32,
    pub _pad: [u32; 2],
}

impl GpuPrimitive {
    pub fn new(transform_idx: u32, color: usvg::Color, alpha: f32) -> Self {
        GpuPrimitive {
            transform: transform_idx,
            color: ((color.red as u32) << 24)
                + ((color.green as u32) << 16)
                + ((color.blue as u32) << 8)
                + (alpha * 255.0) as u32,
            _pad: [0; 2],
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GpuGlobals {
    pub zoom: [f32; 2],
    pub pan: [f32; 2],
    pub aspect_ratio: f32,
    pub _pad: f32,
}

pub struct VertexCtor {
    pub prim_id: u32,
}

impl FillVertexConstructor<GpuVertex> for VertexCtor {
    fn new_vertex(&mut self, vertex: tessellation::FillVertex) -> GpuVertex {
        GpuVertex {
            position: vertex.position().to_array(),
            prim_id: self.prim_id,
        }
    }
}

impl StrokeVertexConstructor<GpuVertex> for VertexCtor {
    fn new_vertex(&mut self, vertex: tessellation::StrokeVertex) -> GpuVertex {
        GpuVertex {
            position: vertex.position().to_array(),
            prim_id: self.prim_id,
        }
    }
}
/// Some glue between usvg's iterators and lyon's.

pub struct PathConvIter<'a> {
    iter: usvg::tiny_skia_path::PathSegmentsIter<'a>,
    //std::slice::Iter<'a, usvg::tiny_skia_path::PathSegment>,
    prev: Point,
    first: Point,
    needs_end: bool,
    deferred: Option<PathEvent>,
}

impl<'l> Iterator for PathConvIter<'l> {
    type Item = PathEvent;
    fn next(&mut self) -> Option<PathEvent> {
        if self.deferred.is_some() {
            return self.deferred.take();
        }

        let next = self.iter.next();
        match next {
            Some(usvg::tiny_skia_path::PathSegment::MoveTo(p)) => {
                if self.needs_end {
                    let last = self.prev;
                    let first = self.first;
                    self.needs_end = false;
                    self.prev = Point::new(p.x, p.y);
                    self.deferred = Some(PathEvent::Begin { at: self.prev });
                    self.first = self.prev;
                    Some(PathEvent::End {
                        last,
                        first,
                        close: false,
                    })
                } else {
                    self.first = Point::new(p.x, p.y);
                    self.needs_end = true;
                    Some(PathEvent::Begin { at: self.first })
                }
            }
            Some(usvg::tiny_skia_path::PathSegment::LineTo(p)) => {
                self.needs_end = true;
                let from = self.prev;
                self.prev = Point::new(p.x, p.y);
                Some(PathEvent::Line {
                    from,
                    to: self.prev,
                })
            }
            Some(usvg::tiny_skia_path::PathSegment::QuadTo(p1, p2)) => {
                // https://www.w3.org/TR/SVG/paths.html#PathDataQuadraticBezierCommands
                self.needs_end = true;
                let from = self.prev;
                self.prev = Point::new(p2.x, p2.y);
                let ctrl = Point::new(p1.x, p1.y);
                Some(PathEvent::Quadratic {
                    from,
                    ctrl,
                    to: self.prev,
                })
            }
            Some(usvg::tiny_skia_path::PathSegment::CubicTo(p1, p2, p3)) => {
                // https://www.w3.org/TR/SVG/paths.html#PathDataCubicBezierCommands
                self.needs_end = true;
                let from = self.prev;
                self.prev = Point::new(p3.x, p3.y);
                let ctrl1 = Point::new(p1.x, p1.y);
                let ctrl2 = Point::new(p2.x, p2.y);
                Some(PathEvent::Cubic {
                    from,
                    ctrl1,
                    ctrl2,
                    to: self.prev,
                })
            }
            Some(usvg::tiny_skia_path::PathSegment::Close) => {
                self.needs_end = false;
                self.prev = self.first;
                Some(PathEvent::End {
                    last: self.prev,
                    first: self.first,
                    close: true,
                })
            }
            None => {
                if self.needs_end {
                    self.needs_end = false;
                    let last = self.prev;
                    let first = self.first;
                    Some(PathEvent::End {
                        last,
                        first,
                        close: false,
                    })
                } else {
                    None
                }
            }
        }
    }
}

pub fn convert_path(p: &usvg::Path) -> PathConvIter {
    PathConvIter {
        iter: p.data().segments(),
        first: Point::new(0.0, 0.0),
        prev: Point::new(0.0, 0.0),
        deferred: None,
        needs_end: false,
    }
}

pub fn convert_stroke(s: &usvg::Stroke) -> (usvg::Color, StrokeOptions) {
    let color = match s.paint() {
        usvg::Paint::Color(c) => c.clone(),
        _ => FALLBACK_COLOR,
    };
    let linecap = match s.linecap() {
        usvg::LineCap::Butt => tessellation::LineCap::Butt,
        usvg::LineCap::Square => tessellation::LineCap::Square,
        usvg::LineCap::Round => tessellation::LineCap::Round,
    };
    let linejoin = match s.linejoin() {
        usvg::LineJoin::Miter => tessellation::LineJoin::Miter,
        usvg::LineJoin::Bevel => tessellation::LineJoin::Bevel,
        usvg::LineJoin::Round => tessellation::LineJoin::Round,
        usvg::LineJoin::MiterClip => tessellation::LineJoin::MiterClip,
    };

    let opt = StrokeOptions::tolerance(0.01)
        .with_line_width(s.width().get() as f32)
        .with_line_cap(linecap)
        .with_line_join(linejoin);

    (color, opt)
}

unsafe impl bytemuck::Pod for GpuGlobals {}
unsafe impl bytemuck::Zeroable for GpuGlobals {}
unsafe impl bytemuck::Pod for GpuVertex {}
unsafe impl bytemuck::Zeroable for GpuVertex {}
unsafe impl bytemuck::Pod for GpuPrimitive {}
unsafe impl bytemuck::Zeroable for GpuPrimitive {}
unsafe impl bytemuck::Pod for GpuTransform {}
unsafe impl bytemuck::Zeroable for GpuTransform {}
