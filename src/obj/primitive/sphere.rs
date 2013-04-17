/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: obj/primitive/sphere.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      An arbitrarily subdivided Icosahedron.
*/

use math::Vec3f;

#[path = "../../gl/mod.rs"]
mod gl;
#[path = "../../gl/util.rs"]
mod util;
#[macro_escape]
#[path = "../../gl/check.rs"]
mod check;

pub struct Sphere
{
  radius: f32,
  vbo: gl::GLuint,
  verts: ~[Vec3f],
}

impl Sphere
{
  pub fn new(new_radius: f32, new_subdivides: i32) -> Sphere
  {
    /* Magic numbers place all verts 1.0 from center. */
    let magic_x = 0.525731112119133606f32;
    let magic_z = 0.850650808352039932f32;

    let mut sphere = Sphere
    {
      radius: new_radius,
      vbo: 0,
      verts: ~[],
    };

    let root_verts: [Vec3f, ..12] =
    [
      Vec3f::new(-magic_x, 0.0, magic_z),
      Vec3f::new(magic_x, 0.0, magic_z),
      Vec3f::new(-magic_x, 0.0, -magic_z),
      Vec3f::new(magic_x, 0.0, -magic_z),

      Vec3f::new(0.0, magic_z, magic_x),
      Vec3f::new(0.0, magic_z, -magic_x),
      Vec3f::new(0.0, -magic_z, magic_x),
      Vec3f::new(0.0, -magic_z, -magic_x),

      Vec3f::new(magic_z, magic_x, 0.0),
      Vec3f::new(-magic_z, magic_x, 0.0),
      Vec3f::new(magic_z, -magic_x, 0.0),
      Vec3f::new(-magic_z, -magic_x, 0.0),
    ];

    //sphere.verts =
    //~[
    //  /* http://www.glprogramming.com/red/chapter02.html#name8 */
    //  root_verts[0], root_verts[4], root_verts[1], root_verts[0], root_verts[9], root_verts[4], 
    //  root_verts[9], root_verts[5], root_verts[4], root_verts[4], root_verts[5], root_verts[8], 
    //  root_verts[4], root_verts[8], root_verts[1], root_verts[8], root_verts[10], root_verts[1], 
    //  root_verts[8], root_verts[3], root_verts[10], root_verts[5], root_verts[3], root_verts[8], 
    //  root_verts[5], root_verts[2], root_verts[3], root_verts[2], root_verts[7], root_verts[3],
    //  root_verts[7], root_verts[10], root_verts[3], root_verts[7], root_verts[6], root_verts[10], 
    //  root_verts[7], root_verts[11], root_verts[6], root_verts[11], root_verts[0], root_verts[6], 
    //  root_verts[0], root_verts[1], root_verts[6], root_verts[6], root_verts[1], root_verts[10], 
    //  root_verts[9], root_verts[0], root_verts[11], root_verts[9], root_verts[11], root_verts[2], 
    //  root_verts[9], root_verts[2], root_verts[5], root_verts[7], root_verts[2], root_verts[11],
    //];

    sphere.verts =
    ~[
      /* http://www.glprogramming.com/red/chapter02.html#name8 */
      root_verts[0], root_verts[0], root_verts[4], root_verts[4], root_verts[1], root_verts[1], root_verts[0], root_verts[0], root_verts[9], root_verts[9],   root_verts[4],  root_verts[4],  
      root_verts[9], root_verts[9], root_verts[5], root_verts[5], root_verts[4], root_verts[4], root_verts[4], root_verts[4], root_verts[5], root_verts[5],   root_verts[8],  root_verts[8],  
      root_verts[4], root_verts[4], root_verts[8], root_verts[8], root_verts[1], root_verts[1], root_verts[8], root_verts[8], root_verts[10],root_verts[10],  root_verts[1],  root_verts[1], 
      root_verts[8], root_verts[8], root_verts[3], root_verts[3], root_verts[10],root_verts[10], root_verts[5], root_verts[5], root_verts[3], root_verts[3],  root_verts[8],  root_verts[8], 
      root_verts[5], root_verts[5], root_verts[2], root_verts[2], root_verts[3], root_verts[3], root_verts[2], root_verts[2], root_verts[7], root_verts[7],   root_verts[3],  root_verts[3],  
      root_verts[7], root_verts[7], root_verts[10],root_verts[10], root_verts[3], root_verts[3], root_verts[7], root_verts[7], root_verts[6], root_verts[6],  root_verts[10], root_verts[10],
      root_verts[7], root_verts[7], root_verts[11],root_verts[11], root_verts[6], root_verts[6], root_verts[11], root_verts[11], root_verts[0], root_verts[0], root_verts[6],   root_verts[6],
      root_verts[0], root_verts[0], root_verts[1], root_verts[1], root_verts[6], root_verts[6], root_verts[6], root_verts[6], root_verts[1], root_verts[1],    root_verts[10], root_verts[10],  
      root_verts[9], root_verts[9], root_verts[0], root_verts[0], root_verts[11],root_verts[11], root_verts[9], root_verts[9], root_verts[11], root_verts[11], root_verts[2],  root_verts[2],
      root_verts[9], root_verts[9], root_verts[2], root_verts[2], root_verts[5], root_verts[5], root_verts[7], root_verts[7], root_verts[2], root_verts[2],    root_verts[11], root_verts[11],  
    ];

    sphere.vbo = check!(gl::gen_buffers(1))[0];
    check!(gl::bind_buffer(gl::ARRAY_BUFFER, sphere.vbo));
    check!(gl::buffer_data(gl::ARRAY_BUFFER, sphere.verts, gl::STATIC_DRAW));

    sphere
  }

  pub fn draw(&self)
  {
    check!(gl::bind_buffer(gl::ARRAY_BUFFER, self.vbo));
    check!(gl::vertex_attrib_pointer_f32(0, 3, false, (sys::size_of::<Vec3f>() * 2) as i32, 0));
    check!(gl::vertex_attrib_pointer_f32(1, 3, false, (sys::size_of::<Vec3f>() * 2) as i32, sys::size_of::<Vec3f>() as u32));
    check!(gl::enable_vertex_attrib_array(0));
    check!(gl::enable_vertex_attrib_array(1));
    check!(gl::draw_arrays(gl::TRIANGLES, 0, (self.verts.len() as i32) / 2));
    check!(gl::disable_vertex_attrib_array(0));
    check!(gl::disable_vertex_attrib_array(1));
  }
}

