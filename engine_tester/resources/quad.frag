#version 420 core

layout (location = 0) out vec4 o_color;

in vec4 v_color;
in vec2 v_texture_coord;
in float v_texture_index;

layout (binding = 0) uniform sampler2D u_textures[32];

void main() {
	if (v_color.a == 0.0) discard;

	vec4 texture_color = v_color;

	texture_color *= texture(u_textures[int(v_texture_index)], v_texture_coord);

	
	o_color = texture_color;
}