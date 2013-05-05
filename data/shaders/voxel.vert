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

layout (location = 0) in vec3 in_position;
layout (location = 1) in vec3 in_offset;

out vec4 trans_color;

void main()
{
  vec4 position = vec4(in_position + (in_offset * voxel_size), 1.0f);
  gl_Position = proj * world * position;

  trans_color = position;
}

