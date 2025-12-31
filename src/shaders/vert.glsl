#version 140
in vec3 position;
in vec2 tex_coords;
out vec2 v_tex_coords;
uniform mat4 matrix;
uniform vec3 cam_pos;
uniform vec3 cam_rot;
uniform float x;

uniform mat3 r_mat_x;
uniform mat3 r_mat_y;
uniform mat3 r_mat_z;

void main() {
    v_tex_coords = tex_coords;
    // position = position - cam_pos;
    // vec3 sex = position - cam_pos;
    // // sex = sex + vec3(0, -0.5, 0);
    // float distace = sqrt(pow(sex.x, 2) + pow(sex.y, 2) + pow(sex.z, 2));
    // sex = rotate3d(sex, vec3(0, x, 0), vec3(0, 0, -1));

    gl_Position = matrix * vec4(position, 1.0);

}