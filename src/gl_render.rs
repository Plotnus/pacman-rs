use gl;
use std;
use std::ffi::{CString};

pub fn use_program(program: &GlProgram) {
    // TODO: add error handling
    unsafe {
        gl::UseProgram(program.handle);
    }
}

pub struct GlProgram {
    pub handle: gl::types::GLuint,
}

impl GlProgram {
    pub fn from_shaders(shaders: &[GlShader]) -> Result<GlProgram, String> {
        let program_handle = unsafe { gl::CreateProgram() };
        for shader in shaders {
            unsafe {
                gl::AttachShader(program_handle, shader.h);
            }
        }

        unsafe {
            gl::LinkProgram(program_handle);
        }

        let link_status = {
            let mut status = 0;
            unsafe {
                gl::GetProgramiv(program_handle, gl::LINK_STATUS, &mut status);
            }
            status
        };

        if link_status == 0 {
            let error_msg = {
                let gl_log_len = {
                    let mut len: gl::types::GLint = 0;
                    unsafe {
                        gl::GetProgramiv(program_handle, gl::INFO_LOG_LENGTH, &mut len);
                    }
                    len as usize
                };

                let msg = cstring_from_len(gl_log_len);
                unsafe {
                    gl::GetProgramInfoLog(
                        program_handle,
                        gl_log_len as gl::types::GLint,
                        std::ptr::null_mut(),
                        msg.as_ptr() as *mut gl::types::GLchar,
                    );
                }
                msg.to_string_lossy().into_owned()
            };
            return Err(error_msg);
        };

        Ok(GlProgram {
            handle: program_handle,
        })
    }
}

impl Drop for GlProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.handle);
        }
    }
}

pub struct GlShader {
    pub h: gl::types::GLuint,
}

impl GlShader {
    pub fn from_source(source: &str, shader_type: gl::types::GLuint) -> Result<GlShader, String> {
        let shader_handle = unsafe { gl::CreateShader(shader_type) };

        unsafe {
            let len: gl::types::GLint = source.len() as gl::types::GLint;
            let src_ptr = source.as_ptr() as *const gl::types::GLchar;
            gl::ShaderSource(
                shader_handle,
                1,
                &src_ptr as *const *const gl::types::GLchar,
                & len,
            );
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
                let error_msg = cstring_from_len(gl_log_length);

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

        Ok(GlShader { h: shader_handle })
    }
}

impl Drop for GlShader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.h);
        }
    }
}

fn cstring_from_len(len: usize) -> CString {
    let mut buffer: Vec<u8> = vec![b' '; len + 1];
    buffer[len] = 0;
    unsafe { CString::from_vec_unchecked(buffer) }
}
