#version 450 core

layout (std140, binding=0)
uniform Camera {
    vec2 u_world_size;
    vec2 u_world_ll;
    float u_screen_height;
    float u_screen_width;
};

layout (location=0) in vec2 a_pos;
layout (location=1) in vec4 a_color;
layout (location=2) in float a_size;

layout (location=0) out vec4 v_color;
layout (location=1) out vec2 v_pos;
layout (location=2) out float v_size;

void main() {
    // Convert from world space to screen space [-1, 1]
    vec2 pos = (a_pos - u_world_ll) / u_world_size * 2.0 - 1.0;
    
    v_color = a_color;
    v_pos   = pos;

    // Convert world space size to screen space size (in pixels)
    v_size = a_size / u_world_size.x * u_screen_width * 2.0;
    
    gl_Position = vec4(pos, 0, 1);
    gl_PointSize = v_size;
}