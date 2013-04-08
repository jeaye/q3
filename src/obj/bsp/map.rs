/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: obj/map.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      Loader and handler of BSP maps.
*/

pub use self::math::*;

#[path = "lump.rs"]
mod lump;
#[path = "../../math/math.rs"]
mod math;

#[path = "../../gl/gl.rs"]
mod gl;
#[path = "../../gl/check.rs"]
mod check;

pub struct Map
{
  header: lump::Header,
  entity: lump::Entity,
  verts: ~[lump::Vertex],
  faces: ~[lump::Face],
  mesh_verts: ~[lump::Mesh_Vert],
  vbo: ~[gl::GLuint],
  position: math::Vec3f, /* TODO: Trait for positional objects. */
  bb: math::BB3
}

impl Map
{
  pub fn new(file: &str) -> Map
  {
    let mut map = Map{  header: lump::Header::new(),
                        entity: lump::Entity::new(),
                        verts: ~[],
                        faces: ~[],
                        mesh_verts: ~[],
                        vbo: ~[],
                        position: math::Vec3f::zero(),
                        bb: math::BB3::zero()
                        };

    let mut fio = io::file_reader(@path::PosixPath(file)).unwrap();
    unsafe {  fio.read( cast::transmute((&map.header, sys::size_of::<lump::Header>())),
              sys::size_of::<lump::Header>()); }

    assert!( map.header.magic[0] == 'I' as i8 &&
             map.header.magic[1] == 'B' as i8 &&
             map.header.magic[2] == 'S' as i8 &&
             map.header.magic[3] == 'P' as i8);

    map.read_verts(fio);
    map.read_faces(fio);
    map.read_mesh_verts(fio);

    map.triangulate();
    map.upload();
    
    map
  }


  /* TODO: Make this generic. */
  priv fn read_verts(&mut self, fio: @io::Reader)
  {
    fio.seek(self.header.lumps[lump::Vertex_Type as int].offset as int, io::SeekSet);
    let num_verts = (self.header.lumps[lump::Vertex_Type as int].length) /
                    (sys::size_of::<lump::Vertex>() as i32);
    assert!(num_verts > 0);

    let mut vert = lump::Vertex::new();
    for i32::range(0, num_verts) |i|
    {
      unsafe { fio.read( cast::transmute((&vert, sys::size_of::<lump::Vertex>())),
                sys::size_of::<lump::Vertex>()); }
      
      /* BSP likes Z to be up; I like Y to be up. */
      let temp = vert.position.y;
      vert.position.y = vert.position.z;
      vert.position.z = -temp;

      /* TODO: Color hack. */
      if vert.color.x == 0 { vert.color.x += 100; }
      if vert.color.y == 0 { vert.color.y += 100; }
      if vert.color.z == 0 { vert.color.z += 100; }

      if vert.color.x == 255 { vert.color.x -= 100; }
      if vert.color.y == 255 { vert.color.y -= 100; }
      if vert.color.z == 255 { vert.color.z -= 100; }
      vert.color.w = 1;

      /* Global size scale. */
      vert.position.x /= 32.0;
      vert.position.y /= 32.0;
      vert.position.z /= 32.0;

      /* Create bounding box based on first vert; this'll update as more come. */
      match i
      {
        0 =>
        { self.bb.top_left = vert.position; self.bb.bottom_right = vert.position; }
        _ =>
        {
          /* Keep track of bounds. */
          if vert.position.x < self.bb.top_left.x
          { self.bb.top_left.x = vert.position.x; }
          else if vert.position.x > self.bb.bottom_right.x
          { self.bb.bottom_right.x = vert.position.x }

          if vert.position.y > self.bb.top_left.y
          { self.bb.top_left.y = vert.position.y; }
          else if vert.position.y < self.bb.bottom_right.y
          { self.bb.bottom_right.y = vert.position.y }

          if vert.position.z > self.bb.top_left.z
          { self.bb.top_left.z = vert.position.z; }
          else if vert.position.z < self.bb.bottom_right.z
          { self.bb.bottom_right.z = vert.position.z }
        }
      }

      self.verts.push(vert);
    }
  }

  priv fn read_faces(&mut self, fio: @io::Reader)
  {
    fio.seek(self.header.lumps[lump::Face_Type as int].offset as int, io::SeekSet);
    let num_faces = (self.header.lumps[lump::Face_Type as int].length) /
                    (sys::size_of::<lump::Face>() as i32);
    assert!(num_faces > 0);

    let mut face = lump::Face::new();

    for i32::range(0, num_faces) |_|
    {
      unsafe { fio.read( cast::transmute((&face, sys::size_of::<lump::Face>())),
                sys::size_of::<lump::Face>()); }
      self.faces.push(face);
    }
  }

  priv fn read_mesh_verts(&mut self, fio: @io::Reader)
  {
    fio.seek(self.header.lumps[lump::Mesh_Vert_Type as int].offset as int, io::SeekSet);
    let num_obj = (self.header.lumps[lump::Mesh_Vert_Type as int].length) /
                    (sys::size_of::<lump::Mesh_Vert>() as i32);
    assert!(num_obj > 0);

    let mut obj = lump::Mesh_Vert::new();

    for i32::range(0, num_obj) |_|
    {
      unsafe { fio.read( cast::transmute((&obj, sys::size_of::<lump::Mesh_Vert>())),
                sys::size_of::<lump::Mesh_Vert>()); }
      self.mesh_verts.push(obj);
    }
  }

  priv fn triangulate(&mut self)
  {
    let mut verts: ~[lump::Vertex] = ~[];
    for self.faces.each() |face|
    {
      if face.kind != 1 { loop; }

      match face.num_vertices
      {
        n if n >= 3 =>
        {
          for i32::range(0, n - 2) |i|
          {
            verts.push(self.verts[face.start_vertex]);
            verts.push(self.verts[face.start_vertex + i + 2]);
            verts.push(self.verts[face.start_vertex + i + 1]);
          }
        }
        /* Something else. */
        n => { io::println(fmt!("BSP: Invalid face: %?", n)); }
      }
    };

    self.verts = verts;
  }

  priv fn upload(&mut self)
  {
    unsafe
    {
      self.vbo = check!(gl::gen_buffers(1));
      assert!(self.vbo.len() == 1);
      check!(gl::bind_buffer(gl::ARRAY_BUFFER, self.vbo[0]));
      check!(gl::buffer_data(gl::ARRAY_BUFFER, self.verts, gl::STATIC_DRAW));
    }
  }

  pub fn draw(&self)
  {
    check!(gl::enable_vertex_attrib_array(0));
    check!(gl::enable_vertex_attrib_array(1));
    check!(gl::bind_buffer(gl::ARRAY_BUFFER, self.vbo[0]));
    check!(gl::vertex_attrib_pointer_f32(0, 3, false, 
                sys::size_of::<lump::Vertex>() as i32, 
                0));
    check!(gl::vertex_attrib_pointer_u8(1, 4, false, 
                sys::size_of::<lump::Vertex>() as i32, 
                sys::size_of::<lump::Vertex>() as u32 -
                sys::size_of::<math::Vec4<u8>>() as u32));

    check!(gl::draw_arrays(gl::TRIANGLES, 0, self.verts.len() as i32));

    check!(gl::disable_vertex_attrib_array(0));
    check!(gl::disable_vertex_attrib_array(1));
  }

  pub fn center(&self) -> math::Vec3f
  { self.bb.center_with_offset(self.position) }
}

