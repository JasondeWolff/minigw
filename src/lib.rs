pub extern crate cgmath;
pub extern crate imgui;
pub use cgmath::*;

pub mod rc_cell;
pub use rc_cell::*;

pub mod input;
pub use input::*;

mod window;
use window::*;
mod core_loop;
use core_loop::*;
mod renderer;
use renderer::*;
mod gl_helpers;
use gl_helpers::DebugUI;

pub fn new<T: Copy + Default + 'static, F: FnMut(RcCell<Input>, RcCell<RenderTextureView<T>>, &mut DebugUI) + 'static>(title: &'static str, core_update: F) {
    let core_loop = CoreLoop::new();
    let window = Window::new(&core_loop, title, 1280, 720);
    let input = Input::new(window.clone());

    core_loop.run(
        core_update,
        window,
        input
    );
}