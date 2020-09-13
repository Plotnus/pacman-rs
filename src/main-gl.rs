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

    //////////////////////////////////////////
    // Compile Shaders & Link programs
    // TODO: have a step for reading the files instead of just using `include_str!
    let vert_source = include_str!("shaders/triangle.vert");
    let vert_shader = gl_render::GlShader::from_source(vert_source, gl::VERTEX_SHADER)?;

    let frag_source = include_str!("shaders/triangle.frag");
    let frag_shader = gl_render::GlShader::from_source(frag_source, gl::FRAGMENT_SHADER)?;

    let shader_program = gl_render::GlProgram::from_shaders(&[vert_shader, frag_shader])?;
    gl_render::use_program(&shader_program);

    /////////////////////////////////////////
    // Rendering Data for VBO etc
    let vertices: Vec<f32> = vec![-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];
    // create and setup the vertex buffer object
    let vbo_handle: gl::types::GLuint = unsafe {
        let mut vbo_handle: gl::types::GLuint = 0;
        gl::GenBuffers(1, &mut vbo_handle);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo_handle);
        gl::BufferData(
            gl::ARRAY_BUFFER,                                                       //target
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, //size
            vertices.as_ptr() as *const gl::types::GLvoid,                          //data
            gl::STATIC_DRAW,                                                        //usage
        );

        // unbind the buffer
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        vbo_handle
    };

    // create and setup a vertex arary object aka DataLayout
    let vao_handle: gl::types::GLuint = unsafe {
        let mut vao_handle: gl::types::GLuint = 0;
        gl::GenVertexArrays(1, &mut vao_handle);
        gl::BindVertexArray(vao_handle);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo_handle);
        gl::EnableVertexAttribArray(0); // this is `layout (location = 0)` in vertex.shader
        gl::VertexAttribPointer(
            0,         // index of the vertex attribute  `layout (location = 0)`
            3,         // number of components per attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized
            (3 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride
            std::ptr::null(), // initial offest
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
        vao_handle
    };

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
        // clear background
        unsafe {
            let color = color_cycler.next().unwrap();
            let r = color.r as f32 / 255.0;
            let g = color.g as f32 / 255.0;
            let b = color.b as f32 / 255.0;
            gl::Viewport(0, 0, SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);
            gl::ClearColor(r, g, b, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        // draw triangle
        gl_render::use_program(&shader_program);
        unsafe {
            gl::BindVertexArray(vao_handle);
            gl::DrawArrays(
                gl::TRIANGLES, // mode
                0,             // starting index
                3,             // number of elements to render
            );
        }
        sdl_window.gl_swap_window();

        //////////////////////////////////////////
        // WAIT FOR FRAME
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
