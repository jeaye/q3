/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: data/shaders/text.vert
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      The vertex shader program for rendering colored
      TTF text.
*/

#version 330 core

uniform mat4 proj; 
uniform mat4 world; 

layout (location = 0) in vec4 in_coord; 

out vec2 trans_coord; 

void main() 
{ 
  gl_Position = proj * vec4(in_coord.xy, -10.0, 1.0); 
  trans_coord = in_coord.zw; 
}

