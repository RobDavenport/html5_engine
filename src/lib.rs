extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use web_sys::*;
use js_sys::*;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);

  #[wasm_bindgen]
  fn alert(s: &str);
}

#[wasm_bindgen]
pub struct WebGpuRenderer {
  device: GpuDevice,
  queue: GpuQueue,
  color_texture_view: GpuTextureView,
  depth_texture_view: GpuTextureView,
  pipeline: GpuRenderPipeline,
  canvas: HtmlCanvasElement,
  vertex_gpu_buffer: GpuBuffer,
  index_gpu_buffer: GpuBuffer,
  color_gpu_buffer: GpuBuffer,
  swap_chain: GpuSwapChain
}

#[wasm_bindgen]
impl WebGpuRenderer {
  #[wasm_bindgen(constructor)]
  pub fn new(in_device: JsValue, in_canvas_context: JsValue, in_canvas: JsValue) -> Self {
    log("Initializing Rust Client...");
    console_error_panic_hook::set_once();

    let device = GpuDevice::from(in_device);
    let context = GpuCanvasContext::from(in_canvas_context);
    let canvas = HtmlCanvasElement::from(in_canvas);

    let queue = device.default_queue();

    let swap_chain = context.configure_swap_chain(&GpuSwapChainDescriptor::new(
        &device,
        GpuTextureFormat::Bgra8unorm,
    ));

    let depth_texture_desc = GpuTextureDescriptor::new(
      GpuTextureFormat::Depth24plusStencil8,
      GpuExtent3dDict::new(1, canvas.height(), canvas.width()).as_ref(),
      GpuTextureUsage::OUTPUT_ATTACHMENT | GpuTextureUsage::COPY_SRC
    );

    let depth_texture = device.create_texture(&depth_texture_desc);
    let depth_texture_view = depth_texture.create_view();

    let color_texture = swap_chain.get_current_texture();
    let color_texture_view = color_texture.create_view();

    let vertices: [f32; 9] = [
      1., -1., 0.,
      -1., -1., 0.,
      0., -1., 0.,
    ];

    let colors: [f32; 9] = [
      1., 0., 1.,
      0., 1., 0.,
      0., 0., 1.,
    ];

    let indexes: [u16; 3] = [0, 1, 2];

    let uniform_data: [f32; 24] = [
      1., 0., 0., 0.,
      0., 1., 0., 0.,
      0., 0., 1., 0.,
      0., 0., 0., 1.,
      
      0.9, 0.1, 0.3, 1.,
      0.8, 0.2, 0.8, 1.,
    ];

    let vert_raw: [u32; 220] = [119734787, 65536, 524295, 30, 0, 131089, 1, 393227, 1, 1280527431, 1685353262, 808793134, 0, 196622, 0, 1, 589839, 0, 4, 1852399981, 0, 9, 11, 16, 20, 196611, 2, 450, 589828, 1096764487, 1935622738, 1918988389, 1600484449, 1684105331, 1868526181, 1667590754, 29556, 589828, 1096764487, 1935622738, 1768186216, 1818191726, 1969712737, 1600481121, 1882206772, 7037793, 262149, 4, 1852399981, 0, 327685, 9, 1131705711, 1919904879, 0, 262149, 11, 1866690153, 7499628, 393221, 14, 1348430951, 1700164197, 2019914866, 0, 393222, 14, 0, 1348430951, 1953067887, 7237481, 196613, 16, 0, 262149, 20, 1867542121, 115, 262215, 9, 30, 0, 262215, 11, 30, 1, 327752, 14, 0, 11, 0, 196679, 14, 2, 262215, 20, 30, 0, 131091, 2, 196641, 3, 2, 196630, 6, 32, 262167, 7, 6, 3, 262176, 8, 3, 7, 262203, 8, 9, 3, 262176, 10, 1, 7, 262203, 10, 11, 1, 262167, 13, 6, 4, 196638, 14, 13, 262176, 15, 3, 14, 262203, 15, 16, 3, 262165, 17, 32, 1, 262187, 17, 18, 0, 262187, 6, 19, 1048576000, 262203, 10, 20, 1, 262187, 6, 22, 1065353216, 262176, 28, 3, 13, 327734, 2, 4, 0, 3, 131320, 5, 262205, 7, 12, 11, 196670, 9, 12, 262205, 7, 21, 20, 327761, 6, 23, 21, 0, 327761, 6, 24, 21, 1, 327761, 6, 25, 21, 2, 458832, 13, 26, 23, 24, 25, 22, 327822, 13, 27, 26, 19, 327745, 28, 29, 16, 18, 196670, 29, 27, 65789, 65592];
    let vert_raw_view = unsafe { Uint32Array::view(&vert_raw) };
    let vert_spriv = Uint32Array::new_with_length(vert_raw.len() as u32);
    vert_spriv.set(vert_raw_view.as_ref(), 0);

    let frag_raw: [u32; 143] = [119734787, 65536, 524295, 19, 0, 131089, 1, 393227, 1, 1280527431, 1685353262, 808793134, 0, 196622, 0, 1, 458767, 4, 4, 1852399981, 0, 9, 12, 196624, 4, 7, 196611, 2, 450, 589828, 1096764487, 1935622738, 1918988389, 1600484449, 1684105331, 1868526181, 1667590754, 29556, 589828, 1096764487, 1935622738, 1768186216, 1818191726, 1969712737, 1600481121, 1882206772, 7037793, 262149, 4, 1852399981, 0, 393221, 9, 1182037359, 1130848626, 1919904879, 0, 262149, 12, 1866690153, 7499628, 262215, 9, 30, 0, 262215, 12, 30, 0, 131091, 2, 196641, 3, 2, 196630, 6, 32, 262167, 7, 6, 4, 262176, 8, 3, 7, 262203, 8, 9, 3, 262167, 10, 6, 3, 262176, 11, 1, 10, 262203, 11, 12, 1, 262187, 6, 14, 1065353216, 327734, 2, 4, 0, 3, 131320, 5, 262205, 10, 13, 12, 327761, 6, 15, 13, 0, 327761, 6, 16, 13, 1, 327761, 6, 17, 13, 2, 458832, 7, 18, 15, 16, 17, 14, 196670, 9, 18, 65789, 65592];
    let frag_raw_view = unsafe { Uint32Array::view(&frag_raw) };
    let frag_spriv = Uint32Array::new_with_length(frag_raw.len() as u32);
    frag_spriv.set(frag_raw_view.as_ref(), 0);

    let vertex_buffer = device.create_buffer_mapped(&GpuBufferDescriptor::new((vertices.len() * 4) as f64, GpuBufferUsage::VERTEX));
    let vertex_gpu_buffer = GpuBuffer::from(vertex_buffer.get(0));
    let vertex_arr_dst_raw = vertex_buffer.get(1);
    let vertex_array_src = unsafe { Float32Array::view(&vertices) };
    let vertex_arr_dst = Float32Array::new(&vertex_arr_dst_raw);
    vertex_arr_dst.set(vertex_array_src.as_ref(), 0);
    vertex_gpu_buffer.unmap();

    let color_buffer = device.create_buffer_mapped(&GpuBufferDescriptor::new((colors.len() * 4) as f64, GpuBufferUsage::VERTEX));
    let color_gpu_buffer = GpuBuffer::from(color_buffer.get(0));
    let color_arr_dst_raw = color_buffer.get(1);
    let color_arr_src = unsafe { Float32Array::view(&colors) };
    let color_arr_dst = Float32Array::new(&color_arr_dst_raw);
    color_arr_dst.set(color_arr_src.as_ref(), 0);
    color_gpu_buffer.unmap();

    let index_buffer = device.create_buffer_mapped(&GpuBufferDescriptor::new((indexes.len() * 2) as f64, GpuBufferUsage::INDEX));
    let index_gpu_buffer = GpuBuffer::from(index_buffer.get(0));
    let index_arr_dst_raw = index_buffer.get(1);
    let index_arr_src = unsafe { Uint16Array::view(&indexes) };
    let index_arr_dst = Uint16Array::new(&index_arr_dst_raw);
    index_arr_dst.set(index_arr_src.as_ref(), 0);
    index_gpu_buffer.unmap();

    let uniform_buffer = device.create_buffer_mapped(&GpuBufferDescriptor::new((uniform_data.len() * 4) as f64, GpuBufferUsage::UNIFORM | GpuBufferUsage::COPY_DST));
    let uniform_gpu_buffer = GpuBuffer::from(uniform_buffer.get(0));
    let uniform_arr_dst_raw = uniform_buffer.get(1);
    let uniform_arr_src = unsafe { Float32Array::view(&uniform_data) };
    let uniform_arr_dst = Float32Array::new(&uniform_arr_dst_raw);
    uniform_arr_dst.set(uniform_arr_src.as_ref(), 0);
    uniform_gpu_buffer.unmap();
    
    let vertex_module = device.create_shader_module(&GpuShaderModuleDescriptor::new(&vert_spriv));
    let fragment_module = device.create_shader_module(&GpuShaderModuleDescriptor::new(&frag_spriv));
   
    //Pipeline setup
    let uniform_bind_group_layout = device.create_bind_group_layout(
      &GpuBindGroupLayoutDescriptor::new(
        &Array::of1(
          GpuBindGroupLayoutBinding::new(
            0, 
            GpuBindingType::UniformBuffer, 
            GpuShaderStage::VERTEX
          ).as_ref()
        ).as_ref()
      )
    );
    
    let resource = Object::new();
    Reflect::set(&resource, &"buffer".into(), uniform_gpu_buffer.as_ref()).unwrap();
    let uniform_bind_group = device.create_bind_group(
      &GpuBindGroupDescriptor::new(
        Array::of1(
          GpuBindGroupBinding::new(
            0, 
            resource.as_ref()
          ).as_ref()
        ).as_ref(),
        uniform_bind_group_layout.as_ref()));

    let bind_group_layouts = Array::new();
    bind_group_layouts.push(&uniform_bind_group_layout);
    let pipeline_layout = device.create_pipeline_layout(
      &GpuPipelineLayoutDescriptor::new(&bind_group_layouts));

    //Graphics Pipeline
    let position_attrib_desc = GpuVertexAttributeDescriptor::new(GpuVertexFormat::Float3, 0., 0);
    let mut position_buffer_desc = GpuVertexBufferLayoutDescriptor::new((4 * 3) as f64, &Array::of1(position_attrib_desc.as_ref()));
    position_buffer_desc.step_mode(GpuInputStepMode::Vertex);

    let color_attrib_desc = GpuVertexAttributeDescriptor::new(GpuVertexFormat::Float3, 0., 1);
    let mut color_buffer_desc = GpuVertexBufferLayoutDescriptor::new((4 * 3) as f64, &Array::of1(color_attrib_desc.as_ref()));
    color_buffer_desc.step_mode(GpuInputStepMode::Vertex);

    let mut vertex_state_desc = GpuVertexStateDescriptor::new();
    let vertex_buffers_obj = Array::of2(position_buffer_desc.as_ref(), color_buffer_desc.as_ref());
    vertex_state_desc.index_format(GpuIndexFormat::Uint16);
    vertex_state_desc.vertex_buffers(vertex_buffers_obj.as_ref());

    let vertex_stage = GpuProgrammableStageDescriptor::new("main", &vertex_module);
    let fragment_stage = GpuProgrammableStageDescriptor::new("main", &fragment_module);

    let mut depth_stencil_state = GpuDepthStencilStateDescriptor::new(GpuTextureFormat::Depth24plusStencil8);
    depth_stencil_state.depth_write_enabled(true);
    depth_stencil_state.depth_compare(GpuCompareFunction::Less);

    let mut color_state_desc = GpuColorStateDescriptor::new(GpuTextureFormat::Bgra8unorm);
    let mut alpha_blend = GpuBlendDescriptor::new();
    alpha_blend.src_factor(GpuBlendFactor::SrcAlpha);
    alpha_blend.dst_factor(GpuBlendFactor::OneMinusSrcAlpha);
    alpha_blend.operation(GpuBlendOperation::Add);
    let mut color_blend = GpuBlendDescriptor::new();
    color_blend.src_factor(GpuBlendFactor::SrcAlpha);
    color_blend.dst_factor(GpuBlendFactor::OneMinusSrcAlpha);
    color_blend.operation(GpuBlendOperation::Add);

    color_state_desc.alpha_blend(&alpha_blend);
    color_state_desc.color_blend(&color_blend);
    color_state_desc.write_mask(GpuColorWrite::ALL);

    let mut rasterization_state_desc = GpuRasterizationStateDescriptor::new();
    rasterization_state_desc.front_face(GpuFrontFace::Cw);
    rasterization_state_desc.cull_mode(GpuCullMode::None);

    let color_states = Array::of1(color_state_desc.as_ref());
    let mut pipeline_desc = GpuRenderPipelineDescriptor::new(
      color_states.as_ref(), 
      GpuPrimitiveTopology::TriangleList, 
      &vertex_stage
    );
    pipeline_desc.layout(&pipeline_layout);
    pipeline_desc.fragment_stage(&fragment_stage);
    pipeline_desc.depth_stencil_state(&depth_stencil_state);
    pipeline_desc.vertex_state(&vertex_state_desc);
    pipeline_desc.rasterization_state(&rasterization_state_desc);

    let pipeline = device.create_render_pipeline(&pipeline_desc);

    log("webGPU successfully initialized!");

    Self {
      device,
      queue,
      color_texture_view,
      depth_texture_view,
      pipeline,
      canvas,
      vertex_gpu_buffer,
      index_gpu_buffer,
      color_gpu_buffer,
      swap_chain
    }
  }

