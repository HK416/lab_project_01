#version 450 core

layout (location = 0) in vec4 inColor;
layout (location = 1) in vec3 inNormalW;
layout (location = 2) in vec4 inLightSpaceFragPosition;

layout (location = 0) out vec4 outFragColor;

layout (set = 0, binding = 0) uniform CameraUniformLayout {
    mat4 view;
    mat4 projection;
    vec4 position;
} uCamera;

layout (set = 2, binding = 0) uniform GlobalLightUniformLayout {
    mat4 mtxProjView;
    vec4 f4Direction;
    vec4 f4LightColor;
} uGlobalLight;

layout (set = 3, binding = 0) uniform texture2D uShadowMap;
layout (set = 3, binding = 1) uniform sampler uSampler;

float calculateShadow(vec4 f4LightSpaceFragPosition) {
    if (f4LightSpaceFragPosition.w <= 0.0) {
        return 1.0;
    }

    float fCurrentDepth = f4LightSpaceFragPosition.z / f4LightSpaceFragPosition.w;
    vec2 f2ProjCoords = f4LightSpaceFragPosition.xy / f4LightSpaceFragPosition.w;
    f2ProjCoords = f2ProjCoords * vec2(0.5, -0.5) + 0.5;
    return texture(sampler2DShadow(uShadowMap, uSampler), vec3(f2ProjCoords, fCurrentDepth));
}

void main() {
    float fShadow = calculateShadow(inLightSpaceFragPosition);
    outFragColor = inColor * 0.2 + (inColor * fShadow);
}
