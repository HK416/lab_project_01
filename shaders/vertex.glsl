#version 450 core

layout (location = 0) in vec3 inPosition;
layout (location = 1) in vec3 inNormal;

layout (location = 0) out vec4 outColor;
layout (location = 1) out vec3 outNormalW;

layout (set = 0, binding = 0) uniform CameraUniformLayout {
    mat4 view;
    mat4 projection;
    vec4 position;
} uCamera;

layout (set = 1, binding = 0) uniform ObjectUniformLayout {
    mat4 world;
    vec4 color;
} uEntity;

void main() {
    outColor = uEntity.color;
    outNormalW = mat3(uEntity.world) * inNormal;
    gl_Position = uCamera.projection * uCamera.view * uEntity.world * vec4(inPosition, 1.0);
}
