//use pixels::{self, Pixels, SurfaceTexture, wgpu::Surface};

use sdl2::{
    self,
    pixels,
    event::Event, video, keyboard::Keycode, render::Texture, rect::Point,
}; //::{event::Event, Sdl, };

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;
const BOX_SIZE: u16 = 64;

fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32)
        }
    }
    None
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let window = video_subsystem.window("Window", WIDTH, HEIGHT)
        .opengl()
        .build()
        .unwrap();
    
    let mut canvas = window.into_canvas()
        .index(find_sdl_gl_driver().unwrap())
        .build()
        .unwrap();

    let mut event_pump = sdl.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                
                _ => {}
            }
        }
        canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.set_draw_color(pixels::Color::RGB(255, 255, 255));

        for y in 0..HEIGHT {
            for x in ((WIDTH / 2) as u32)..WIDTH {
                canvas.draw_point(Point::new(x as i32, y as i32));
            }
        }

        canvas.present();
    }


    Ok(())
}
