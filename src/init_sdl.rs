#![allow(dead_code)]
#![allow(unused_imports)]

// Aliases
use sdl2::{
    event, 
    video, 
    keyboard, 
    pixels, 
    rect, 
    render, 
    VideoSubsystem, 
    EventPump, 
    render::WindowCanvas, 
    Sdl, 
    init,
    event::Event
};

// Contains all required data to initialize SDL and prepare a window
pub struct Context {
    pub context: Sdl,
    pub video: VideoSubsystem,
    pub window: video::Window,
    pub canvas: WindowCanvas,
    pub event_pump: sdl2::EventPump
} 

impl Context {
    pub fn new() -> Result<Context, String> {
        
        let context  = init()?;
        let video = context.video()?;
        
        // Create and initialize window
        let window = video
            .window("Game Window", 800, 600)
            .resizable()
            .build()
            .unwrap();
        
        // Create and initialize renderer
        let canvas: WindowCanvas = window
            .to_owned()
            .into_canvas()
            .accelerated()
            .present_vsync()
            .build()
            .unwrap();

        // Create and initialize event pipeline
        let event_pump = context.event_pump().unwrap();

        // Check for errors and return new instance of the sdl context
        return Ok(Context {
            context,
            video,
            window,
            canvas,
            event_pump
        });
    }
}