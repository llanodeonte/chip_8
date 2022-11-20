// Current thoughts:
// Work on test suite quirks

//Current Mod:
// cpu

//Module Todo:
// Incorporate timer into game loop

extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;

use std::{
    thread,
    time::Duration,
    collections::HashSet,
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
use input::Keypad;

pub const ROM_START: usize = 0x200; //0x200 = 512
pub const CHIP8_WIDTH: u32 = 64;
pub const CHIP8_HEIGHT: u32 = 32;

fn main() {
    let sdl_context = sdl2::init()
        .expect("Failed to initialize the sdl library");
    let mut events = sdl_context
        .event_pump().expect("Failed to obtain event pump");

    let mut cpu = Cpu::new();
    let mut ram = Ram::new();
    let mut cartridge = Cartridge::new();
    let mut display = Display::new(&sdl_context);
    let mut keypad = Keypad::new();

    let texture_creator = display.canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGBA8888, CHIP8_WIDTH, CHIP8_HEIGHT)
        .expect("Failed to create texture");

    ram.load_font_set();
    cartridge.load_rom(&mut ram);

    'running: loop {
        // Check for quit requests
        for event in events.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        // Update keypad with newly pressed keys
        keypad.reset_keypad();
        let pressed_keys: HashSet<Keycode> = events
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();
        keypad.update_keys(pressed_keys);
        keypad.update_keypad();
        
        // 60Hz loop that draws once per 60 cpu ticks
        for _hertz in 0..60 {
            cpu.tick(&mut ram, &mut keypad);
        }

        display.draw(&cpu, &mut texture);
        // Temp sleep to display screen before panic
        thread::sleep(Duration::from_millis(100));
    }
}