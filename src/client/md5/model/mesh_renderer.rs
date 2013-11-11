/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: client/md5/model/mesh_renderer.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      Renders an MD5 mesh.
*/

use std::{ mem, ptr };
use gl, gl::types::*;
use gfx;
use math;
use super::Mesh;

#[macro_escape]
#[path = "../../gfx/check.rs"]
mod check;

struct Mesh_Renderer<'self>
{
  priv mesh: &'self Mesh,

  priv vao: gfx::VAO,
  priv position_vbo: gfx::VBO,
  priv tex_vbo: gfx::VBO,
  priv ibo: gfx::VBO,
  priv tex0_loc: GLint,

  priv shader: @mut gfx::Shader, 
  priv texture: Option<gfx::Texture>,
}

impl<'self> Mesh_Renderer<'self>
{
  pub fn new(m: &'self Mesh, sh: @mut gfx::Shader) -> Mesh_Renderer<'self>
  {
    let mut mr = Mesh_Renderer
    {
      mesh: m,

      vao: gfx::VAO::zero(),
      position_vbo: gfx::VBO::zero(),
      tex_vbo: gfx::VBO::zero(),
      ibo: gfx::VBO::zero(),
      tex0_loc: 0,

      shader: sh,
      texture: None,
    };

    if m.texture.len() > 0
    { mr.texture = Some(gfx::Texture::new(gl::TEXTURE_2D, m.texture)); }

    mr.shader.bind();
    mr.tex0_loc = mr.shader.get_uniform_location("tex0");
    mr.shader.update_uniform_i32(mr.tex0_loc, 0);

    mr.upload();

    mr
  }

  fn upload(&mut self)
  {
    self.vao = gfx::VAO::new();

    self.position_vbo = gfx::VBO::new(gl::ARRAY_BUFFER);
    self.tex_vbo = gfx::VBO::new(gl::ARRAY_BUFFER);
    self.ibo = gfx::VBO::new(gl::ELEMENT_ARRAY_BUFFER);

    /* Upload data. */
    self.position_vbo.bind();
    self.position_vbo.buffer_data(self.mesh.positions, gl::DYNAMIC_DRAW);

    self.tex_vbo.bind();
    self.tex_vbo.buffer_data(self.mesh.tex_coords, gl::DYNAMIC_DRAW);

    self.ibo.bind();
    self.ibo.buffer_data(self.mesh.indices, gl::STATIC_DRAW);

    /* Setup vertex attribs. */
    self.vao.bind();

    self.vao.enable_vertex_attrib_array(0);
    self.position_vbo.bind();
    self.vao.vertex_attrib_pointer_f32(0, 3, false, mem::size_of::<math::Vec3f>() as i32, ptr::null());

    self.vao.enable_vertex_attrib_array(1);
    self.tex_vbo.bind();
    self.vao.vertex_attrib_pointer_f32(1, 2, false, mem::size_of::<math::Vec2f>() as i32, ptr::null());

  }

  pub fn update(&mut self, _dt: f32)
  {
    self.position_vbo.bind();
    self.position_vbo.buffer_sub_data(0, self.mesh.positions);
  }

  pub fn render(&self)
  {
    match self.texture
    {
      Some(ref tex) => { tex.bind(gl::TEXTURE_2D); },
      None => { }
    }

    self.vao.bind();

    self.ibo.bind();
    check_unsafe!(gl::DrawElements(gl::TRIANGLES, self.mesh.indices.len() as GLsizei,
                                   gl::UNSIGNED_INT, ptr::null()));

    self.ibo.unbind();
    self.vao.unbind();
  }
}
