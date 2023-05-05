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

pub fn new<T, F>(
    title: &'static str,
    width: u32,
    height: u32,
    core_update: F
) where
    T: RenderTextureType + 'static,
    F: FnMut(RcCell<Input>, RcCell<RenderTextureView<T>>, &mut DebugUI) + 'static
{
    let core_loop = CoreLoop::new();
    let window = Window::new(&core_loop, title, width, height);
    let input = Input::new(window.clone());

    core_loop.run(
        core_update,
        window,
        input
    );
}