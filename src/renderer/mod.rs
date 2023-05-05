use cgmath::Vector3;

use crate::Window;
use crate::gl_helpers::*;

mod shaders;
use shaders::*;
mod render_texture;
use render_texture::*;
pub mod render_texture_view;
pub use render_texture_view::*;

pub struct Renderer {
    imgui: ImGui,
    display_program: GLShaderProgram,
    display_vao: GLVAO,
    render_texture: RenderTexture,
    display_format: u32
}

impl Renderer {
    pub fn new(window: &Window, display_format: u32) -> Renderer {
        gl_init(window);

        let (width, height) = (window.width(), window.height());

        let mut imgui = ImGui::new();
        imgui.resize(width, height);
        gl_viewport(width, height);

        let vertex_shader = GLShader::new(GLShaderType::VERTEX, &DISPLAY_SHADER_SRC_VERT.to_owned());
        let fragment_shader = GLShader::new(GLShaderType::FRAGMENT, &DISPLAY_SHADER_SRC_FRAG.to_owned());
        let display_program = GLShaderProgram::new(&vertex_shader, &fragment_shader);
        let display_vao = GLVAO::new();

        let render_texture = RenderTexture::new(display_format, width, height);

        Renderer {
            imgui,
            display_program,
            render_texture,
            display_vao,
            display_format
        }
    }

    pub fn imgui(&mut self) -> &mut ImGui {
        &mut self.imgui
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.imgui.resize(width, height);

        let width = std::cmp::max(width, 1);
        let height = std::cmp::max(height, 1);

        gl_viewport(width, height);
        self.render_texture = RenderTexture::new(self.display_format, width, height);
    }

    pub fn render_texture_view<T: Copy>(&self) -> RenderTextureView<T> {
        self.render_texture.map()
    }

    pub fn render(&mut self, window: &Window) {
        gl_clear_color(Vector3::new(1.0, 0.0, 1.0));
        gl_clear();

        self.display_program.bind(); {
            self.render_texture.unmap();
            self.render_texture.bind(0);
            self.display_program.set_sampler_slot(&"tex".to_owned(), 0);

            self.display_vao.bind();
            gl_draw_arrays(gl::TRIANGLES, 0, 3);
        } self.display_program.unbind();

        self.imgui.render();
        window.internal_context().swap_buffers()
            .expect("Failed to swap buffers.");
    }
}