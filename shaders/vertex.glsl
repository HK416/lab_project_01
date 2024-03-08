#version 450 core

layout (location = 0) in vec3 inPosition;
layout (location = 1) in vec3 inNormal;

layout (location = 0) out vec4 outColor;
layout (location = 1) out vec3 outNormalW;
layout (location = 2) out vec4 outLightSpaceFragPosition;

layout (set = 0, binding = 0) uniform CameraUniformLayout {
    mat4 mtxView;
    mat4 mtxProjection;
    vec4 f4Position;
} uCamera;

layout (set = 1, binding = 0) uniform ObjectUniformLayout {
    mat4 mtxWorld;
    vec4 f4Color;
} uEntity;

layout (set = 2, binding = 0) uniform GlobalLightUniformLayout {
    mat4 mtxProjView;
    vec4 f4Direction;
    vec4 f4LightColor;
} uGlobalLight;

void main() {
    outColor = uEntity.f4Color;
    outNormalW = mat3(uEntity.mtxWorld) * inNormal;
    outLightSpaceFragPosition = uGlobalLight.mtxProjView * uEntity.mtxWorld * vec4(inPosition, 1.0);
    gl_Position = uCamera.mtxProjection * uCamera.mtxView * uEntity.mtxWorld * vec4(inPosition, 1.0);
}
