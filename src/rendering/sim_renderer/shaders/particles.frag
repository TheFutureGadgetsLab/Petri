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

float edgeRamp(float x, float f, float d) {
    return clamp(1.0 - f * (x - 1.0 + d + 1.0 / f), 0.0, 1.0);
}

void main() {
    float v = length(gl_PointCoord - 0.5);
    float a = edgeRamp(v * 2.0, 35.0, 0.05);
    if (a < 1e-9) {
        discard;
    } else {
        out_color = vec4(v_color.rgb, a);
    }
}