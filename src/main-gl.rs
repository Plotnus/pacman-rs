extern crate gl;
extern crate sdl2;

use std::result::Result;
use std::string::String;

fn main() -> Result<(), String> {
    const SCREEN_WIDTH: u32 = 1280;
    const SCREEN_HEIGHT: u32 = 720;

    let sdl_context = sdl2::init()?;
    let sdl_video = sdl_context.video()?;
    let sdl_window = sdl_video
        .window("gl", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = sdl_window
        .into_canvas()
        .build()
        .map_err(|e| e.to_string())?;
    canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut events = sdl_context.event_pump()?;

    let target_fps = 30.0;
    let target_frame_duration = std::time::Duration::from_secs_f64(1.0 / target_fps);

    let colors = [
        sdl2::pixels::Color::RGB(255, 0, 0),
        sdl2::pixels::Color::RGB(255, 255, 0),
        sdl2::pixels::Color::RGB(0, 255, 0),
        sdl2::pixels::Color::RGB(0, 255, 255),
        sdl2::pixels::Color::RGB(0, 0, 255),
        sdl2::pixels::Color::RGB(255, 0, 255),
    ];
    let mut color_cycler = colors.iter().cycle();


    'main: loop {
        let frame_start_time = std::time::Instant::now();

        for event in events.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::Escape),
                    ..
                } => break 'main,
                _ => {}
            }
        }

        canvas.set_draw_color(*color_cycler.next().unwrap());
        canvas.clear();
        canvas.present();

        // wait out the frame
        if frame_start_time.elapsed() > target_frame_duration {
            let elapsed_time = frame_start_time.elapsed().as_secs_f64();
            let target_time = target_frame_duration.as_secs_f64();
            println!(
                "WARNING: OVER TIME BUDGET BY {:.2}%)",
                (elapsed_time / target_time) - 1.0
            );
        } else {
            // waste time: `while` loop more accurate than `std::thread::sleep`
            while frame_start_time.elapsed() < target_frame_duration {}
        }
        // TODO: render FPS to screen
    }

    Ok(())
}
