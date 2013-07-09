/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: obj/primitive/sphere.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      An arbitrarily subdivided Icosahedron.
*/

use std::{ uint, sys };
use math;
use primitive::{ Triangle, Vertex_PC };
use gl2 = opengles::gl2;

#[path = "../../gl/check.rs"]
mod check;

pub struct Sphere
{
  radius: f32,
  vao: gl2::GLuint,
  vbo: gl2::GLuint,
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

    let root_verts: [math::Vec3f, ..12] =
    [
      math::Vec3f::new(-magic_x, 0.0, magic_z),
      math::Vec3f::new(magic_x, 0.0, magic_z),
      math::Vec3f::new(-magic_x, 0.0, -magic_z),
      math::Vec3f::new(magic_x, 0.0, -magic_z),

      math::Vec3f::new(0.0, magic_z, magic_x),
      math::Vec3f::new(0.0, magic_z, -magic_x),
      math::Vec3f::new(0.0, -magic_z, magic_x),
      math::Vec3f::new(0.0, -magic_z, -magic_x),

      math::Vec3f::new(magic_z, magic_x, 0.0),
      math::Vec3f::new(-magic_z, magic_x, 0.0),
      math::Vec3f::new(magic_z, -magic_x, 0.0),
      math::Vec3f::new(-magic_z, -magic_x, 0.0), 
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

    let name = check!(gl2::gen_vertex_arrays(1));
    assert!(name.len() == 1);
    sphere.vao = name[0];

    let name = check!(gl2::gen_buffers(1));
    assert!(name.len() == 1);
    sphere.vbo = name[0];
    check!(gl2::bind_vertex_array(sphere.vao));
    check!(gl2::bind_buffer(gl2::ARRAY_BUFFER, sphere.vbo));
    check!(gl2::buffer_data(gl2::ARRAY_BUFFER, sphere.tris, gl2::STATIC_DRAW));

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

    let mut v12 = Vertex_PC::zero();
    let mut v23 = Vertex_PC::zero();
    let mut v31 = Vertex_PC::zero();
    v12.color = tri.verts[0].color;
    v23.color = tri.verts[0].color;
    v31.color = tri.verts[0].color;

    v12.position.x = tri.verts[0].position.x + tri.verts[1].position.x; /* XXX: Lack of clean mutable indexing. */
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
    check!(gl2::bind_vertex_array(self.vao));
    check!(gl2::bind_buffer(gl2::ARRAY_BUFFER, self.vbo));

    check!(gl2::vertex_attrib_pointer_f32(0, 3, false, (sys::size_of::<Vertex_PC>()) as i32, 0));
    check!(gl2::vertex_attrib_pointer_f32(1, 3, false, (sys::size_of::<Vertex_PC>()) as i32, sys::size_of::<math::Vec3f>() as u32));
    check!(gl2::enable_vertex_attrib_array(0));
    check!(gl2::enable_vertex_attrib_array(1));

    check!(gl2::polygon_mode(gl2::FRONT_AND_BACK, gl2::LINE));
    check!(gl2::draw_arrays(gl2::TRIANGLES, 0, (self.tris.len() as i32 * 3)));
    check!(gl2::polygon_mode(gl2::FRONT_AND_BACK, gl2::FILL));

    check!(gl2::disable_vertex_attrib_array(0));
    check!(gl2::disable_vertex_attrib_array(1));
    check!(gl2::bind_vertex_array(0));
    check!(gl2::bind_buffer(gl2::ARRAY_BUFFER, 0));
  }
}

