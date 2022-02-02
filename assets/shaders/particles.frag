#version 450 core

layout (std140, binding=0)
uniform Camera {
    vec2 u_translation;
    vec2 u_window_size;
    vec2 u_zoom;
};

layout (location=0) in vec4 v_color;
layout (location=1) in vec2 v_pos;
layout (location=2) in float v_size;

layout (location=0) out vec4 out_color;

void main() {
    vec2 pcoord = gl_FragCoord.xy / u_window_size;
    vec2 vcoord = (vec2(v_pos.x, -v_pos.y) * 0.5) + 0.5;
    vec2 pdiff = 2.0 * ((pcoord - vcoord) / (vec2(v_size) / u_window_size));
    float alpha = clamp(1.0 - dot(pdiff, pdiff), 0.0, 1.0 / v_size) * v_size;
    out_color = vec4(vec3(v_color.rgb), alpha);
}