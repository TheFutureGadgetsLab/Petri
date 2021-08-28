#version 450 core

layout (location=0) in vec2 a_pos;
layout (location=1) in vec4 a_color;
layout (location=2) in float a_size;

layout (location=0) out vec4 v_color;
layout (location=1) out vec2 v_pos;
layout (location=2) out float v_size;

layout (std140, binding=1)
uniform Camera {
    vec2 u_scale;
    vec2 u_translation;
};

void main() {
    vec2 size = a_size * u_scale;
    v_pos = u_translation + a_pos * u_scale;
    gl_Position = vec4(v_pos, 0, 0);
    gl_PointSize = size.x;
    v_color = a_color;
    v_size = size.x;
}