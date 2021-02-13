extern crate sdl2;
extern crate serde_json;
extern crate serde;
#[macro_use]
extern crate serde_derive;

mod settingsfileload;

use sdl2_sys::*;

const version: &str = "0.0.2";

fn main()
{
    // initialize video
    let sdl = sdl2::init().unwrap();
    let sdlvideo = sdl.video().unwrap();

    // window and renderer
    let window: *mut SDL_Window = sdlvideo.window("Grocery Tycoon", 1024, 768)
        .position_centered()
        .build().unwrap().raw();
    // SDL_RENDERER_ACCELERATED = 2
    // SDL itself should be rewritten in Rust
    let renderer = SDL_CreateRenderer(window, -1, 2);

    'mainloop: loop {
        // lol
    }
}
