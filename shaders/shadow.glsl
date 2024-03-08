#version 450 core

layout (location = 0) in vec3 inPosition;
layout (location = 1) in vec3 inNormal;

layout (set = 0, binding = 0) uniform GlobalLightUniformLayout {
    mat4 mtxProjView;
    vec4 f4Direction;
    vec4 f4LightColor;
} uGlobalLight;

layout (set = 1, binding = 0) uniform ObjectUniformLayout {
    mat4 mtxWorld;
    vec4 f4Color;
} uEntity;

void main() {
    gl_Position = uGlobalLight.mtxProjView * uEntity.mtxWorld * vec4(inPosition, 1.0);
}
