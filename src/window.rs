use crate::RcCell;
use crate::CoreLoop;

pub use glutin::window::{Icon, BadIcon};

pub(crate) struct Window {
    context: glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>,
    support_pbo: bool
}

impl Window {
    pub(crate) fn new(core_loop: &CoreLoop, title: &'static str, width: u32, height: u32, icon: Option<Icon>) -> RcCell<Self> {        
        let window_builder = glutin::window::WindowBuilder::new()
            .with_title(title)
            .with_inner_size(glutin::dpi::LogicalSize::new(width, height))
            .with_window_icon(icon);

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

    pub(crate) fn width(&self) -> u32 {
        self.internal_window().inner_size().width
    }

    pub(crate) fn height(&self) -> u32 {
        self.internal_window().inner_size().height
    }
}