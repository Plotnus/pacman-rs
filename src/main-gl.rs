extern crate gl;
extern crate sdl2;

use std::ffi::{CStr, CString};

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

fn _shader_from_source(
    source: &CStr,
    shader_type: gl::types::GLuint,
) -> Result<gl::types::GLuint, String> {
    let shader_handle = unsafe { gl::CreateShader(shader_type) };

    unsafe {
        // set shader source
        gl::ShaderSource(shader_handle, 1, &source.as_ptr(), std::ptr::null());

        // compile the shader
        gl::CompileShader(shader_handle);
    }

    let mut did_compilation_succeed: gl::types::GLint = 0;
    unsafe {
        gl::GetShaderiv(
            shader_handle,
            gl::COMPILE_STATUS,
            &mut did_compilation_succeed,
        );
    }

    let did_compilation_succeed = unsafe {
        let mut status = 0;
        gl::GetShaderiv(shader_handle, gl::COMPILE_STATUS, &mut status);
        status != 0
    };

    if !did_compilation_succeed {
        // gl error buffer contents -> String
        let gl_log_length = unsafe {
            let mut gl_log_length: gl::types::GLint = 0;
            gl::GetShaderiv(shader_handle, gl::INFO_LOG_LENGTH, &mut gl_log_length);
            gl_log_length as usize
        };

        let error_msg: String = unsafe {
            let error_msg: CString = {
                let mut buffer: Vec<u8> = vec![b' '; gl_log_length + 1];
                buffer[gl_log_length] = 0;
                CString::from_vec_unchecked(buffer)
            };

            // populate the message
            gl::GetShaderInfoLog(
                shader_handle,
                gl_log_length as gl::types::GLint,
                std::ptr::null_mut(),
                error_msg.as_ptr() as *mut gl::types::GLchar,
            );

            error_msg.to_string_lossy().into_owned()
        };

        return Err(error_msg);
    }

    Ok(shader_handle)
}
