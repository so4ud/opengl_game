#version 140
in vec3 position;
in vec2 tex_coords;
out vec2 v_tex_coords;
uniform mat4 matrix;
uniform vec3 cam_pos;
uniform vec3 cam_rot;

void main() {
    v_tex_coords = tex_coords;
    // position = position - cam_pos;
    vec3 sex = position - cam_pos;
    float distace = sqrt(pow(sex.x, 2) + pow(sex.y, 2) + pow(sex.z, 2));
    distace = sin(distace);

    gl_Position = matrix * vec4(sex, 1.0);

}