#version 450

layout (location = 0) in vec3 inPos;
layout (location = 1) in vec3 inColor;

layout (location = 0) out vec3 outColor;

layout (set = 0, binding = 0) uniform UBO
{
  mat4 modelViewProj;
  vec4 primaryColor;
  vec4 accentColor;
};

void main()
{
    outColor = inColor;
    gl_Position = modelViewProj * vec4(inPos, 1.0);
}