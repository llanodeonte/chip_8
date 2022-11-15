//Module Todo:
// This module is in need of refactoring at some point
// Test read and write byte functions?
// Move load_font_set to display module?

// use crate::cpu::Cpu;

const RAM_SIZE: usize = 0xFFF; //0xFFF = 4096

pub struct Ram {
    pub mem: [u8; RAM_SIZE],
}

impl Ram {
    pub fn new() -> Self {
        Self {
            mem: [0; RAM_SIZE],
        }
    }

    //Potentially move this function to display module later
    pub fn load_font_set(&mut self) {
        for i in 0..80 {
            self.mem[i] = crate::display::FONT_SET[i];
        };
    }

    // // Do I need a ram read function here based on cpu.i?
    // pub fn read_byte(&self, cpu: Cpu) -> u8 {
    //     self.mem[cpu.i]
    // }

    // // Do I need a ram write function here based on cpu.i?
    // pub fn write_byte(&mut self, cpu: Cpu, byte: u8) {
    //     self.mem[cpu.i] = byte;
    // }

    // // Print the full contents of ram
    // pub fn test_ram(&self) {
    //     println!("Ram contents: {:X?}", self.mem);
    // }
}