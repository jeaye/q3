/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: data/shaders/color.frag
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      The fragment shader program for rendering colored
      simplistic geometry.
*/

#version 330

in vec4 trans_color;
out vec4 out_color;

void main() 
{
  out_color = trans_color;
}

