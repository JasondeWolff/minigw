//! minigw
//! ======
//! 
//! [![minigw crate](https://img.shields.io/crates/v/minigw.svg)](https://crates.io/crates/minigw)
//! ![minimum rustc 1.0](https://img.shields.io/badge/rustc-1.0+-red.svg)
//! [![minigw documentation](https://docs.rs/minigw/badge.svg)](https://docs.rs/minigw)
//! 
//! A convenient Rust library for creating cross platform windows and displaying pixel buffers. It also makes it easy to get keyboard and mouse input. There is full imgui rendering support build-in.
//! 
//! ![Example](screenshots/example.png)
//! 
//! ## Usage
//! Add this to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! minigw = "0.0.4"
//! ```
//! This example shows how to create a window and how to draw a gradient every frame.
//! ```rust
//! extern crate minigw;
//! 
//! fn main() {
//!     minigw::new::<u8, _>("Example", 1280, 720,
//!         move |input, render_texture, imgui| {  
//!             let mut render_texture = render_texture.as_mut();
//! 
//!             // Draw a red and green gradient.
//!             for x in 0..render_texture.width() {
//!                 for y in 0..render_texture.height() {
//!                     let uv = (x as f32 / render_texture.width() as f32, y as f32 / render_texture.height() as f32);
//!                     render_texture.set_pixel(x, y, &[(uv.0 * 255.99) as u8, (uv.1 * 255.99) as u8, 0]);
//!                 }
//!             }
//!         });
//! }
//! ```
//! ## Planned features
//! - [X] Gamma correction
//! - [X] Window icon
//! - [X] Framebuffer scaling
//! - [ ] f32 (HDR) colour conversion
//! - [ ] Adjustable colour grading
//! - [ ] Dedicated render thread
//! 
//! ## License
//! This project is licensed under the MIT license ([LICENSE-MIT](LICENSE.md) or <https://opensource.org/licenses/MIT>).
//! 
//! <br/><br/>
//! 
//! # Examples
//! ## Input Handling & ImGui
//! Input can be handled by querying the state of the input struct. This example will toggle the cursor state between unlocked and locked every time the Space Bar has been pressed. Debug UI can be drawn by directly accessing the `imgui::Ui` struct.
//! ```rust
//! extern crate minigw;
//! use minigw::imgui;
//! 
//! fn main() {
//!     minigw::new::<u8, _>("Example", 1280, 720,
//!         move |input, render_texture, imgui| {
//!             // ...
//! 
//!             // Toggle the cursor mode between FREE and LOCKED.
//!             let mut input_mut = input.as_mut();
//!             if input_mut.key_down(minigw::VirtualKeyCode::Space) {
//!                 input_mut.toggle_cursor_mode();
//!             }
//! 
//!             imgui.window("Example window")
//!                 .size([400.0, 700.0], imgui::Condition::FirstUseEver)
//!                 .build(|| {
//!                     let mut x = 0.0;
//!                     imgui.slider("Slider", 0.0, 1.0, &mut x);
//!                 });
//!         });
//! }
//! ```
//! 
//! ## Framebuffer type
//! The framebuffer type can be defined when creating a new window. `u8` works best for performance reasons but other supported types are: `i8`, `u16`, `i16`, `u32`, `i32` and `f32`. The same gradient example but now with an `f32` framebuffer can be seen below.
//! ```rust
//! extern crate minigw;
//! 
//! fn main() {
//!     minigw::new::<f32, _>("Example", 1280, 720,
//!         move |input, render_texture, imgui| {  
//!             // ...
//!                 // ...
//!                     render_texture.set_pixel(x, y, &[uv.0, uv.1, 0]);
//!         });
//! }
//! ```
//! 
//! ## Window & Framebuffer resizing
//! The resizing of the window can be enabled and disabled. The framebuffer's rendertexture can: resize with the window, resize with the window scaled by a factor or don't resize with the window. The example below shows how to switch between these modes.
//! ```rust
//! extern crate minigw;
//! 
//! fn main() {
//!     let mut mode = 0;
//! 
//!     minigw::new::<u8, _>("Example", 1280, 720,
//!     move |window, input, render_texture, _imgui| {
//!         let window_mut = window.as_mut();
//!         let input_mut = input.as_mut();
//!         let mut render_texture = render_texture.as_mut();
//! 
//!         // Draw checkerboard pattern.
//!         for x in 0..render_texture.get_width() {
//!             for y in 0..render_texture.get_height() {
//!                 if (x / 60 + y / 60) % 2 == 0 {
//!                     render_texture.set_pixel(x, y, &[255, 255, 255]);
//!                 } else {
//!                     render_texture.set_pixel(x, y, &[0, 0, 0]);
//!                 }
//!             }
//!         }
//!         
//!         // Toggle window resizability.
//!         if input_mut.key_down(minigw::VirtualKeyCode::Space) {
//!             window_mut.set_resizable(!window_mut.is_resizable());
//!         }
//! 
//!         // Loop over all RenderTextureResizing modes.
//!         if input_mut.key_down(minigw::VirtualKeyCode::M) {
//!             mode = (mode + 1) % 3;
//! 
//!             let rtm = match mode {
//!                 0 => minigw::RenderTextureResizing::Resizable,
//!                 1 => minigw::RenderTextureResizing::ResizableScaled(0.3),
//!                 _ => minigw::RenderTextureResizing::NonResizable
//!             };
//! 
//!             render_texture.set_resizing_mode(rtm);
//!         }
//!     });
//! }
//! ```
//! ## Adding a window & taskbar icon
//! Adding a window icon requires a `Vec<u8>` of pixel data. To get the data this example uses the [stb_image crate](https://crates.io/crates/stb_image) but feel free to use any image library that suits your needs best.
//! ```rust
//! extern crate minigw;
//! extern crate stb_image;
//! 
//! fn load_img(path: &str) -> (Vec<u8>, u32, u32) {
//!     let cpath = std::ffi::CString::new(path.as_bytes()).unwrap();
//! 
//!     unsafe {
//!         let mut width = 0;
//!         let mut height = 0;
//!         let mut channels = 0;
//!         let data = stb_image::stb_image::bindgen::stbi_load(
//!             cpath.as_ptr(),
//!             &mut width,
//!             &mut height,
//!             &mut channels,
//!             4
//!         );
//!         assert!(!data.is_null(), "Failed to read image file at \"{:?}\"", path);
//!         let data: Vec<u8> = std::slice::from_raw_parts(data, (width * height * 4) as usize).to_vec();
//! 
//!         (data, width as u32, height as u32)
//!     }
//! }
//! 
//! fn main() {
//!     let mut once = true;
//! 
//!     minigw::new::<u8, _>("Example", 1280, 720,
//!     move |window, _input, _render_texture, _imgui| {
//!         if once {
//!             once = false;
//! 
//!             let (rgba, width, height) = load_img("assets/rust.png");
//!             let icon = minigw::window::Icon::from_rgba(rgba, width, height).unwrap();
//!             window.as_mut().set_icon(Some(icon));
//!         }
//!     });
//! }
//! ```

#![warn(clippy::all)]
#![allow(clippy::manual_memcpy)]

pub extern crate cgmath;
pub extern crate imgui;

pub mod rc_cell;
pub use rc_cell::*;

pub mod input;
pub use input::*;
pub mod renderer;
pub use renderer::*;
pub mod window;
pub use window::*;

mod core_loop;
use core_loop::*;
mod gl_helpers;
use gl_helpers::DebugUI;

/// Creates a new minigw game window.
pub fn new<T, F>(
    title: &'static str,
    width: u32,
    height: u32,
    core_update: F
) where
    T: RenderTextureType + 'static,
    F: FnMut(RcCell<Window>, RcCell<Input>, RcCell<RenderTexture<T>>, &mut DebugUI) + 'static
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