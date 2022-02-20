#version 450 core

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
layout (location = 1) in vec3 in_color;
layout (location = 2) in float in_size;

layout (location = 0) out vec2 out_pos;
layout (location = 1) out vec3 out_color;
layout (location = 2) out float out_size;

void main() {
    vec4 in_pos = vec4(in_pos, 1.0);
    vec4 proj   = ViewProj * in_pos;

    vec4 v = ViewProj*(in_pos + vec4(in_size, in_size, 0.0, 0.0));
    float radius = width * length(v - proj);

    out_pos   = proj.xy;
    out_size  = radius;
    out_color = in_color;

    gl_Position  = proj; 
    gl_PointSize = radius;
}