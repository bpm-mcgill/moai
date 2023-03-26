»Vertex
#version 330 core
layout (location = 0) in vec3 aPos;
void main() {
    gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
}

»Fragment
#version 330 core
uniform vec4 col;
out vec4 FragColor;
void main() {
    FragColor = col;
}