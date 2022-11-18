//Module Todo:
// Continue implementing opcodes and match statements
// Build timer functions

use crate::{
    ram::Ram, 
    ROM_START,
};

const OPCODE_INTERVAL: usize = 2;

#[derive(Clone, Copy)]
enum ProgramCounter {
    Next,
    Prev,
    Jump(usize),
}

pub struct Cpu {
    pc: usize,
    sp: usize, //May not need due to .push() and .pop()
    i: usize,
    v: [u8; 16],
    stack: [u16; 16], //Keep stack an array for now. Use vector if issues arise.
    pub vram: [u8; 32 * 64 * 4], // RGBA VRAM (Height: 32, Width: 64, RGBA: 4)
    dt: u8, // Delay Timer
    // st: u8, //Todo: Implement Sound Timer
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            pc: ROM_START,
            sp: 0,
            i: 0,
            v: [0; 16],
            stack: [0; 16],
            vram: [0; 8192], // RGBA VRAM
            dt: 0,
            // st: 0,
        }
    }

    fn debug(&self, current_opcode: &u16) {
        println!("Current Opcode: {:X?}", current_opcode);
        println!("Next PC: {:X?}, SP: {:X?}, I: {:X?}", self.pc, self.sp, self.i);
        println!("V registers: {:X?}", self.v);
        println!("Stack: {:X?}\n", self.stack);
    }

    fn set_pc(&mut self, addr: ProgramCounter) {
        self.pc = match addr {
            ProgramCounter::Next => self.pc + OPCODE_INTERVAL,
            ProgramCounter::Prev => self.pc - OPCODE_INTERVAL,
            ProgramCounter::Jump(addr) => addr,
        };
    }

    fn set_i(&mut self, addr: usize) {
        self.i = addr;
    }

    fn read_v(&self, addr: usize) -> u8 {
        self.v[addr]
    }

    fn write_v(&mut self, addr: usize, data: u8) {
        self.v[addr] = data;
    }

    pub fn tick(&mut self, ram: &mut Ram, key_pressed: &bool, keypad: &[bool; 16]) {
        let current_opcode = self.fetch_opcode(ram);
        self.execute_opcode(ram, key_pressed, keypad, &current_opcode);
        if self.dt > 0 {
            self.dt -= 1;
        }
    }

    pub fn fetch_opcode(&mut self, ram: &Ram) -> u16 {
        let opcode = (ram.mem[self.pc] as u16) << 8 | (ram.mem[self.pc + 1] as u16);
        self.set_pc(ProgramCounter::Next);
        opcode
    }

    pub fn execute_opcode(&mut self, ram: &mut Ram, key_pressed: &bool, keypad: &[bool; 16], current_opcode: &u16) {
        //Represent the nibbles of the current instruction as a series of tuple values 
        let opcode_nibbles = (
            //Use bitwise and to zero out everything other than the focus nibble
            (current_opcode & 0xF000) >> 12 as u8,
            (current_opcode & 0x0F00) >> 8 as u8,
            (current_opcode & 0x00F0) >> 4 as u8,
            (current_opcode & 0x000F) as u8,
        );

        let nnn = (current_opcode & 0x0FFF) as usize; //12 bit address for ram
        let kk = (current_opcode & 0x00FF) as u8; //8 bit data
        let x = opcode_nibbles.1 as usize; //4 bit address for v register x
        let y = opcode_nibbles.2 as usize; //4 bit address for v register y
        let n = opcode_nibbles.3 as usize; //4 bit address for the byte range of a sprite in ram

        match opcode_nibbles {
            //Match to an instruction based on the nibbles tuple values
            (0x00, 0x00, 0x0E, 0x00) => self.opcode_00e0(),
            (0x00, 0x00, 0x0E, 0x0E) => self.opcode_00ee(),
            (0x01,    _,    _,    _) => self.opcode_1nnn(nnn),
            (0x02,    _,    _,    _) => self.opcode_2nnn(nnn),
            (0x03,    _,    _,    _) => self.opcode_3xkk(x, kk),
            (0x04,    _,    _,    _) => self.opcode_4xkk(x, kk),
            (0x05,    _,    _, 0x00) => self.opcode_5xy0(x, y),
            (0x06,    _,    _,    _) => self.opcode_6xkk(x, kk),
            (0x07,    _,    _,    _) => self.opcode_7xkk(x, kk),
            (0x08,    _,    _, 0x00) => self.opcode_8xy0(x, y),
            (0x08,    _,    _, 0x01) => self.opcode_8xy1(x, y),
            (0x08,    _,    _, 0x02) => self.opcode_8xy2(x, y),
            (0x08,    _,    _, 0x03) => self.opcode_8xy3(x, y),
            (0x08,    _,    _, 0x04) => self.opcode_8xy4(x, y),
            (0x08,    _,    _, 0x05) => self.opcode_8xy5(x, y),
            (0x08,    _,    _, 0x06) => self.opcode_8xy6(x),
            (0x08,    _,    _, 0x0E) => self.opcode_8xye(x),
            (0x09,    _,    _, 0x00) => self.opcode_9xy0(x, y),
            (0x0A,    _,    _,    _) => self.opcode_annn(nnn),
            (0x0D,    _,    _,    _) => self.opcode_dxyn(ram, x, y, n),
            (0x0E,    _, 0x09, 0x0E) => self.opcode_ex9e(key_pressed, keypad, x),
            (0x0E,    _, 0x0A, 0x01) => self.opcode_exa1(key_pressed, keypad, x),
            (0x0F,    _, 0x00, 0x07) => self.opcode_fx07(x),
            (0x0F,    _, 0x00, 0x0A) => self.opcode_fx0a(key_pressed, keypad, x),
            (0x0F,    _, 0x01, 0x05) => self.opcode_fx15(x),
            (0x0F,    _, 0x01, 0x0E) => self.opcode_fx1e(ram, x),
            (0x0F,    _, 0x03, 0x03) => self.opcode_fx33(ram, x),
            (0x0F,    _, 0x05, 0x05) => self.opcode_fx55(ram, x),
            (0x0F,    _, 0x06, 0x05) => self.opcode_fx65(ram, x),
            _ => panic!("Unknown opcode {:X?} at PC {:X?}", current_opcode, self.pc),
        };

        self.debug(&current_opcode);
    }

    //All Chip 8 opcodes are defined below as functions

    // Clear vram
    fn opcode_00e0(&mut self) {
        for height in 0..32 {
            for width in 0..64 {
                for rgba in 0..4 {
                    // RGBA VRAM Clear
                    self.vram[(height * 64 * 4) + (width * 4) + rgba] = 0;
                }
            }
        }
    }

    // Return from a subroutine
    fn opcode_00ee(&mut self) {
        if self.sp > 0 {
            self.sp -= 1;
        }
        self.pc = self.stack[self.sp] as usize;
        self.stack[self.sp] = 0;
    }

    // Jump to address nnn
    fn opcode_1nnn(&mut self, nnn: usize) {
        self.set_pc(ProgramCounter::Jump(nnn));
    }

    // Call subroutine at nnn
    fn opcode_2nnn(&mut self, nnn: usize) {
        self.stack[self.sp] = self.pc as u16;
        self.sp += 1;
        self.pc = nnn;
    }

    // If vx = kk, skip the next opcode
    fn opcode_3xkk (&mut self, x: usize, kk: u8) {
        if self.read_v(x) == kk {
            self.set_pc(ProgramCounter::Next);
        }
    }

    // If vx != kk, skip the next opcode
    fn opcode_4xkk (&mut self, x: usize, kk: u8) {
        if self.read_v(x) != kk {
            self.set_pc(ProgramCounter::Next);
        }
    }

    // If vx = vy, skip the next opcode
    fn opcode_5xy0 (&mut self, x: usize, y: usize) {
        if self.read_v(x) == self.read_v(y) {
            self.set_pc(ProgramCounter::Next);
        }
    }

    // Set vx = kk
    fn opcode_6xkk(&mut self, x: usize, kk: u8) {
        self.write_v(x, kk);
    }

    // Set vx = vx + kk
    fn opcode_7xkk(&mut self, x: usize, kk: u8) {
        self.write_v(x, self.read_v(x).wrapping_add(kk));
    }

    // Set vx = vy
    fn opcode_8xy0(&mut self, x: usize, y: usize) {
        self.write_v(x, self.read_v(y)); 
    }

    // Set vx = vx bitwise or vy
    fn opcode_8xy1(&mut self, x: usize, y: usize) {
        self.write_v(x, self.read_v(x) | self.read_v(y));
    }

    // Set vx = vx bitwise and vy
    fn opcode_8xy2(&mut self, x: usize, y: usize) {
        self.write_v(x, self.read_v(x) & self.read_v(y));
    }

    // Set vx = vx bitwise xor vy
    fn opcode_8xy3(&mut self, x: usize, y: usize) {
        self.write_v(x, self.read_v(x) ^ self.read_v(y));
    }

    // Set vx = vx + vy and set vf = carry bit
    fn opcode_8xy4(&mut self, x: usize, y: usize) {
        let (v_sum, carry_flag) = self.read_v(x).overflowing_add(self.read_v(y));
        self.write_v(0xF, carry_flag as u8);
        self.write_v(x, v_sum);
    }

    // Set vx = vx - vy and set vf = carry bit
    fn opcode_8xy5(&mut self, x: usize, y: usize) {
        let (v_diff, carry_flag) = self.read_v(x).overflowing_sub(self.read_v(y));
        self.write_v(0xF, carry_flag as u8);
        self.write_v(x, v_diff);
    }

    // Set vx = vx right shift 1 bit and set vf = carry bit
    fn opcode_8xy6(&mut self, x: usize) {
        let(v_rshift, carry_flag) = self.read_v(x).overflowing_shr(1);
        self.write_v(0xF, carry_flag as u8);
        self.write_v(x, v_rshift);
    }
    
    // Set vx = vx left shift 1 bit and set vf = carry bit
    fn opcode_8xye(&mut self, x: usize) {
        let(v_lshift, carry_flag) = self.read_v(x).overflowing_shl(1);
        self.write_v(0xF, carry_flag as u8);
        self.write_v(x, v_lshift);
    }

    // If vx != vy, skip the next opcode
    fn opcode_9xy0(&mut self, x: usize, y: usize) {
        if self.read_v(x) != self.read_v(y) {
            self.set_pc(ProgramCounter::Next);
        }
    }

    // Set i = nnn
    fn opcode_annn(&mut self, nnn: usize) {
        self.set_i(nnn);
    }

    // Write sprite from ram to vram
    fn opcode_dxyn(&mut self, ram: &Ram, x: usize, y: usize, n: usize) {
        let x_coord = self.read_v(x) as usize;
        let y_coord = self.read_v(y) as usize;

        // RGBA VRAM
        for byte in 0..n { // sprite height
            for bit in 0..8 { // sprite width
                for rgba in 0..4 { // pixel bit expanded to 4 rgba pixels
                    let screen_row = ((y_coord + byte) * 64 * 4) % (32 * 64 * 4);
                    let rgba_pixel = ((x_coord + bit) * 4) % (64 * 4);
                    let rgba_byte = 3 - rgba;
                    let vram_index = screen_row + rgba_pixel + rgba_byte;

                    // Set VRAM A bytes of RGBA to on
                    if rgba == 3 {
                        self.vram[vram_index] = 0xFF;
                    // Toggle VRAM RGB bytes using RGB of RGBA 
                    } else {
                        let sprite_bit = ram.mem[self.i + byte] >> (7 -bit) &0b1;

                        // Vram byte will toggle
                        if sprite_bit > 0 {
                            // If vram byte will toggle 1 to 0, set vf = 1
                            if self.vram[vram_index] > 0 {
                                self.write_v(0xF, 1);
                            // If vram byte will toggle 0 to 1, set vf = 0
                            } else {
                                self.write_v(0xF, 0);
                            }
                            self.vram[vram_index] =
                                self.vram[vram_index] ^ (sprite_bit * 0xFF);
                        // Vram byte will not toggle
                        } else {
                            // Set vf = 0
                            self.write_v(0xF, 0);
                        }
                    }
                }
            }
        }
    }

    // If key with value of vx is pressed, skip the next opcode
    fn opcode_ex9e(&mut self, key_pressed: &bool, keypad: &[bool; 16], x: usize) {
        if *key_pressed {
            let x_val = self.read_v(x);

            'ex9e_key_check: for (i, key) in keypad.iter().enumerate() {
                if i == x_val as usize {
                    if *key {
                        self.set_pc(ProgramCounter::Next);
                    }

                    break 'ex9e_key_check;
                }
            }
        }
    }

    // If key with value of vx is not pressed, skip the next opcode
    fn opcode_exa1(&mut self, key_pressed: &bool, keypad: &[bool; 16], x: usize) {
        if !key_pressed {
            self.set_pc(ProgramCounter::Next);
        } else {
            let x_val = self.read_v(x);

            'exa1_key_check: for (i, key) in keypad.iter().enumerate() {
                if i == x_val as usize {
                    if !key {
                        self.set_pc(ProgramCounter::Next);
                    }

                    break 'exa1_key_check;
                }
            }
        }
    }

    // Set vx = dt
    fn opcode_fx07(&mut self, x: usize) {
        self.write_v(x, self.dt);
    }

    // If key not pressed, decrement pc to loop; Else, set vx = key's Chip 8 hex value
    fn opcode_fx0a(&mut self, key_pressed: &bool, keypad: &[bool; 16], x: usize) {
        if !key_pressed {
            self.set_pc(ProgramCounter::Prev);
        } else {
            'key_assign: for (i, key) in keypad.iter().enumerate() {
                if *key {
                    self.write_v(x, i as u8);

                    // Breaks for loop after first key match
                    // Todo: Find better method
                    break 'key_assign;
                }
            }
        }
    }

    // Set dt = vx
    fn opcode_fx15(&mut self, x: usize) {
        self.dt = self.read_v(x);
    }

    // Set i = i + vx
    // If i overflows ram (0xFFF), set vf = 1 (Add when needed)
    fn opcode_fx1e(&self, ram: &mut Ram, x: usize) {
        ram.write_ram(self.i, ram.read_ram(self.i).wrapping_add(self.read_v(x)));
    }

    // Store BCD representation of vx in memory locations i, i+1, and i+2
    fn opcode_fx33(&self, ram: &mut Ram, x: usize) {
        let val = self.read_v(x);
        let hunds = val / 100;
        let tens = (val / 10) % 10;
        let ones = val % 10;

        ram.write_ram(self.i, hunds);
        ram.write_ram(self.i + 1, tens);
        ram.write_ram(self.i + 2, ones);
    }

    // Store registers V0 through Vx in memory starting at location I
    fn opcode_fx55(&self, ram: &mut Ram, x: usize) {
        for vreg in 0..(x + 1) {
            ram.write_ram(self.i + vreg, self.read_v(vreg));
        }
    }

    // Store values from memory starting at location i in registers v0 through vx
    fn opcode_fx65(&mut self, ram: &Ram, x: usize) {
        for vreg in 0..(x + 1) {
            self.write_v(vreg, ram.read_ram(self.i + vreg));
        }
    }
}