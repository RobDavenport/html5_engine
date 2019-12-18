use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    use winit::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        platform::web::WindowExtWebSys,
        window::WindowBuilder,
    };

    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("Canvas")
        .with_inner_size((1600, 900).into())
        .build(&event_loop)
        .unwrap();

    let document = web_sys::window()
        .expect("Failed to obtain window")
        .document()
        .expect("Failed to obtain document");

    document
        .get_element_by_id("gameMount")
        .expect("Failed to obtain mount")
        .append_child(&window.canvas())
        .unwrap();

    event_loop.run(move |event, _, control_flow| {
        web_sys::console::log_1(&(&format!("{:?}", event)[..]).into());

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            _ => *control_flow = ControlFlow::Wait,
        }
    });
}