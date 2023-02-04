#version 450 core

layout (location=0) in vec2 a_pos;
layout (location=1) in vec4 a_color;
layout (location=2) in float a_size;

layout (location=0) out vec4 v_color;
layout (location=1) out vec2 v_pos;
layout (location=2) out float v_size;

layout (std140, binding=0)
uniform Camera {
    vec2 u_translation;
    vec2 u_window_size;
    vec2 u_zoom;
};

void main() {
    vec2 pos = (2.0*a_pos + u_translation) / u_window_size * u_zoom;

    v_color = a_color;
    v_pos   = pos;
    v_size  = max(a_size * u_zoom.x, 1.0) * 2.0;
    
    gl_Position = vec4(pos, 0, 1);
    gl_PointSize = v_size;
}