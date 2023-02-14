#version 450 core

layout (std140, binding=0)
uniform Camera {
    vec2 u_world_xbounds;
    vec2 u_world_ybounds;
    float u_screen_height;
    float u_screen_width;
};

layout (location=0) in vec4 v_color;
layout (location=1) in vec2 v_pos;
layout (location=2) in float v_size;

layout (location=0) out vec4 out_color;

// Fragment shader
void main() {
    if (length(gl_PointCoord.xy - 0.5) > 0.5) {
        discard;
    }
    out_color = v_color;  
}