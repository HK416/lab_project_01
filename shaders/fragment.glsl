#version 450 core

layout (location = 0) in vec4 inColor;
layout (location = 1) in vec3 inNormalW;

layout (location = 0) out vec4 outColor;

layout (set = 0, binding = 0) uniform CameraUniformLayout {
    mat4 view;
    mat4 projection;
    vec4 position;
} u_Camera;

void main() {
    outColor = inColor;
}
