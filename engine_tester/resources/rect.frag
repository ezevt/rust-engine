#version 420 core

layout (location = 0) out vec4 o_color;

struct VertexOutput {
	vec4 color;
	vec2 texture_coord;
	float corner_radius;
	vec2 size;
	float outline_thickness;
	vec4 outline_color;
};

layout (location = 0) in VertexOutput v_input;
layout (location = 6) in flat float v_texture_index;

layout (binding = 0) uniform sampler2D u_textures[32];

void main() {
	if (v_input.color.a == 0.0) discard;

	vec4 texture_color = v_input.color;
	texture_color *= texture(u_textures[int(v_texture_index)], v_input.texture_coord);

	vec2 d = abs((v_input.texture_coord - 0.5) * v_input.size) - (v_input.size * 0.5 - v_input.corner_radius);
	float dist = length(max(d, 0.0)) + min(max(d.x, d.y), 0.0) - v_input.corner_radius;
	
	float outlineA = smoothstep(-v_input.outline_thickness, -v_input.outline_thickness + 2.0, dist);
	texture_color = mix(texture_color, v_input.outline_color, outlineA);

	texture_color.a *= smoothstep(1.0, -1.0, dist);

	o_color = texture_color;
}