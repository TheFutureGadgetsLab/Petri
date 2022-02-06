#version 450

layout(set = 0, binding = 0) uniform CameraViewProj {
    mat4 ViewProj;
    mat4 InverseView;
    mat4 Projection;
    vec3 WorldPosition;
    float near;
    float far;
    float width;
    float height;
};

layout (location = 0) in vec3 v_pos;
layout (location = 1) in vec4 v_color;
layout (location = 2) in float v_size;

layout (location=0) out vec4 out_color;

void main() {
    vec2 pcoord = gl_FragCoord.xy / vec2(width, height);
    vec2 vcoord = (vec2(v_pos.x, -v_pos.y) * 0.5) + 0.5;
    vec2 pdiff = 2.0 * ((pcoord - vcoord) / (vec2(v_size) / vec2(width, height)));
    float alpha = clamp(1.0 - dot(pdiff, pdiff), 0.0, 1.0 / v_size) * v_size;
    out_color = vec4(vec3(v_color.rgb), alpha);
}