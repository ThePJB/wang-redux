layout (location = 0) in vec3 in_pos;
layout (location = 1) in vec3 in_colour;
layout (location = 2) in vec2 in_uv;

// uniform mat4 projection;
const mat4 projection = mat4(
    2, 0, 0, 0,
    0, -2, 0, 0,
    0, 0, -0.001, 0,
    -1, 1, 1, 1
);
// mat4 projection = [
//     2, 0, 0, -1,
//     0, -2, 0, 1,
//     0, 0, -0.001, 1,
//     0, 0, 0, 1,
// ];
// const mat4 projection = mat4(
//     1, 0, 0, 0,
//     0, 1, 0, 0,
//     0, 0, 1, 0,
//     0, 0, 0, 1
// );

out vec3 vert_colour;
out vec2 uv;

void main() {
    vert_colour = in_colour;
    uv = in_uv;
    gl_Position = projection * vec4(in_pos, 1.0);
}