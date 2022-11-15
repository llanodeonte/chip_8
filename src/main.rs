// Current thoughts:
// Focus on Corax89's opcode test
// With IBM logo good to go, continue building opcodes until test 2 can work
// Incorporate input handling after Corax test

//Current Mod:
// cpu (opcodes)

//Module Todo:
// Incorporate timer into game loop
// Remove legacy loop once happy with 60Hz loop

extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;

use std::{
    thread,
    time,
};

mod cpu;
mod cartridge;
mod ram;
mod display;
mod input;

use cpu::Cpu;
use ram::Ram;
use cartridge::Cartridge;
use display::Display;

pub const ROM_START: usize = 0x200; //0x200 = 512
pub const CHIP8_WIDTH: u32 = 64;
pub const CHIP8_HEIGHT: u32 = 32;

fn main() {
    let sdl_context = sdl2::init()
        .expect("Failed to initialize the sdl library");
    let mut event_pump = sdl_context
        .event_pump().expect("Failed to obtain event pump");

    let mut cpu = Cpu::new();
    let mut ram = Ram::new();
    let mut cartridge = Cartridge::new();
    let mut display = Display::new(&sdl_context);

    // Instructions for toggling between pixel by pixel and texture rendering
    // Toggle 00E0, DXYN, texture.update()
    // Toggle Display struct's texture field on
    // Toggle to 4x vram with an inner array of 4 u8s per pixel for RGBA color data
    // Toggle to 4x vram write function in DXYN
    // Toggle to either RGBA renderer in draw function below

    let texture_creator = display.canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGBA8888, CHIP8_WIDTH, CHIP8_HEIGHT)
        .expect("Failed to create texture");

    ram.load_font_set();
    cartridge.load_rom(&mut ram);
    //ram.test_ram();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        // New 60Hz loop that draws once per 60 cpu ticks
        for _hertz in 0..60 {
            cpu.tick(&ram);
        }

        display.draw(&cpu, &mut texture);
        // Temp sleep to display screen before panic
        thread::sleep(time::Duration::from_millis(3000));

        // Legacy loop that draws per each vram update
        // cpu.tick(&ram);

        // if cpu.vram_update {
        //     display.draw(&cpu);
        //     thread::sleep(time::Duration::from_millis(200));
        // }
    }
}