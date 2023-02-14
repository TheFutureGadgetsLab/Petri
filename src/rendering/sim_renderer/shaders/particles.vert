#version 450 core

layout (std140, binding=0)
uniform Camera {
    vec2 u_world_xbounds;
    vec2 u_world_ybounds;
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
    vec2 pos = vec2(
        (a_pos.x - u_world_xbounds.x) / (u_world_xbounds.y - u_world_xbounds.x) * 2.0 - 1.0,
        (a_pos.y - u_world_ybounds.x) / (u_world_ybounds.y - u_world_ybounds.x) * 2.0 - 1.0
    );
    
    v_color = a_color;
    v_pos   = pos;

    // Convert world space size to screen space size (in pixels)
    v_size = a_size / (u_world_xbounds.y - u_world_xbounds.x) * u_screen_width * 2.0;
    
    gl_Position = vec4(pos, 0, 1);
    gl_PointSize = v_size;
}