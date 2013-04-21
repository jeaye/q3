/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: data/shaders/color.vert
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      The vertex shader program for rendering colored
      simplistic geometry.
*/

#version 330

uniform mat4x4 proj;
uniform mat4x4 world;

layout (location = 0) in vec4 in_position;
layout (location = 1) in vec4 in_color;

out vec4 trans_color;

void main()
{
  gl_Position = proj * world * in_position;
  trans_color = in_color;
}

