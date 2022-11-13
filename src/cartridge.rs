//Module Todo:
// Remove ram.mem[0x1FF] = 1; once done with test rom suite
// Improve rom load so you can specify a path to load from
// Change module/struct name to Game?

use std::fs;
use crate::{ram::Ram, ROM_START};

pub struct Cartridge {
    rom: Vec<u8>,
}

impl Cartridge {
    pub fn new() -> Self {
        Self {
            rom: Vec::new(),
        }
    }

    //At some point pass the rom file path as an input variable. Currently it's static
    pub fn load_rom(&mut self, ram: &mut Ram) {
        ram.mem[0x1FF] = 1; //Manually specify a test from the test suite (1 - 5)
        self.rom = (fs::read("rom/chip8-test-suite.ch8"))
            .expect("No rom file or invalid path specified");
        for i in 0..self.rom.len() {
          ram.mem[ROM_START + i] = self.rom[i];
        }
    }
}