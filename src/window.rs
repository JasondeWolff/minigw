use crate::RcCell;
use crate::CoreLoop;

use glutin::platform::windows::WindowExtWindows;
pub use glutin::window::{Icon, BadIcon, CursorIcon};

/// Wrapper around a glutin window.
pub struct Window {
    context: glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>,
    support_pbo: bool
}

impl Window {
    pub(crate) fn new(core_loop: &CoreLoop, title: &'static str, width: u32, height: u32) -> RcCell<Self> {        
        let window_builder = glutin::window::WindowBuilder::new()
            .with_title(title)
            .with_inner_size(glutin::dpi::LogicalSize::new(width, height));

        let mut support_pbo = true;
        let context = glutin::ContextBuilder::new()
            .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (3, 0)))
            .build_windowed(window_builder.clone(), core_loop.winit_loop())
            .unwrap_or_else(|_| -> _ {
                support_pbo = false;
                glutin::ContextBuilder::new()
                    .with_gl(glutin::GlRequest::GlThenGles {
                         opengl_version: (2, 0),
                         opengles_version: (3, 0),
                     })
                    .build_windowed(window_builder, core_loop.winit_loop())
                    .expect("Failed to create context.")
            });

        let context = unsafe {
            context.make_current()
                .expect("Failed to make context current.")
        };

        RcCell::new(Window {
            context,
            support_pbo
        })
    }

    pub(crate) fn internal_context(&self) -> &glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window> {
        &self.context
    }

    pub(crate) fn internal_window(&self) -> &glutin::window::Window {
        self.context.window()
    }

    pub(crate) fn support_pbo(&self) -> bool {
        self.support_pbo
    }

    /// Get inner with.
    pub fn get_width(&self) -> u32 {
        self.internal_window().inner_size().width
    }

    /// Get inner height.
    pub fn get_height(&self) -> u32 {
        self.internal_window().inner_size().height
    }

    /// Set inner width.
    pub fn set_width(&self, width: u32) {
        self.internal_window().set_inner_size(
            glutin::dpi::LogicalSize::new(
                width,
                self.internal_window().inner_size().height
            )
        );
    }

    /// Set inner height.
    pub fn set_height(&self, height: u32) {
        self.internal_window().set_inner_size(
            glutin::dpi::LogicalSize::new(
                self.internal_window().inner_size().width,
                height
            )
        );
    }

    /// Set the window and taskbar icon, if `icon == None` the os default window icon will be used.
    pub fn set_icon(&self, icon: Option<Icon>) {
        self.internal_window().set_window_icon(icon.clone());
        self.internal_window().set_taskbar_icon(icon);
    }

    /// Set the cursor icon.
    pub fn set_cursor_icon(&self, cursor: CursorIcon) {
        self.internal_window().set_cursor_icon(cursor);
    }

    /// Get if the window is resizable.
    pub fn is_resizable(&self) -> bool {
        self.internal_window().is_resizable()
    }

    /// Set if the window is resizable.
    pub fn set_resizable(&self, resizable: bool) {
        self.internal_window().set_resizable(resizable);
    }
}