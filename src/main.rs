// Current thoughts:
// Build input opcodes to use the new input handling

//Current Mod:
// cpu (input opcodes)

//Module Todo:
// Move input handling to input.rs once fully functional
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

    let texture_creator = display.canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGBA8888, CHIP8_WIDTH, CHIP8_HEIGHT)
        .expect("Failed to create texture");

    ram.load_font_set();
    cartridge.load_rom(&mut ram);

    // let mut prev_keys: HashSet<Keycode> = HashSet::new();

    'running: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        let mut keypad = [false; 16];
        let mut key_pressed = false;

        // Add all pressed keys to a HashSet
        let keys: HashSet<Keycode> = events
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        for key in keys {
            // Match each key and return the Chip 8 hex value
            let hex_key: Option<usize> = match key {
                Keycode::Num1 => Some(0x1),
                Keycode::Num2 => Some(0x2),
                Keycode::Num3 => Some(0x3),
                Keycode::Num4 => Some(0xC),
                Keycode::Q    => Some(0x4),
                Keycode::W    => Some(0x5),
                Keycode::E    => Some(0x6),
                Keycode::R    => Some(0xD),
                Keycode::A    => Some(0x7),
                Keycode::S    => Some(0x8),
                Keycode::D    => Some(0x9),
                Keycode::F    => Some(0xE),
                Keycode::Z    => Some(0xA),
                Keycode::X    => Some(0x0),
                Keycode::C    => Some(0xB),
                Keycode::V    => Some(0xF),
                _ => None,
            };

            // If valid key, set keypad[hexvalue] = true and key_pressed = true
            if let Some(i) = hex_key {
                keypad[i] = true;
                if !key_pressed {
                    key_pressed = true;
                }
            }
        }
        
        // let new_keys = &keys - &prev_keys;
        // let old_keys = &prev_keys - &keys;

        // if !new_keys.is_empty() || !old_keys.is_empty() {
        //     println!("New keys: {:?}", new_keys);
        //     println!("Old keys: {:?}\n", old_keys);
        // }

        // 60Hz loop that draws once per 60 cpu ticks
        for _hertz in 0..60 {
            cpu.tick(&mut ram, &key_pressed, &keypad);
        }

        display.draw(&cpu, &mut texture);
        // Temp sleep to display screen before panic
        thread::sleep(Duration::from_millis(100));

        // prev_keys = keys;
    }
}