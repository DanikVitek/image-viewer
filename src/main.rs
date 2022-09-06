mod config;
mod error;

use std::cmp::max;

use crate::error::ViewError;
use clap::Parser;
use config::InputData;
use image::{io::Reader, DynamicImage};
use show_image::{
    create_window,
    event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent, WindowKeyboardInputEvent},
    ImageInfo, ImageView, WindowOptions,
};

const MIN_SIZE: [u32; 2] = [300; 2];

#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let image = get_image()?;

    let image_view = get_image_view(&image)?;

    let window = create_window(
        "Image viewer",
        WindowOptions::new().set_size(Some([
            max(MIN_SIZE[0], image.width()),
            max(MIN_SIZE[1], image.height()),
        ])),
    )?;
    window.set_image("img", image_view)?;

    let mut is_fullscreen = false;
    let mut ignore_fs_input = false;

    for event in window.event_channel()? {
        match event {
            WindowEvent::KeyboardInput(WindowKeyboardInputEvent {
                input:
                    KeyboardInput {
                        key_code: Some(key_code),
                        state,
                        ..
                    },
                ..
            }) => match key_code {
                VirtualKeyCode::Escape => break,
                VirtualKeyCode::F11 if state == ElementState::Pressed && !ignore_fs_input => {
                    is_fullscreen = !is_fullscreen;
                    ignore_fs_input = true;

                    window.run_function(move |mut ctx| {
                        ctx.set_borderless(is_fullscreen);
                        ctx.set_fullscreen(is_fullscreen);
                    });
                }
                VirtualKeyCode::F11 if state == ElementState::Released && ignore_fs_input => {
                    ignore_fs_input = false;
                }
                _ => {}
            },
            _ => {}
        }
    }

    Ok(())
}

fn get_image_view<'img>(image: &'img DynamicImage) -> crate::error::Result<ImageView<'img>> {
    let image_view = match image {
        DynamicImage::ImageRgba8(p) => {
            ImageView::new(ImageInfo::rgba8(image.width(), image.height()), p)
        }
        DynamicImage::ImageRgb8(p) => {
            ImageView::new(ImageInfo::rgb8(image.width(), image.height()), p)
        }
        _ => return Err(ViewError::UnsupportedImageType),
    };

    Ok(image_view)
}

fn get_image() -> crate::error::Result<DynamicImage> {
    let input_data = InputData::parse();
    let path = std::fs::canonicalize(input_data.file_name())?;
    Reader::open(path)?.decode().map_err(|e| e.into())
}
