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
  vao: gl::GLuint,
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
      vao: 0,
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

    let verts: &[Vec3f] = 
    &[
      /* http://www.glprogramming.com/red/chapter02.html#name8 */
      root_verts[0], root_verts[0], root_verts[4], root_verts[0], root_verts[1], root_verts[0], root_verts[0], root_verts[1], root_verts[9], root_verts[1],   root_verts[4],  root_verts[1],  
      root_verts[9], root_verts[2], root_verts[5], root_verts[2], root_verts[4], root_verts[2], root_verts[4], root_verts[3], root_verts[5], root_verts[3],   root_verts[8],  root_verts[3],  
      root_verts[4], root_verts[4], root_verts[8], root_verts[4], root_verts[1], root_verts[4], root_verts[8], root_verts[5], root_verts[10],root_verts[5],  root_verts[1],  root_verts[5], 
      root_verts[8], root_verts[6], root_verts[3], root_verts[6], root_verts[10],root_verts[6], root_verts[5], root_verts[7], root_verts[3], root_verts[7],  root_verts[8],  root_verts[7], 
      root_verts[5], root_verts[8], root_verts[2], root_verts[8], root_verts[3], root_verts[8], root_verts[2], root_verts[9], root_verts[7], root_verts[9],   root_verts[3],  root_verts[9],  
      root_verts[7], root_verts[1], root_verts[10],root_verts[1], root_verts[3], root_verts[1], root_verts[7], root_verts[2], root_verts[6], root_verts[2],  root_verts[10], root_verts[2],
      root_verts[7], root_verts[3], root_verts[11],root_verts[3], root_verts[6], root_verts[3], root_verts[11],root_verts[4], root_verts[0], root_verts[4], root_verts[6],   root_verts[4],
      root_verts[0], root_verts[5], root_verts[1], root_verts[5], root_verts[6], root_verts[5], root_verts[6], root_verts[6], root_verts[1], root_verts[6],    root_verts[10], root_verts[6],  
      root_verts[9], root_verts[7], root_verts[0], root_verts[7], root_verts[11],root_verts[7], root_verts[9], root_verts[8], root_verts[11],root_verts[8], root_verts[2],  root_verts[8],
      root_verts[9], root_verts[9], root_verts[2], root_verts[9], root_verts[5], root_verts[9], root_verts[7], root_verts[0], root_verts[2], root_verts[0],    root_verts[11], root_verts[0],  
    ];
    for uint::range_step(0, verts.len(), 6) |x|
    { sphere.subdivide(verts[x], verts[x + 2], verts[x + 4], new_subdivides); }

    sphere.vao = check!(gl::gen_vertex_arrays(1))[0]; /* TODO: Check these. */
    sphere.vbo = check!(gl::gen_buffers(1))[0];
    check!(gl::bind_vertex_array(sphere.vao));
    check!(gl::bind_buffer(gl::ARRAY_BUFFER, sphere.vbo));
    check!(gl::buffer_data(gl::ARRAY_BUFFER, sphere.verts, gl::STATIC_DRAW));

    sphere
  }

  /* Recursive subdivide for a given triangle. */
  priv fn subdivide(&mut self, v1: Vec3f, v2: Vec3f, v3: Vec3f, depth: i32)
  {
    let mut v12 = Vec3f::zero(), v23 = Vec3f::zero(), v31 = Vec3f::zero();

    if depth == 0
    {
      self.verts.push(v1);
      self.verts.push(v1);

      self.verts.push(v2);
      self.verts.push(v1);

      self.verts.push(v3);
      self.verts.push(v1);
      return;
    }

    v12.x = v1.x + v2.x; /* TODO: Lack of clean mutable indexing. */
    v23.x = v2.x + v3.x;
    v31.x = v3.x + v1.x;

    v12.y = v1.y + v2.y;
    v23.y = v2.y + v3.y;
    v31.y = v3.y + v1.y;

    v12.z = v1.z + v2.z;
    v23.z = v2.z + v3.z;
    v31.z = v3.z + v1.z;

    v12.normalize();
    v23.normalize();
    v31.normalize();
    self.subdivide(v1, v12, v31, depth - 1);
    self.subdivide(v2, v23, v12, depth - 1);
    self.subdivide(v3, v31, v23, depth - 1);
    self.subdivide(v12, v23, v31, depth - 1);
  }

  pub fn draw(&self)
  {
    check!(gl::bind_vertex_array(self.vao));
    check!(gl::bind_buffer(gl::ARRAY_BUFFER, self.vbo));

    check!(gl::vertex_attrib_pointer_f32(0, 3, false, (sys::size_of::<Vec3f>() * 2) as i32, 0));
    check!(gl::vertex_attrib_pointer_f32(1, 3, false, (sys::size_of::<Vec3f>() * 2) as i32, sys::size_of::<Vec3f>() as u32));
    check!(gl::enable_vertex_attrib_array(0));
    check!(gl::enable_vertex_attrib_array(1));

    check!(gl::draw_arrays(gl::TRIANGLES, 0, (self.verts.len() as i32) / 2));

    check!(gl::disable_vertex_attrib_array(0));
    check!(gl::disable_vertex_attrib_array(1));
    check!(gl::bind_vertex_array(0));
    check!(gl::bind_buffer(gl::ARRAY_BUFFER, 0));
  }
}

priv fn voxelize(verts: &[Vec3f]) -> ~[Vec3f]
{
  /* Pos -> Col, Pos -> Col, etc */
  let mut new_verts: ~[Vec3f] = ~[];

  /* Bounding box of vert dimensions. */
  /* Calculate, given resolution (how many voxels across), the dimensions of a voxel. */
  /* Create 3D array of voxels. Render wireframe? */
  /* Triangle -> box collision checking to enable voxels. */
  /* Pass back on to sphere for rendering. */

  new_verts
}


