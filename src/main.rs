mod config;
mod error;

use std::cmp::max;

use crate::error::{Result, ViewError};
use clap::Parser;
use config::InputData;
use image::{io::Reader, DynamicImage};
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::PhysicalSize,
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

const SCREEN_PERCENTAGE: f32 = 0.8;
const MIN_INNER_WINDOW_SIZE: PhysicalSize<u32> = PhysicalSize {
    width: 100,
    height: 100,
};

fn main() -> Result<()> {
    let image = get_image()?;

    let event_loop = EventLoop::new();

    let scale = get_scale(&event_loop, &image)?;

    let window_inner_size = PhysicalSize::new(
        max(image.width() / scale, MIN_INNER_WINDOW_SIZE.width),
        max(image.height() / scale, MIN_INNER_WINDOW_SIZE.height),
    );

    let window = WindowBuilder::new()
        .with_title("Image view")
        .with_inner_size(window_inner_size)
        .with_min_inner_size(MIN_INNER_WINDOW_SIZE)
        .build(&event_loop)?;

    let surface = SurfaceTexture::new(window_inner_size.width, window_inner_size.height, &window);

    let mut pixels = Pixels::new(image.width(), image.height(), surface)?;
    let pixels_bytes = pixels.get_frame();

    set_pixels_bytes(image, pixels_bytes)?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent { window_id, event } if window_id == window.id() => {
                process_window_event(event, &mut pixels, control_flow)
            }
            Event::RedrawRequested(_) => {
                let _ = pixels.render();
            }
            _ => {}
        }
    })
}

fn process_window_event(event: WindowEvent, pixels: &mut Pixels, control_flow: &mut ControlFlow) {
    match event {
        WindowEvent::Resized(size) => resize(pixels, &size),
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
        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => resize(pixels, &new_inner_size),
        _ => {}
    }
}

fn set_pixels_bytes(image: DynamicImage, pixels_bytes: &mut [u8]) -> Result<()> {
    match image {
        DynamicImage::ImageRgba8(ref p) => {
            let image_bytes = p.as_flat_samples();
            let image_bytes = image_bytes.as_slice();
            image_bytes
                .chunks_exact(4)
                .zip(pixels_bytes.chunks_exact_mut(4))
                .for_each(|(image_pixel, pixel)| {
                    pixel[0] = image_pixel[0];
                    pixel[1] = image_pixel[1];
                    pixel[2] = image_pixel[2];
                    pixel[3] = image_pixel[3];
                });
        }
        DynamicImage::ImageRgb8(ref p) => {
            let image_bytes = p.as_flat_samples();
            let image_bytes = image_bytes.as_slice();
            image_bytes
                .chunks_exact(3)
                .zip(pixels_bytes.chunks_exact_mut(4))
                .for_each(|(image_pixel, pixel)| {
                    pixel[0] = image_pixel[0];
                    pixel[1] = image_pixel[1];
                    pixel[2] = image_pixel[2];
                    pixel[3] = 0xff;
                });
        }
        _ => return Err(ViewError::UnsupportedImageType),
    }

    Ok(())
}

fn resize(pixels: &mut Pixels, new_size: &PhysicalSize<u32>) {
    pixels.resize_surface(new_size.width, new_size.height)
}

fn get_scale(event_loop: &EventLoop<()>, image: &DynamicImage) -> Result<u32> {
    let primary_monitor = event_loop
        .primary_monitor()
        .ok_or(ViewError::NoPrimaryMonitor)?;
    let screen_size = primary_monitor.size();
    let (max_width, max_height) = (
        screen_size.width as f32 * SCREEN_PERCENTAGE,
        screen_size.height as f32 * SCREEN_PERCENTAGE,
    );
    let (hor_scale, vert_scale) = (
        calc_scale(max_width as u32, image.width()),
        calc_scale(max_height as u32, image.height()),
    );

    Ok(max(hor_scale, vert_scale))
}

fn get_image() -> Result<DynamicImage> {
    let input_data = InputData::parse();
    println!("File name: {}", input_data.file_name());
    let path = std::fs::canonicalize(input_data.file_name())?;
    Reader::open(path)?.decode().map_err(|e| e.into())
}

fn calc_scale(max_size: u32, current_size: u32) -> u32 {
    if max_size > current_size {
        1
    } else {
        ((current_size as f32) / (max_size as f32)).ceil() as u32
    }
}
