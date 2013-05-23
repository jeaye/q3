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

/* Per vertex. */
layout (location = 0) in vec4 in_position;

/* Per instance */
layout (location = 1) in vec4 in_offset;
layout (location = 2) in vec4 in_color;

out vec4 trans_color;
void main()
{
  vec4 position = in_position + (in_offset * voxel_size);
  position.w = 1.0f;
  gl_Position = proj * world * position;

  trans_color = in_color;
}

