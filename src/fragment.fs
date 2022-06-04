#version 330 core
out vec4 FragColor;
in vec3 vert_color;

void main() {
    FragColor = vec4(vert_color, 1.0f);
}
