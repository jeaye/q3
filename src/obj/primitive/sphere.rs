/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: obj/primitive/sphere.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      An arbitrarily subdivided Icosahedron.
*/

use math::{ Vec3f };
use primitive::Vertex_PC;
use primitive::Triangle;

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
  tris: ~[Triangle],
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
      tris: ~[],
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

    let tris: &[Triangle] = 
    &[
      /* http://www.glprogramming.com/red/chapter02.html#name8 */
      Triangle::new(Vertex_PC::new(root_verts[0], root_verts[0]), Vertex_PC::new(root_verts[4], root_verts[0]), Vertex_PC::new(root_verts[1], root_verts[0])), 
      Triangle::new(Vertex_PC::new(root_verts[0], root_verts[1]), Vertex_PC::new(root_verts[9], root_verts[1]), Vertex_PC::new(root_verts[4], root_verts[1])),  
      Triangle::new(Vertex_PC::new(root_verts[9], root_verts[2]), Vertex_PC::new(root_verts[5], root_verts[2]), Vertex_PC::new(root_verts[4], root_verts[2])), 
      Triangle::new(Vertex_PC::new(root_verts[4], root_verts[3]), Vertex_PC::new(root_verts[5], root_verts[3]), Vertex_PC::new(root_verts[8], root_verts[3])),  
      Triangle::new(Vertex_PC::new(root_verts[4], root_verts[4]), Vertex_PC::new(root_verts[8], root_verts[4]), Vertex_PC::new(root_verts[1], root_verts[4])), 
      Triangle::new(Vertex_PC::new(root_verts[8], root_verts[5]), Vertex_PC::new(root_verts[10],root_verts[5]), Vertex_PC::new(root_verts[1], root_verts[5])), 
      Triangle::new(Vertex_PC::new(root_verts[8], root_verts[6]), Vertex_PC::new(root_verts[3], root_verts[6]), Vertex_PC::new(root_verts[10],root_verts[6])), 
      Triangle::new(Vertex_PC::new(root_verts[5], root_verts[7]), Vertex_PC::new(root_verts[3], root_verts[7]), Vertex_PC::new(root_verts[8], root_verts[7])), 
      Triangle::new(Vertex_PC::new(root_verts[5], root_verts[8]), Vertex_PC::new(root_verts[2], root_verts[8]), Vertex_PC::new(root_verts[3], root_verts[8])), 
      Triangle::new(Vertex_PC::new(root_verts[2], root_verts[9]), Vertex_PC::new(root_verts[7], root_verts[9]), Vertex_PC::new(root_verts[3], root_verts[9])),  
      Triangle::new(Vertex_PC::new(root_verts[7], root_verts[1]), Vertex_PC::new(root_verts[10],root_verts[1]), Vertex_PC::new(root_verts[3], root_verts[1])), 
      Triangle::new(Vertex_PC::new(root_verts[7], root_verts[2]), Vertex_PC::new(root_verts[6], root_verts[2]), Vertex_PC::new(root_verts[10],root_verts[2])),
      Triangle::new(Vertex_PC::new(root_verts[7], root_verts[3]), Vertex_PC::new(root_verts[11],root_verts[3]), Vertex_PC::new(root_verts[6], root_verts[3])), 
      Triangle::new(Vertex_PC::new(root_verts[11],root_verts[4]), Vertex_PC::new(root_verts[0], root_verts[4]), Vertex_PC::new(root_verts[6], root_verts[4])),
      Triangle::new(Vertex_PC::new(root_verts[0], root_verts[5]), Vertex_PC::new(root_verts[1], root_verts[5]), Vertex_PC::new(root_verts[6], root_verts[5])), 
      Triangle::new(Vertex_PC::new(root_verts[6], root_verts[6]), Vertex_PC::new(root_verts[1], root_verts[6]), Vertex_PC::new(root_verts[10],root_verts[6])),  
      Triangle::new(Vertex_PC::new(root_verts[9], root_verts[7]), Vertex_PC::new(root_verts[0], root_verts[7]), Vertex_PC::new(root_verts[11],root_verts[7])), 
      Triangle::new(Vertex_PC::new(root_verts[9], root_verts[8]), Vertex_PC::new(root_verts[11],root_verts[8]), Vertex_PC::new(root_verts[2], root_verts[8])),
      Triangle::new(Vertex_PC::new(root_verts[9], root_verts[9]), Vertex_PC::new(root_verts[2], root_verts[9]), Vertex_PC::new(root_verts[5], root_verts[9])), 
      Triangle::new(Vertex_PC::new(root_verts[7], root_verts[0]), Vertex_PC::new(root_verts[2], root_verts[0]), Vertex_PC::new(root_verts[11],root_verts[0])),  
    ];
    for uint::range(0, tris.len()) |x|
    { sphere.subdivide(tris[x], new_subdivides); }

    sphere.vao = check!(gl::gen_vertex_arrays(1))[0]; /* TODO: Check these. */
    sphere.vbo = check!(gl::gen_buffers(1))[0];
    check!(gl::bind_vertex_array(sphere.vao));
    check!(gl::bind_buffer(gl::ARRAY_BUFFER, sphere.vbo));
    check!(gl::buffer_data(gl::ARRAY_BUFFER, sphere.tris, gl::STATIC_DRAW));

    sphere
  }

  /* Recursive subdivide for a given triangle. */
  priv fn subdivide(&mut self, tri: Triangle, depth: i32)
  {
    if depth == 0
    {
      self.tris.push(tri);
      return;
    }

    let mut v12 = Vertex_PC::zero(), v23 = Vertex_PC::zero(), v31 = Vertex_PC::zero();
    v12.color = tri.verts[0].color;
    v23.color = tri.verts[0].color;
    v31.color = tri.verts[0].color;

    v12.position.x = tri.verts[0].position.x + tri.verts[1].position.x; /* TODO: Lack of clean mutable indexing. */
    v23.position.x = tri.verts[1].position.x + tri.verts[2].position.x;
    v31.position.x = tri.verts[2].position.x + tri.verts[0].position.x;

    v12.position.y = tri.verts[0].position.y + tri.verts[1].position.y;
    v23.position.y = tri.verts[1].position.y + tri.verts[2].position.y;
    v31.position.y = tri.verts[2].position.y + tri.verts[0].position.y;

    v12.position.z = tri.verts[0].position.z + tri.verts[1].position.z;
    v23.position.z = tri.verts[1].position.z + tri.verts[2].position.z;
    v31.position.z = tri.verts[2].position.z + tri.verts[0].position.z;

    v12.position.normalize();
    v23.position.normalize();
    v31.position.normalize();
    self.subdivide(Triangle::new(tri.verts[0], v12, v31), depth - 1);
    self.subdivide(Triangle::new(tri.verts[1], v23, v12), depth - 1);
    self.subdivide(Triangle::new(tri.verts[2], v31, v23), depth - 1);
    self.subdivide(Triangle::new(v12, v23, v31), depth - 1);
  }

  pub fn draw(&self)
  {
    check!(gl::bind_vertex_array(self.vao));
    check!(gl::bind_buffer(gl::ARRAY_BUFFER, self.vbo));

    check!(gl::vertex_attrib_pointer_f32(0, 3, false, (sys::size_of::<Vertex_PC>()) as i32, 0));
    check!(gl::vertex_attrib_pointer_f32(1, 3, false, (sys::size_of::<Vertex_PC>()) as i32, sys::size_of::<Vec3f>() as u32));
    check!(gl::enable_vertex_attrib_array(0));
    check!(gl::enable_vertex_attrib_array(1));

    check!(gl::polygon_mode(gl::FRONT_AND_BACK, gl::LINE));
    check!(gl::draw_arrays(gl::TRIANGLES, 0, (self.tris.len() as i32 * 3)));
    check!(gl::polygon_mode(gl::FRONT_AND_BACK, gl::FILL));

    check!(gl::disable_vertex_attrib_array(0));
    check!(gl::disable_vertex_attrib_array(1));
    check!(gl::bind_vertex_array(0));
    check!(gl::bind_buffer(gl::ARRAY_BUFFER, 0));
  }
}

