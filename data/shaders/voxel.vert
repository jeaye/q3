/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: data/shaders/voxel.vert
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      Renders an instanced cube massive amounts of times.
*/

#version 330

uniform mat4x4 proj;
uniform mat4x4 world;
uniform float voxel_size = 1.0f;
uniform samplerBuffer offsets;

/* Per vertex. */
layout (location = 0) in vec4 in_position;

/* Per instance */
// nothing

out vec4 trans_color;

void main()
{
  vec4 tex_offset = texelFetch(offsets, gl_InstanceID * 2);
  vec4 tex_color = texelFetch(offsets, (gl_InstanceID * 2) + 1);

  vec4 position = in_position + (tex_offset * voxel_size);
  position.w = 1.0f;

  trans_color = tex_color;
  gl_Position = proj * world * position;
}

