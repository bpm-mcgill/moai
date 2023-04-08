»Vertex
#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNorm;
layout (location = 2) in vec3 aTex;

uniform mat4 model;
uniform mat4 view;

out vec3 NORM;
out vec3 TEX_C;

void main() {
    gl_Position = view * model * vec4(aPos.x, aPos.y, aPos.z, 1.0);
    NORM = aNorm;
    TEX_C = aTex;
}

»Fragment
#version 330 core

uniform vec4 col;
in vec3 NORM;
in vec3 TEX_C;
uniform sampler2D ourTexture;

out vec4 FragColor;
void main() {
    FragColor = texture(ourTexture, TEX_C.xy)+(col-vec4(NORM, 1.0));
}