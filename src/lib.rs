pub mod rc_cell;
pub use rc_cell::*;
mod core_loop;
use core_loop::*;
pub mod window;
pub use window::*;
pub mod input;
pub use input::*;

pub fn new<F: Fn(RcCell<Window>, RcCell<Input>) + 'static>(title: &'static str, core_update: F) {
    let core_loop = CoreLoop::new();
    let window = Window::new(&core_loop, title, 1280, 720);
    let input = Input::new(window.clone());

    core_loop.run(
        core_update,
        window,
        input
    );
}