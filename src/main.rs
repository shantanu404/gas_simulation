extern crate gas_simulation;
extern crate sdl2;

use gas_simulation::Environment;
use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::pixels::Color;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let _img_context = sdl2::image::init(InitFlag::PNG).unwrap();

    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Gas Simulation!", 500, 500)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let tc = canvas.texture_creator();
    let psprite = tc.load_texture("res/particle.png").unwrap();

    // Simulation
    let mut world = Environment::new(500f32, 500f32, 1000);

    'mainloop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'mainloop,
                _ => {}
            }
        }

        world.update();

        ::std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        world.render(&mut canvas, &psprite);

        canvas.present();
    }
}
