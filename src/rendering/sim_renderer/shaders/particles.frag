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
    vec2 pcoord = gl_FragCoord.xy / vec2(u_screen_width, u_screen_height);
    vec2 vcoord = (v_pos + 1.0) * 0.5;

    float dist = distance(pcoord, vcoord);
    if (dist > v_size / u_screen_width) {
        discard;
    }
    out_color = vec4(v_color.rgb, 1.0);  
}