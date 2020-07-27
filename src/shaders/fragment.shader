#version 410

uniform vec3 u_color;
in vec2 f_uv;
out vec4 color;

void main() {
    color = vec4(1.0, f_uv.x, f_uv.y, 1.0);
    color = vec4(u_color, 1.0);
}