extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
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
  device: web_sys::GpuDevice
}

#[wasm_bindgen]
impl WebGpuRenderer {
  #[wasm_bindgen(constructor)]
  pub fn new(in_device: JsValue, in_canvas_context: JsValue) -> Self {
    log("Initializing Rust Client...");
    console_error_panic_hook::set_once();

    let device = web_sys::GpuDevice::from(in_device);
    let context = web_sys::GpuCanvasContext::from(in_canvas_context);

    let queue = device.default_queue();

    let swap_chain = context.configure_swap_chain(&GpuSwapChainDescriptor::new(
        &device,
        GpuTextureFormat::Bgra8unorm,
    ));

    let mut depth_texture_desc = GpuDepthStencilStateDescriptor::new(GpuTextureFormat::Depth24plusStencil8);
    depth_texture_desc.depth_write_enabled(true);
    depth_texture_desc.depth_compare(GpuCompareFunction::Less);

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

    let create_vertex_buffer_result = device.create_buffer_mapped(&GpuBufferDescriptor::new(vertices.len() as f64, web_sys::GpuBufferUsage::VERTEX));
    let vertex_array_buffer = js_sys::ArrayBuffer::from(create_vertex_buffer_result.get(1));
    let vertex_array_view = unsafe { Float32Array::view(&vertices) };
    let typed_vertex_array = Float32Array::new_with_length(vertex_array_buffer.byte_length());
    typed_vertex_array.set(vertex_array_view.as_ref(), 0);

    let create_color_buffer_result = device.create_buffer_mapped(&GpuBufferDescriptor::new(colors.len() as f64, web_sys::GpuBufferUsage::VERTEX));
    let color_array_buffer = js_sys::ArrayBuffer::from(create_color_buffer_result.get(1));
    let color_array_view = unsafe { Float32Array::view(&colors) };
    let typed_color_array = Float32Array::new_with_length(color_array_buffer.byte_length());
    typed_color_array.set(color_array_view.as_ref(), 0);

    let create_index_buffer_result = device.create_buffer_mapped(&GpuBufferDescriptor::new(indexes.len() as f64, web_sys::GpuBufferUsage::INDEX));
    let index_array_buffer = js_sys::ArrayBuffer::from(create_index_buffer_result.get(1));
    let index_array_view = unsafe { Uint16Array::view(&indexes) };
    let typed_index_array = Uint16Array::new_with_length(index_array_buffer.byte_length());
    typed_color_array.set(color_array_view.as_ref(), 0);

    let create_uniform_buffer_result = device.create_buffer_mapped(&GpuBufferDescriptor::new(uniform_data.len() as f64, web_sys::GpuBufferUsage::UNIFORM | web_sys::GpuBufferUsage::COPY_DST));
    let uniform_buffer = web_sys::GpuBuffer::from(create_uniform_buffer_result.get(0));
    let uniform_array_buffer = js_sys::ArrayBuffer::from(create_uniform_buffer_result.get(1));
    let uniform_array_view = unsafe { Float32Array::view(&uniform_data) };
    let typed_uniform_array = Float32Array::new_with_length(uniform_array_buffer.byte_length());
    typed_uniform_array.set(uniform_array_view.as_ref(), 0);
    
    let vertex_module = device.create_shader_module(&web_sys::GpuShaderModuleDescriptor::new(&vert_spriv));
    let fragment_module = device.create_shader_module(&web_sys::GpuShaderModuleDescriptor::new(&frag_spriv));

   
    let bindings_array_layout = js_sys::Array::new();
    let binding_layout = web_sys::GpuBindGroupLayoutBinding::new(0, web_sys::GpuBindingType::UniformBuffer, web_sys::GpuShaderStage::VERTEX);
    bindings_array_layout.push(binding_layout.as_ref());
    let uniform_bind_group_layout = device.create_bind_group_layout(
      &web_sys::GpuBindGroupLayoutDescriptor::new(&bindings_array_layout));
      
    let bindings_array_group = js_sys::Array::new();
    let resource = js_sys::Object::new();
    js_sys::Reflect::set(&resource, &"buffer".into(), uniform_buffer.as_ref()).unwrap();
    let binding_group = web_sys::GpuBindGroupBinding::new(0, &resource);
    bindings_array_group.push(binding_group.as_ref()); 
    let uniform_bind_group = device.create_bind_group(
      &web_sys::GpuBindGroupDescriptor::new(&bindings_array_group, &uniform_bind_group_layout));

    let bind_group_layouts = js_sys::Array::new();
    bind_group_layouts.push(&uniform_bind_group_layout);
    let pipeline_layout = device.create_pipeline_layout(
      &web_sys::GpuPipelineLayoutDescriptor::new(&bind_group_layouts));

      //Continue Graphics Pipeline...

    log("webGPU successfully initialized!");

    Self {
      device 
    }
  }

  pub fn update(&self, time: f32, width: f32, height: f32) -> Result<(), JsValue> {
    
    Ok(())
  }
  
  #[wasm_bindgen]
  pub fn draw(&self) {
    // let color_texture = swapchain.get_current_texture();
    // let color_texture_view = color_texture.create_view();
      
  }
}
