extern crate sdl2;
extern crate ndarray;
pub mod image_utility;
use image_utility::ArrayUtility;
// i needed to do this: 
// in the Cargo.toml [dependencies] < sdl2 = "0.35.2"
// sudo apt-get install libsdl2-dev libsdl2-image-dev libsdl2-ttf-dev


use sdl2::pixels::PixelFormatEnum;
use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;
use std::any::Any;
use std::array;
use std::time::Duration;
use sdl2::video::Window;
use ndarray::prelude::*;

use ndarray::{Array2, arr2, iter::Windows};


fn main() {
    let window_size: (usize, usize) = (1000, 1000);
    let colour_array: ArrayBase<ndarray::OwnedRepr<u8>, Dim<[usize; 2]>> = array![[0,0,0, 255], [0, 0, 255, 255], [0, 255, 0, 255], [255, 0, 0, 255]];

    let sdl_context: sdl2::Sdl = sdl2::init().unwrap();
    let video_subsystem: sdl2::VideoSubsystem = sdl_context.video().unwrap();

    let window: Window = video_subsystem.window("rust-sdl2 demo", window_size.1 as u32, window_size.0 as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas: sdl2::render::Canvas<Window> = window.into_canvas().build().unwrap();
    let texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext> = canvas.texture_creator();
    let mut event_pump: sdl2::EventPump = sdl_context.event_pump().unwrap();

    let flat_image_data: Vec<u8> = ArrayUtility::generate_noise(window_size.0, window_size.1);
    
    println!("flat image data: {}, {}", flat_image_data.len()/window_size.1, flat_image_data.len()/window_size.0);
    let conditional_probabilities = arr2(&[[1., 1., 0.], [0.1, 1., 0.1], [0., 1., 1.]]);
    let normalized = ArrayUtility::normalize_rows(conditional_probabilities);
    println!("{:?}", normalized);
    let landscape = ArrayUtility::create_array(window_size, normalized);
    
    // let switch_to_mono = false;
    // let pixel_format = PixelFormatEnum::RGB888;
    
    let mut landscape_u8: Vec<u8> = ArrayUtility::land_to_colours(landscape, colour_array);
    println!("landscape u8: {:?}", landscape_u8.len());
    println!("landscape 1,1: {:?}", &landscape_u8[0..30]);
    canvas.present();
    'running: loop {
        canvas.clear();
        // let mut flat_image_data: Vec<u8> = ArrayUtility::generate_noise(window_size.0, window_size.1);
        
        // let mut landscape_u8: Vec<u8> = ArrayUtility::land_to_colours(landscape, colour_array);
        let surface: sdl2::surface::Surface<'_> = sdl2::surface::Surface::from_data(
            // flat_image_data.as_mut_slice(),
            landscape_u8.as_mut_slice(),
            window_size.1 as u32, 
            window_size.0 as u32, 
            (window_size.1 as u32)*4, 
            PixelFormatEnum::RGBA32
        ).unwrap();

        let texture: sdl2::render::Texture<'_> = texture_creator.create_texture_from_surface(&surface).unwrap();
        
        canvas.copy(&texture, None, Some(Rect::new(0, 0, window_size.1 as u32, window_size.0 as u32))).unwrap();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
