/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: obj/primitive/sphere.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      An arbitrarily subdivided Icosahedron.
*/

use math::{ Vec3f, BB3 };
use primitive::Vertex_PC;
use primitive::Cube;

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
  verts: ~[Vertex_PC],
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

    let verts: &[Vertex_PC] = 
    &[
      /* http://www.glprogramming.com/red/chapter02.html#name8 */
      Vertex_PC::new(root_verts[0], root_verts[0]), Vertex_PC::new(root_verts[4], root_verts[0]), Vertex_PC::new(root_verts[1], root_verts[0]), Vertex_PC::new(root_verts[0], root_verts[1]), Vertex_PC::new(root_verts[9], root_verts[1]),   Vertex_PC::new(root_verts[4],  root_verts[1]),  
      Vertex_PC::new(root_verts[9], root_verts[2]), Vertex_PC::new(root_verts[5], root_verts[2]), Vertex_PC::new(root_verts[4], root_verts[2]), Vertex_PC::new(root_verts[4], root_verts[3]), Vertex_PC::new(root_verts[5], root_verts[3]),   Vertex_PC::new(root_verts[8],  root_verts[3]),  
      Vertex_PC::new(root_verts[4], root_verts[4]), Vertex_PC::new(root_verts[8], root_verts[4]), Vertex_PC::new(root_verts[1], root_verts[4]), Vertex_PC::new(root_verts[8], root_verts[5]), Vertex_PC::new(root_verts[10],root_verts[5]),  Vertex_PC::new(root_verts[1],  root_verts[5]), 
      Vertex_PC::new(root_verts[8], root_verts[6]), Vertex_PC::new(root_verts[3], root_verts[6]), Vertex_PC::new(root_verts[10],root_verts[6]), Vertex_PC::new(root_verts[5], root_verts[7]), Vertex_PC::new(root_verts[3], root_verts[7]),  Vertex_PC::new(root_verts[8],  root_verts[7]), 
      Vertex_PC::new(root_verts[5], root_verts[8]), Vertex_PC::new(root_verts[2], root_verts[8]), Vertex_PC::new(root_verts[3], root_verts[8]), Vertex_PC::new(root_verts[2], root_verts[9]), Vertex_PC::new(root_verts[7], root_verts[9]),   Vertex_PC::new(root_verts[3],  root_verts[9]),  
      Vertex_PC::new(root_verts[7], root_verts[1]), Vertex_PC::new(root_verts[10],root_verts[1]), Vertex_PC::new(root_verts[3], root_verts[1]), Vertex_PC::new(root_verts[7], root_verts[2]), Vertex_PC::new(root_verts[6], root_verts[2]),  Vertex_PC::new(root_verts[10], root_verts[2]),
      Vertex_PC::new(root_verts[7], root_verts[3]), Vertex_PC::new(root_verts[11],root_verts[3]), Vertex_PC::new(root_verts[6], root_verts[3]), Vertex_PC::new(root_verts[11],root_verts[4]), Vertex_PC::new(root_verts[0], root_verts[4]), Vertex_PC::new(root_verts[6],   root_verts[4]),
      Vertex_PC::new(root_verts[0], root_verts[5]), Vertex_PC::new(root_verts[1], root_verts[5]), Vertex_PC::new(root_verts[6], root_verts[5]), Vertex_PC::new(root_verts[6], root_verts[6]), Vertex_PC::new(root_verts[1], root_verts[6]),    Vertex_PC::new(root_verts[10], root_verts[6]),  
      Vertex_PC::new(root_verts[9], root_verts[7]), Vertex_PC::new(root_verts[0], root_verts[7]), Vertex_PC::new(root_verts[11],root_verts[7]), Vertex_PC::new(root_verts[9], root_verts[8]), Vertex_PC::new(root_verts[11],root_verts[8]), Vertex_PC::new(root_verts[2],  root_verts[8]),
      Vertex_PC::new(root_verts[9], root_verts[9]), Vertex_PC::new(root_verts[2], root_verts[9]), Vertex_PC::new(root_verts[5], root_verts[9]), Vertex_PC::new(root_verts[7], root_verts[0]), Vertex_PC::new(root_verts[2], root_verts[0]),    Vertex_PC::new(root_verts[11], root_verts[0]),  
    ];
    for uint::range_step(0, verts.len(), 3) |x|
    { sphere.subdivide(verts[x], verts[x + 1], verts[x + 2], new_subdivides); }
    let voxels = voxelize(sphere.verts);
    //sphere.verts = voxelize(sphere.verts);

    sphere.vao = check!(gl::gen_vertex_arrays(1))[0]; /* TODO: Check these. */
    sphere.vbo = check!(gl::gen_buffers(1))[0];
    check!(gl::bind_vertex_array(sphere.vao));
    check!(gl::bind_buffer(gl::ARRAY_BUFFER, sphere.vbo));
    check!(gl::buffer_data(gl::ARRAY_BUFFER, voxels, gl::STATIC_DRAW));

    sphere
  }

  /* Recursive subdivide for a given triangle. */
  priv fn subdivide(&mut self, v1: Vertex_PC, v2: Vertex_PC, v3: Vertex_PC, depth: i32)
  {
    if depth == 0
    {
      self.verts.push(v1);
      self.verts.push(v2);
      self.verts.push(v3);
      return;
    }

    let mut v12 = Vertex_PC::zero(), v23 = Vertex_PC::zero(), v31 = Vertex_PC::zero();
    v12.color = v1.color;
    v23.color = v1.color;
    v31.color = v1.color;

    v12.position.x = v1.position.x + v2.position.x; /* TODO: Lack of clean mutable indexing. */
    v23.position.x = v2.position.x + v3.position.x;
    v31.position.x = v3.position.x + v1.position.x;

    v12.position.y = v1.position.y + v2.position.y;
    v23.position.y = v2.position.y + v3.position.y;
    v31.position.y = v3.position.y + v1.position.y;

    v12.position.z = v1.position.z + v2.position.z;
    v23.position.z = v2.position.z + v3.position.z;
    v31.position.z = v3.position.z + v1.position.z;

    v12.position.normalize();
    v23.position.normalize();
    v31.position.normalize();
    self.subdivide(v1, v12, v31, depth - 1);
    self.subdivide(v2, v23, v12, depth - 1);
    self.subdivide(v3, v31, v23, depth - 1);
    self.subdivide(v12, v23, v31, depth - 1);
  }

  pub fn draw(&self)
  {
    check!(gl::bind_vertex_array(self.vao));
    check!(gl::bind_buffer(gl::ARRAY_BUFFER, self.vbo));

    check!(gl::vertex_attrib_pointer_f32(0, 3, false, (sys::size_of::<Vertex_PC>()) as i32, 0));
    check!(gl::vertex_attrib_pointer_f32(1, 3, false, (sys::size_of::<Vertex_PC>()) as i32, sys::size_of::<Vec3f>() as u32));
    check!(gl::enable_vertex_attrib_array(0));
    check!(gl::enable_vertex_attrib_array(1));

    check!(gl::draw_arrays(gl::TRIANGLES, 0, (self.verts.len() as i32)));

    check!(gl::disable_vertex_attrib_array(0));
    check!(gl::disable_vertex_attrib_array(1));
    check!(gl::bind_vertex_array(0));
    check!(gl::bind_buffer(gl::ARRAY_BUFFER, 0));
  }
}

