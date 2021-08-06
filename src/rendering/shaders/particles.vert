#version 450 core

layout (location=0) in vec2 a_pos;
layout (location=1) in vec4 a_color;
layout (location=2) in float a_size;

layout (location=0) out vec4 v_color;
layout (location=1) out vec2 v_pos;
layout (location=2) out float v_size;

void main() {
    v_color = a_color;
    v_pos = a_pos;
    v_size = a_size;
    gl_Position = vec4(a_pos, 0, 1);
    gl_PointSize = a_size;
}