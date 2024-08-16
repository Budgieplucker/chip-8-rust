use crate::init_sdl::*;

extern crate sdl2;
use sdl2::{event::Event, keyboard};

use crate::chip8::Emu;

// Main loop
pub fn main_loop(mut context: &mut Context) {
    let mut running = true;

    // Start chip8 and initialize values
    let mut chip8: Emu = Emu::new();
    chip8.reset();
    chip8.load_rom("roms/IBM.ch8".to_string());
    
    while running {
        let op: u16 = chip8.fetch_opcode();
        chip8.execute(op);
        draw_screen(&mut chip8, &mut context.canvas);
        event_loop(&mut running, context)
    }
} 

// Polls for events 
pub fn event_loop(loop_cond: &mut bool, context: &mut Context) {
    for event in context.event_pump.poll_iter() {
        match event {
            Event::Quit { .. } |
            Event::KeyDown {keycode: Some(keyboard::Keycode::ESCAPE), ..} => {*loop_cond = false;},
            _=>{}
        }
    }
}

fn draw_screen(emu: &Emu, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    // Clear canvas as black
    canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
    canvas.clear();

    let screen_buf = emu.screen;
    // Now set draw color to white, iterate through each point and see if it should be drawn
    canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
    for (i, pixel) in screen_buf.iter().enumerate() {
        if *pixel {
            // Convert our 1D array's index into a 2D (x,y) position
            let x = (i % 64) as u32;
            let y = (i / 64) as u32;

            // Draw a rectangle at (x,y), scaled up by our SCALE value
            let rect = sdl2::rect::Rect::new((x * 20) as i32, (y * 20) as i32, 20, 20);
            canvas.fill_rect(rect).unwrap();
        }
    }
    canvas.present();
}