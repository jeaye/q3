/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: data/shaders/md5.vert
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      The vertex shader for MD5 models.
*/

#version 330

uniform mat4 proj; 
uniform mat4 world; 

layout (location = 0) in vec4 in_position; 
layout (location = 1) in vec2 in_coord;

out vec2 trans_coord; 

void main() 
{ 
  gl_Position = proj * world * in_position;
  trans_coord = in_coord;
}

