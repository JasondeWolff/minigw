use cgmath::Vector3;

use crate::RcCell;
use crate::Window;
use crate::gl_helpers::*;

mod shaders;
use shaders::*;
pub mod render_texture;
pub use render_texture::*;
pub mod render_texture_type;
pub use render_texture_type::*;

pub(crate) struct Renderer<T: RenderTextureType> {
    imgui: ImGui,
    display_program: GLShaderProgram,
    display_vao: GLVAO,
    render_texture: RcCell<RenderTexture<T>>
}

impl<T: RenderTextureType> Renderer<T> {
    pub(crate) fn new(window: &Window) -> Renderer<T> {
        gl_init(window);

        let (width, height) = (window.get_width(), window.get_height());

        let mut imgui = ImGui::new();
        imgui.resize(width, height);
        gl_viewport(width, height);

        let vertex_shader = GLShader::new(GLShaderType::Vertex, DISPLAY_SHADER_SRC_VERT);
        let fragment_shader = GLShader::new(GLShaderType::Fragment, DISPLAY_SHADER_SRC_FRAG);
        let display_program = GLShaderProgram::new(&vertex_shader, &fragment_shader);
        let display_vao = GLVAO::new();

        let use_pbo = window.support_pbo();
        let render_texture = RcCell::new(RenderTexture::new(width, height, use_pbo, RenderTextureResizing::Resizable));

        Renderer {
            imgui,
            display_program,
            render_texture,
            display_vao
        }
    }

    pub(crate) fn imgui(&mut self) -> &mut ImGui {
        &mut self.imgui
    }

    pub(crate) fn resize(&mut self, width: u32, height: u32) {
        self.imgui.resize(width, height);

        let width = std::cmp::max(width, 1);
        let height = std::cmp::max(height, 1);

        gl_viewport(width, height);
        self.render_texture.as_mut().resize(width, height);
    }

    pub(crate) fn render_texture(&mut self) -> RcCell<RenderTexture<T>> {
        self.render_texture.clone()
    }

    pub(crate) fn render(&mut self, window: &Window) {
        gl_clear_color(Vector3::new(1.0, 0.0, 1.0));
        gl_clear();

        self.display_program.bind(); {
            let mut render_texture = self.render_texture.as_mut();
            render_texture.write();
            render_texture.bind(0);
            self.display_program.set_sampler_slot(&"tex".to_owned(), 0);

            self.display_vao.bind();
            gl_draw_arrays(gl::TRIANGLES, 0, 3);
        } self.display_program.unbind();

        self.imgui.render();
        window.internal_context().swap_buffers()
            .expect("Failed to swap buffers.");
    }
}