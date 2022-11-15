//Module Todo:
// Remove temporary pieces for display testing:
//     - CLI sprite display in DXYN()
//     - x and y coord test print in DXYN()
// Continue implementing instruction functions and match conditional
// Build timer functions

use crate::{
    ram::Ram, 
    ROM_START,
};

const OPCODE_INTERVAL: usize = 2;

#[derive(Clone, Copy)]
enum ProgramCounter {
    Next,
    Jump(usize),
}

pub struct Cpu {
    pc: usize,
    sp: usize, //May not need due to .push() and .pop()
    i: usize,
    v: [u8; 16],
    stack: [u16; 16], //Keep stack an array for now. Use vector if issues arise.
    pub vram: [u8; 32 * 64 * 4], // RGBA VRAM (Height: 32, Width: 64, RGBA: 4)
    // pub vram: [[u8; 64]; 32], //Legacy VRAM
    // pub vram_update: bool,
    // dt: u8, //Todo: Implement Delay Timer
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
            // vram: [[0; 64]; 32], // Legacy VRAM
            // vram_update: false,
            // dt: 0,
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

    pub fn tick(&mut self, ram: &Ram) {
        // self.vram_update = false;
        let current_opcode = self.fetch_opcode(&ram);
        self.execute_opcode(&ram, &current_opcode);
    }

    pub fn fetch_opcode(&mut self, ram: &Ram) -> u16 {
        let opcode = (ram.mem[self.pc] as u16) << 8 | (ram.mem[self.pc + 1] as u16);
        self.set_pc(ProgramCounter::Next);
        opcode
    }

    pub fn execute_opcode(&mut self, ram: &Ram, current_opcode: &u16) {
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
            (0x09,    _,    _, 0x00) => self.opcode_9xy0(x, y),
            (0x0A,    _,    _,    _) => self.opcode_annn(nnn),
            (0x0D,    _,    _,    _) => self.opcode_dxyn(&ram, x, y, n),
            (0x0F,    _, 0x06, 0x05) => self.opcode_fx65(&ram, x),
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

                    // // Legacy VRAM Clear
                    // self.vram[height][width] = 0;
                    
                    // self.vram_update = true;
                }
            }
        }
    }

    // Return from a subroutine
    fn opcode_00ee(&mut self) {
        self.sp -= 1;
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
        let sum = x + y;
        // Check if sum overflows u8
        if sum > 255 {
            // Once u8 overflow is encountered, figure out carry bit handling
            println!("Finish carry bit handling for opcode 8XY4");
            panic!("X = {:b}, Y = {:b}, Sum = {:b}", x, y, sum);
        }
        self.write_v(x, sum as u8);
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
    // Add detailed comments to code below
    fn opcode_dxyn(&mut self, ram: &Ram, x: usize, y: usize, n: usize) {
        let x_coord = self.read_v(x) as usize;
        let y_coord = self.read_v(y) as usize;

        // RGBA VRAM
        for byte in 0..n {
            for bit in 0..8 {
                for rgba in 0..4 {
                    if rgba == 3 {
                        // Write A of RGBA to VRAM
                        self.vram[((y_coord + byte) * 64 * 4) + ((x_coord + bit) * 4) + (3 - rgba)] = 255;

                        // // Legacy VRAM Alpha Channel Write
                        // self.vram[byte + y_coord][(bit * rgba) + x_coord] = 255;
                    }
                    else {
                        // Write RGB of RGBA to VRAM
                        self.vram[((y_coord + byte) * 64 * 4) + ((x_coord + bit) * 4) + (3 - rgba)] =
                            ((ram.mem[self.i + byte] >> (7 - bit)) & 0b0000_0001) * 255;

                        // // Legacy VRAM Bit Color Write
                        // self.vram[byte + y_coord][(bit * rgba) + x_coord] = 
                        //     ((ram.mem[self.i + byte] >> (7 - bit)) & 0b0000_0001) * 255;
                    }
                }
            }
        }

        // // Legacy VRAM
        // for byte in 0..n {
        //     for bit in 0..8 {
        //         self.vram[byte + y_coord][bit + x_coord] = (ram.mem[self.i + byte] >> (7 - bit)) & 0b0000_0001;

        //         // CLI debug display of the sprite pulled from ram
        //         // match self.vram[byte + y_coord][bit + x_coord] {
        //         //     1 => print!("#"),
        //         //     0 => print!(" "),
        //         //     _ => println!("Invalid print target"),
        //         // }
        //     }
        //     // println!();
        // }

        // self.vram_update = true;
    }

    // Write values from ram at i through i+x into v0 through vx
    fn opcode_fx65(&mut self, ram: &Ram, x: usize) {
        for vreg in 0..(x + 1) {
            let data = ram.mem[self.i + vreg];
            self.write_v(x, data);
        }
    }
}