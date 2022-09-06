mod window;
mod cpu;
mod utils;
use core::time::Duration;
use std::time::Instant;
use sdl2::{self, event::Event, keyboard::Keycode, pixels, rect::Rect};
use std::collections::HashSet;
const WIDTH: u32 = 64;
const HEIGHT: u32 = 32;
const SCALE: u32 = 10;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    let sdl = sdl2::init().unwrap();
    //println!("{:x?}",utils::read_file("./roms/MAZE")?);
    //return Ok(());
    let window = window::create_window(&sdl, WIDTH * SCALE, HEIGHT * SCALE).unwrap();

    let mut canvas = window.into_canvas()
        .index(window::find_sdl_gl_driver().unwrap())
        .build()
        .unwrap();
        
    let mut event_pump = sdl.event_pump().unwrap();
    let mut cpu = cpu::CPU::new();
    
    let mut prev_frame = Instant::now();

    cpu.load_program(utils::read_file("./roms/pong.rom")?);
    //cpu.reg_i = 6;
    //cpu.load_program(vec![0x54, 0xD5, 0x02, 0x12, 0x05, 0x37, 0x2D, 0xFF, 0xFF]);
    
    'running: loop {
        
        
            for event in event_pump.poll_iter() {
                match event {
                Event::Quit { .. }
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                /*Event::KeyDown { keycode: Some(keycode), repeat: true, .. } => {
                    
                }*/
                _ => {}
            }
        }
        
        let keys: HashSet<Keycode> = event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        let mut keypad: u16 = 0;
        if !keys.is_empty() {
            for keycode in keys {
            keypad |= match keycode {
                Keycode::Num1 => 1,
                Keycode::Num2 => 1 << 0x1,
                Keycode::Num3 => 1 << 0x2,
                Keycode::Num4 => 1 << 0x3,
                Keycode::Q => 1 << 0x4,
                Keycode::W => 1 << 0x5,
                Keycode::E => 1 << 0x6,
                Keycode::R => 1 << 0x7,
                Keycode::A => 1 << 0x8,
                Keycode::S => 1 << 0x9,
                Keycode::D => 1 << 0xA,
                Keycode::F => 1 << 0xB,
                Keycode::Z => 1 << 0xC,
                Keycode::X => 1 << 0xD,
                Keycode::C => 1 << 0xE,
                Keycode::V => 1 << 0xF,
                _ => 0,
            };
            }
        }
        cpu.execute_opcode(keypad);
        
        if prev_frame.elapsed() >= Duration::from_nanos(1_000_000_000u64 / 60) {
            prev_frame = Instant::now();

            if cpu.delay_timer > 0 {
                cpu.delay_timer -= 1;
            }
    
            if cpu.sound_timer > 0 {
                cpu.sound_timer -= 1;
            }

        
    }

    if cpu.vram_dirty {

        cpu.vram_dirty = false;
        canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
        canvas.clear();
    
        canvas.set_draw_color(pixels::Color::RGB(255, 255, 255));
        
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if cpu.framebuffer[(x + y * WIDTH) as usize] != 0 {
                //if y < 16 && x < 16 {
                    canvas.fill_rect(Rect::new((x * SCALE) as i32, (y * SCALE) as i32, SCALE, SCALE)).unwrap();

                }
            }
        }

        canvas.present();
    }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 240));
    }

    Ok(())
}
