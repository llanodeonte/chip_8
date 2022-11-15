//Module Todo:
// Currently populating GUI one pixel at a time (BAD)
// Working to implement a texture update method for the GUI using RGBA (GOOD)
// Refer to texture variable at line 61 for notes on toggling between legacy and RGBA GUI

use sdl2::{
    // rect::Rect,
    render::{Canvas, Texture},
    video::Window,
    pixels::Color,
    // pixels::PixelFormatEnum,
};

use crate::{
    cpu::Cpu,
    CHIP8_WIDTH,
    CHIP8_HEIGHT,
};

const SCALE_FACTOR: u32 = 20;
const DISPLAY_WIDTH: u32 = CHIP8_WIDTH * SCALE_FACTOR;
const DISPLAY_HEIGHT: u32 = CHIP8_HEIGHT * SCALE_FACTOR;

pub const FONT_SET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct Display {
    pub canvas: Canvas<Window>,
}

impl Display {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        let video_subsystem = sdl_context
            .video()
            .expect("Failed to initialize the video subsystem");
        let window = video_subsystem
            .window("Chip 8", DISPLAY_WIDTH, DISPLAY_HEIGHT)
            .position_centered()
            .build()
            .expect("Failed to build a new window");
        let mut canvas = window
            .into_canvas()
            .build()
            .expect("Failed to build canvas");

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        Self {
            canvas,
        }
    }

    pub fn draw(&mut self, cpu: &Cpu, texture: &mut Texture) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));

        // Update texture with RGBA VRAM contents
        texture.update(None, cpu.vram.as_slice(), CHIP8_WIDTH as usize * 4)
            .expect("Failed to update texture");

        // Copy current texture contents to canvas
        self.canvas.copy(texture, None, None)
            .expect("Failed to copy texture to canvas");
        
        // // Legacy VRAM Display
        // for width in 0..CHIP8_HEIGHT {
        //     for pixel in 0..CHIP8_WIDTH {
        //         if cpu.vram[width as usize][pixel as usize] == 1 {
        //             let x_coord = (pixel * SCALE_FACTOR) as i32;
        //             let y_coord = (width * SCALE_FACTOR) as i32;
        //             let pixel = Rect::new(x_coord, y_coord, SCALE_FACTOR, SCALE_FACTOR);
        //             self.canvas.fill_rect(pixel).expect("Failed to draw rect");
        //         }
        //     }
        // }

        self.canvas.present();
    }
}