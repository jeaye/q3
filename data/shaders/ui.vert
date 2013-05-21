/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: data/shaders/ui.vert
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      The vertex shader program for rendering textured
      UI elements.
*/

#version 330

uniform mat4 proj; 
uniform mat4 world; 
uniform float alpha = 1.0f;
uniform mat4 tex_world;

layout (location = 0) in vec4 in_coord; 

out vec2 trans_coord; 
out float trans_alpha;

void main() 
{ 
  gl_Position = proj * world * vec4(in_coord.xy, -10.0f, 1.0f); 
  trans_coord = (tex_world * vec4(in_coord.zw, 0.0f, 1.0f)).xy; 
  trans_alpha = alpha;
}

