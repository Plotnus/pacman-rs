use gl;
use std;
use std::ffi::{CStr, CString};

pub struct GlProgram {
    pub handle: gl::types::GLuint,
}

impl GlProgram {
    pub fn from_shaders(shaders: &[Shader]) -> Result<GlProgram, String> {
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

pub struct Shader {
    pub h: gl::types::GLuint,
}

impl Shader {
    pub fn from_source(source: &CStr, shader_type: gl::types::GLuint) -> Result<Shader, String> {
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

        Ok(Shader { h: shader_handle })
    }
}

impl Drop for Shader {
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
