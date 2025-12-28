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

vec3 rotate3d(vec3 vertex, vec3 angles, vec3 center) {
    vertex = vertex - center;

    // angles = radians(angles);
    vec3 vec_1 = vec3(1, 0, 0);
    vec3 vec_2 = vec3(0, cos(angles.x), -sin(angles.x));
    vec3 vec_3 = vec3(0, sin(angles.x), cos(angles.x));
    mat3 mat_x = mat3(vec_1, vec_2, vec_3);

    vec_1 = vec3(cos(angles.y), 0, sin(angles.y));
    vec_2 = vec3(0, 1, -sin(angles.y));
    vec_3 = vec3(-sin(angles.y), 0, cos(angles.y));
    mat3 mat_y = mat3(vec_1, vec_2, vec_3);

    vec_1 = vec3(cos(angles.z), -sin(angles.z), 0);
    vec_2 = vec3(sin(angles.z), cos(angles.z), 0);
    vec_3 = vec3(0, 0, 1);
    mat3 mat_z = mat3(vec_1, vec_2, vec_3);

    vertex = mat_x * mat_y * mat_z * vertex;
    vertex += center;

    return vertex;
}

void main() {
    v_tex_coords = tex_coords;
    // position = position - cam_pos;
    // vec3 sex = position - cam_pos;
    // // sex = sex + vec3(0, -0.5, 0);
    // float distace = sqrt(pow(sex.x, 2) + pow(sex.y, 2) + pow(sex.z, 2));
    // sex = rotate3d(sex, vec3(0, x, 0), vec3(0, 0, -1));

    gl_Position = matrix * vec4(position, 1.0);

}