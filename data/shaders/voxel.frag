/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: data/shaders/voxel.frag
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      Renders an instanced cube massive amounts of times.
*/

#version 330

in vec4 trans_color;
out vec4 out_color;

void main() 
{
  out_color = trans_color;
}

