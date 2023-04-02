»Vertex
#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 acol;
layout (location = 2) in vec2 atex;

uniform mat4 model;
uniform mat4 view;

out vec3 COL;
out vec2 TEX_C;

void main() {
    gl_Position = view * model * vec4(aPos.x, aPos.y, aPos.z, 1.0);
    COL = acol;
    TEX_C = atex;
}

»Fragment
#version 330 core

uniform vec4 col;
in vec3 COL;
in vec2 TEX_C;
uniform sampler2D ourTexture;

out vec4 FragColor;
void main() {
    FragColor = texture(ourTexture, TEX_C)+(col-vec4(COL, 1.0));
}