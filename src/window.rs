use crate::RcCell;
use crate::CoreLoop;

pub struct Window {
    context: glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>
}

impl Window {
    pub fn new(core_loop: &CoreLoop, title: &'static str, width: u32, height: u32) -> RcCell<Self> {        
        let window_builder = glutin::window::WindowBuilder::new()
            .with_title(title)
            .with_inner_size(glutin::dpi::LogicalSize::new(width, height));

        let context = glutin::ContextBuilder::new()
            .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (4, 3)))
            .build_windowed(window_builder, core_loop.winit_loop())
            .expect("Failed to build context.");

        let context = unsafe {
            context.make_current()
                .expect("Failed to make context current.")
        };

        RcCell::new(Window {
            context
        })
    }

    pub(crate) fn internal_context(&self) -> &glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window> {
        &self.context
    }

    pub(crate) fn internal_window(&self) -> &glutin::window::Window {
        self.context.window()
    }

    pub fn width(&self) -> u32 {
        self.internal_window().inner_size().width
    }

    pub fn height(&self) -> u32 {
        self.internal_window().inner_size().height
    }
}