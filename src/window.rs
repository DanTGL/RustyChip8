//use pixels::{self, Pixels, SurfaceTexture, wgpu::Surface};

use sdl2::{
    self,
    pixels::{self, PixelFormatEnum},
    event::Event, Sdl, video::Window, keyboard::Keycode, render::Texture, rect::Point, rect::Rect,
}; //::{event::Event, Sdl, };

//const WIDTH: u32 = 640;
//const HEIGHT: u32 = 320;
//const BOX_SIZE: u32 = 10;

pub fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32)
        }
    }
    None
}

pub fn create_window(sdl: &Sdl, width: u32, height: u32) -> Result<Window, Box<dyn std::error::Error>> {
    let video_subsystem = sdl.video().unwrap();
    let window = video_subsystem.window("Window", width, height)
        .opengl()
        .build()
        .unwrap();


    Ok(window)
}
