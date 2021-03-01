mod gui;
use gui::UnGUI::*;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");



    let mut my_canvas = UnCanvas::new(AbsoluteMeta::new(Vector2D::new(0, 0)).set_dimension(800, 600).clone());
    let mut my_background = UnBackground::new(RGB::new(30, 60, 250), RelativeMeta::new(Vector2D::new(0, 0)));
    let mut my_layout = UnLayoutAbsolute::new(RelativeMeta::new(Vector2D::new(0, 0)));

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();




    let mut i = 0;
    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        i = (i+1)%255;



        my_background.set_color(RGB::new(i, 69, 255-i));
        my_canvas.display(&mut canvas,
                            &[(&my_background, 1)]);





        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }



    Ok(())
}