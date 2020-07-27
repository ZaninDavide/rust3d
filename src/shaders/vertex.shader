#version 410

in vec3 pos;
in vec2 uv;
in float material_id;

out vec2 f_uv;

void main() {
    gl_Position = vec4(pos, 1.0);

    // outs
    f_uv = uv;
}