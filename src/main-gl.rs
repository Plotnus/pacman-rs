extern crate gl;
extern crate sdl2;
pub mod gl_render;

//use std::ffi::{CStr, CString};

use std::result::Result;
use std::string::String;

fn main() -> Result<(), String> {
    const SCREEN_WIDTH: u32 = 1280;
    const SCREEN_HEIGHT: u32 = 720;

    let sdl_context = sdl2::init()?;

    let sdl_video = sdl_context.video()?;

    let gl_attr = sdl_video.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);

    let sdl_window = sdl_video
        .window("gl", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let _gl_context = sdl_window.gl_create_context().unwrap();
    let _gl = gl::load_with(|s| sdl_video.gl_get_proc_address(s) as *const std::os::raw::c_void);

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

        //////////////////////////////////////////
        // INPUT
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
        //////////////////////////////////////////
        // UPDATE
        //////////////////////////////////////////
        // RENDER
        unsafe {
            let color = color_cycler.next().unwrap();
            let r = color.r as f32 / 255.0;
            let g = color.g as f32 / 255.0;
            let b = color.b as f32 / 255.0;
            gl::Viewport(0, 0, SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);
            gl::ClearColor(r, g, b, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        sdl_window.gl_swap_window();

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
