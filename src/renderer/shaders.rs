pub(crate) static DISPLAY_SHADER_SRC_VERT: &str = "
#version 430 core
out vec2 texcoords;

void main() {
    vec2 vertices[3]=vec2[3](vec2(-1,-1), vec2(3,-1), vec2(-1, 3));
    gl_Position = vec4(vertices[gl_VertexID],0,1);
    texcoords = 0.5 * gl_Position.xy + vec2(0.5);
}
";

pub(crate) static DISPLAY_SHADER_SRC_FRAG: &str = "
#version 430 core
uniform sampler2D tex;
in vec2 texcoords;
out vec4 FragColor;

void main() {
    FragColor.rgb = texture(tex, texcoords).rgb;//pow(texture(tex, texcoords).rgb, vec3(2.2));
}
";