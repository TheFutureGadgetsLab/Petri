#version 450

layout(location=0) out vec4 f_color;

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

layout (location = 0) in vec2 out_pos;
layout (location = 1) in vec4 out_color;
layout (location = 2) in float out_size;

void main() {
    f_color = vec4(0.3, 0.2, 0.1, 1.0);
}