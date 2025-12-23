#version 140

in vec3 position;
in vec3 color;      // our new attribute
out vec3 vertex_color;

// uniform mat4 matrix;

void main() {
    vertex_color = color; // we need to set the value of each `out` variable.
    gl_Position = /* matrix */ vec4(position, 1.0);
}