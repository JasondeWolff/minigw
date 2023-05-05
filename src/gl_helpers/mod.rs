extern crate gl;
extern crate glutin;

use gl::types::*;

pub mod buffers;
pub use buffers::*;
pub mod shaders;
pub use shaders::*;
pub mod textures;
pub use textures::*;
pub mod imgui_impl;
pub use imgui_impl::*;

use crate::Window;

pub fn gl_init(window: &Window) {
    gl::load_with(|ptr| window.internal_context().get_proc_address(ptr) as *const _);
    gl_check();
}

fn gl_check() {
    unsafe {
        let error = gl::GetError();
        match error {
            gl::NO_ERROR => return,
            gl::INVALID_ENUM => panic!("GL invalid enum."),
            gl::INVALID_VALUE => panic!("GL invalid value."),
            gl::INVALID_OPERATION => panic!("GL invalid operation."),
            gl::OUT_OF_MEMORY => panic!("GL out of memory."),
            gl::STACK_OVERFLOW => panic!("GL stack overflow."),
            gl::STACK_UNDERFLOW => panic!("GL stack underflow"),
            _ => panic!("GL unkown error."),
        }
    }
}

pub fn gl_enable_depth() {
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl_check();
    }
}

pub fn gl_cull(mode: GLenum) {
    unsafe {
        gl::Enable(gl::CULL_FACE);
        gl::CullFace(mode);
        gl_check();
    }
}

pub fn gl_clear_color(color: cgmath::Vector3<f32>) {
    unsafe {
        gl::ClearColor(color.x, color.y, color.z, 1.0f32);
        gl_check();
    }
}

pub fn gl_clear() {
    unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        gl_check();
    }
}

pub fn gl_viewport(width: u32, height: u32) {
    unsafe {
        gl::Viewport(0, 0, width as i32, height as i32);
        gl_check();
    }
}

pub fn gl_draw_elems(mode: GLenum, count: usize, index_type: GLenum) {
    unsafe {
        gl::DrawElements(mode, count as i32, index_type, std::ptr::null());
        gl_check();
    }
}

pub fn gl_draw_arrays(mode: GLenum, offset: usize, count: usize) {
    unsafe {
        gl::DrawArrays(mode, offset as i32, count as i32);
        gl_check();
    }
}

pub fn gl_finish() {
    unsafe {
        gl::Finish();
        gl_check();
    }
}