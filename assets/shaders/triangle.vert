#version 330 core

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aColor;
uniform vec2 offset;

out vec3 vertexColor;


void main()
{
    gl_Position = vec4(aPos.xy + offset, aPos.z, 1.0);
    vertexColor = aColor;
}