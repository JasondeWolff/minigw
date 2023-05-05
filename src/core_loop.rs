use glutin::event::{Event, VirtualKeyCode, ElementState, KeyboardInput, WindowEvent, DeviceEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use cgmath::Vector2;

use crate::RcCell;
use crate::Window;
use crate::Input;
use crate::{Renderer, RenderTextureView};
use crate::gl_helpers::DebugUI;

pub struct CoreLoop {
    event_loop: EventLoop<()>
}

impl CoreLoop {
    pub fn new() -> Self {
        let event_loop = EventLoop::new();

        CoreLoop {
            event_loop
        }
    }

    pub(crate) fn winit_loop(&self) -> &EventLoop<()> {
        &self.event_loop
    }

    pub fn run<T: Copy, F: Fn(RcCell<Input>, RenderTextureView<T>, &mut DebugUI) + 'static>(self,
        core_update: F,
        rc_window: RcCell<Window>,
        rc_input: RcCell<Input>
    ) {
        let mut renderer = Renderer::new(&rc_window.as_ref(), gl::RGBA32F);

        self.event_loop.run(move |event, _, control_flow| {
            match event {
                | Event::WindowEvent { event, .. } => {
                    match event {
                        | WindowEvent::CloseRequested => {
                            *control_flow = ControlFlow::Exit
                        },
                        | WindowEvent::Resized(size) => {
                            renderer.resize(size.width, size.height);
                        },
                        | WindowEvent::KeyboardInput { input, .. } => {
                            match input {
                                | KeyboardInput { virtual_keycode, state, .. } => {
                                    match (virtual_keycode, state) {
                                        | (Some(VirtualKeyCode::Escape), ElementState::Pressed) => {
                                            *control_flow = ControlFlow::Exit
                                        },
                                        | (Some(virtual_keycode), state) => {
                                            rc_input.as_mut().set_key(virtual_keycode, state == ElementState::Pressed);
                                        },
                                        | _ => {}
                                    }
                                },
                            }
                        },
                        | WindowEvent::MouseInput { state, button, .. } => {
                            rc_input.as_mut().set_mouse_button(button, state == ElementState::Pressed, renderer.imgui());
                        },
                        | WindowEvent::CursorMoved { position, .. } => {
                            rc_input.as_mut().set_mouse_pos(Vector2::new(position.x as i32, position.y as i32), renderer.imgui());
                        }
                        | _ => {},
                    }
                },
                | Event::MainEventsCleared => {
                    rc_window.as_ref().internal_window().request_redraw();
                },
                | Event::RedrawRequested(_window_id) => {
                    core_update(
                        rc_input.clone(),
                        renderer.render_texture_view(),
                        renderer.imgui().new_frame()
                    );

                    rc_input.as_mut().update();

                    renderer.render(&rc_window.as_ref());
                },
                | Event::LoopDestroyed => {
                    
                },
                | Event::DeviceEvent { event, ..} => {
                    match event {
                        | DeviceEvent::MouseMotion { delta } => {
                            rc_input.as_mut().set_mouse_delta(Vector2::new(delta.0 as f32, delta.1 as f32));
                        },
                        | _ => {}
                    }
                },
                _ => (),
            }
        })
    }
}