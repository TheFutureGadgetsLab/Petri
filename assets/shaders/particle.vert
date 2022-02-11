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
    vec4 proj = ViewProj * vec4(in_pos, 1.0);

    out_pos   = proj.xy;
    out_size  = in_size;
    out_color = in_color;

    // This is definitely wrong
    // If you rotate the camera it will oscillate the scale
    float m00 = ViewProj[0][0];
    float m01 = ViewProj[1][1];
    float scalingFactor = 10000.0 * sqrt(m00 * m00 + m01 * m01);

    gl_Position  = proj; 
    gl_PointSize = in_size * scalingFactor;
}