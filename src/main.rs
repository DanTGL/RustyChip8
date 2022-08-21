mod window;
mod cpu;
mod framebuffer;

const CHIP8_WIDTH: u8 = 64;
const CHIP8_HEIGHT: u8 = 32;

fn main() {
    println!("Hello, world!");
    window::run();
}