priv fn voxelize(verts: &[Vertex_PC]) -> ~[Cube]
{
  /* Require at least one triangle. */
  assert!(verts.len() >= 3);

  let resolution = 10.0f32;

  /* Bounding box of vert dimensions. */
  let mut min = Vec3f::new(verts[0].position.x, verts[0].position.y, verts[0].position.z);
  let mut max = Vec3f::new(verts[0].position.x, verts[0].position.y, verts[0].position.z);
  for verts.each |curr|
  {
    min.x = cmp::min(min.x, curr.position.x);
    min.y = cmp::min(min.y, curr.position.y);
    min.z = cmp::min(min.z, curr.position.z);

    max.x = cmp::max(max.x, curr.position.x);
    max.y = cmp::max(max.y, curr.position.y);
    max.z = cmp::max(max.z, curr.position.z);
  }
  io::println(fmt!("Min: %s", min.to_str()));
  io::println(fmt!("Max: %s", max.to_str()));

  /* Calculate, given resolution (how many voxels across), the dimensions of a voxel. */
  let size = cmp::max(max.x - min.x, cmp::max(max.y - min.y, max.z - min.z)) / resolution;
  io::println(fmt!("Size: %?", size));

  /* Create 3D array of voxels. Render wireframe? */
  macro_rules! index
  (
    ($arr:ident[$x:expr][$y:expr][$z:expr]) => 
    (
      $arr[($z * resolution * resolution) + (y * resolution) + x]
    )
  )
  let mut new_verts: ~[Cube] = vec::with_capacity((resolution * resolution * resolution) as uint);
  for uint::range(0, resolution as uint) |z|
  {
    for uint::range(0, resolution as uint) |y|
    {
      for uint::range(0, resolution as uint) |x|
      {
        new_verts.push(Cube::new(size, Vec3f::new(x as f32, y as f32, z as f32)));
      }
    }
  }
  /* Triangle -> box collision checking to enable voxels. */

  /* Pass back on to sphere for rendering. */
  new_verts
}


