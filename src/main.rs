mod error;

use crate::error::Result;
use show_image::winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() -> Result<()> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Image view")
        .build(&event_loop)?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent { window_id, event } if window_id == window.id() => match event {
                WindowEvent::Resized(_) => {}
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => *control_flow = ControlFlow::Exit,
                WindowEvent::ScaleFactorChanged {
                    scale_factor: _,
                    new_inner_size: _,
                } => {}
                _ => {}
            },
            Event::RedrawRequested(_) => {}
            _ => {}
        }
    })
}