  fn encode_commands(&self) {
    let mut color_attachment = GpuRenderPassColorAttachmentDescriptor::new(
      &self.color_texture_view,
      GpuColorDict::new(1., 0., 0., 0.,).as_ref()
    );
    color_attachment.store_op(GpuStoreOp::Store);

    let depth_attachment = GpuRenderPassDepthStencilAttachmentDescriptor::new(
      &self.depth_texture_view,
      &JsValue::from(1.),
      GpuStoreOp::Store,
      &JsValue::from(GpuLoadOp::Load),
      GpuStoreOp::Store
    );

    let mut render_pass_desc = GpuRenderPassDescriptor::new(
      Array::of1(color_attachment.as_ref()).as_ref(),
    );
    render_pass_desc.depth_stencil_attachment(&depth_attachment);

    let command_encoder = self.device.create_command_encoder();

    let pass_encoder = command_encoder.begin_render_pass(&render_pass_desc);
    pass_encoder.set_pipeline(&self.pipeline);
    pass_encoder.set_viewport(0., 0., self.canvas.width() as f32, self.canvas.height() as f32, 0., 1.);
    pass_encoder.set_scissor_rect(0, 0, self.canvas.width(), self.canvas.height());
    pass_encoder.set_vertex_buffer(0, &self.vertex_gpu_buffer);
    pass_encoder.set_vertex_buffer(1, &self.color_gpu_buffer);
    pass_encoder.set_index_buffer(&self.index_gpu_buffer);
    pass_encoder.draw_indexed(3, 1, 0, 0, 0);
    pass_encoder.end_pass();

    self.queue.submit(Array::of1(command_encoder.finish().as_ref()).as_ref());
  }

  pub fn update(&self, time: f32, width: f32, height: f32) -> Result<(), JsValue> {
    
    Ok(())
  }
  
  pub fn draw(&mut self) {
    let color_texture = self.swap_chain.get_current_texture();
    self.color_texture_view = color_texture.create_view();
    self.encode_commands();
  }
}
