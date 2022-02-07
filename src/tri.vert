#version 450

const vec2 positions[3] = vec2[3](
    vec2(0.0, 0.5),
    vec2(-0.5, -0.5),
    vec2(0.5, -0.5)
);

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

layout (location = 0) in vec3 in_pos;
layout (location = 1) in vec4 in_color;
layout (location = 2) in float in_size;

layout (location = 0) out vec2 out_pos;
layout (location = 1) out vec4 out_color;
layout (location = 2) out float out_size;

void main() {
    gl_Position = vec4(positions[gl_VertexIndex], 0.0, 1.0);
}

