/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: obj/md5/model_renderer.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      Renders an MD5 model (which contains
      multiple meshes).
*/

use gl2 = opengles::gl2;
use gl;
use super::{ Model, Mesh_Renderer };

#[path = "../../gl/check.rs"]
mod check;

struct Model_Renderer<'self>
{
  model: &'self Model,
  mesh_renderers: ~[Mesh_Renderer<'self>],

  shader: @mut gl::Shader, /* TODO: shared */
  proj_loc: gl2::GLint,
  world_loc: gl2::GLint,
}

impl<'self> Model_Renderer<'self>
{
  pub fn new(m: &'self Model) -> Model_Renderer<'self>
  {
    let mut mr = Model_Renderer
    {
      model: m,
      mesh_renderers: ~[],

      shader: gl::Shader_Builder::new_with_files("data/shaders/md5.vert", "data/shaders/md5.frag"),
      proj_loc: 0,
      world_loc: 0,
    };

    mr.shader.bind();
    mr.proj_loc = mr.shader.get_uniform_location("proj");
    mr.world_loc = mr.shader.get_uniform_location("world");

    for x in mr.model.meshes.iter()
    { mr.mesh_renderers.push(Mesh_Renderer::new(x, mr.shader)); }

    mr
  }

  pub fn render(&mut self)
  {
    check!(gl2::front_face(gl2::CW));

    do gl::Camera::get_active() |camera|
    {
      self.shader.bind();
      self.shader.update_uniform_mat(self.proj_loc, &camera.projection);
      self.shader.update_uniform_mat(self.world_loc, &camera.view);
    }

    for x in self.mesh_renderers.iter()
    { x.render(); }

    check!(gl2::front_face(gl2::CCW));
  }
}

