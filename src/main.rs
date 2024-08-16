mod init_sdl;
use init_sdl::Context;

mod chip8;
mod main_loop;

fn main() {
    let mut ctx = Context::new().unwrap();
    main_loop::main_loop(&mut ctx);
}