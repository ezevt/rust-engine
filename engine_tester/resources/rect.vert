#version 420 core

layout (location = 0) in vec2 a_position;
layout (location = 1) in vec4 a_color;
layout (location = 2) in vec2 a_texture_coord;
layout (location = 3) in float a_texture_index;
layout (location = 4) in vec2 a_size;
layout (location = 5) in float a_corner_radius;
layout (location = 6) in float a_outline_thickness;
layout (location = 7) in vec4 a_outline_color;

uniform mat4 u_screen_matrix;

struct VertexOutput {
	vec4 color;
	vec2 texture_coord;
	float corner_radius;
	vec2 size;
	float outline_thickness;
	vec4 outline_color;
};

layout (location = 0) out VertexOutput v_output;
layout (location = 6) out flat float v_texture_index;


void main() {
	v_texture_index = a_texture_index;
	v_output.texture_coord = a_texture_coord;
	v_output.color = a_color;
	v_output.size = a_size;
	v_output.corner_radius = a_corner_radius;
	v_output.outline_thickness = a_outline_thickness;
	v_output.outline_color = a_outline_color;

	gl_Position = u_screen_matrix * vec4(a_position, 0.0, 1.0);
}
