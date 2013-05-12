/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: obj/voxel/voxel.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      A 3D volumetric unit.
*/

use super::{ Triangle, Triangle_Index };

#[packed]
pub struct Voxel
{
  triangles: ([Triangle, ..12]),
}
//impl Voxel
//{
//  pub fn new(size: f32, center: Vec3f) -> Voxel
//  {
//    let half = size / 2.0;
//    Voxel
//    {
//      triangles:
//      ([
//          Triangle::new_with_position(Vec3f::new(-half,-half,-half) + center, 
//                                      Vec3f::new(-half,-half, half) + center,
//                                      Vec3f::new(-half, half, half) + center),
//          Triangle::new_with_position(Vec3f::new(half, half,-half) + center,
//                                      Vec3f::new(-half,-half,-half) + center,
//                                      Vec3f::new(-half, half,-half) + center),
//          Triangle::new_with_position(Vec3f::new(half,-half, half) + center,
//                                      Vec3f::new(-half,-half,-half) + center,
//                                      Vec3f::new(half,-half,-half) + center),
//          Triangle::new_with_position(Vec3f::new(half, half,-half) + center,
//                                      Vec3f::new(half,-half,-half) + center,
//                                      Vec3f::new(-half,-half,-half) + center),
//          Triangle::new_with_position(Vec3f::new(-half,-half,-half) + center,
//                                      Vec3f::new(-half, half, half) + center,
//                                      Vec3f::new(-half, half,-half) + center),
//          Triangle::new_with_position(Vec3f::new(half,-half, half) + center,
//                                      Vec3f::new(-half,-half, half) + center,
//                                      Vec3f::new(-half,-half,-half) + center),
//          Triangle::new_with_position(Vec3f::new(-half, half, half) + center,
//                                      Vec3f::new(-half,-half, half) + center,
//                                      Vec3f::new(half,-half, half) + center),
//          Triangle::new_with_position(Vec3f::new(half, half, half) + center,
//                                      Vec3f::new(half,-half,-half) + center,
//                                      Vec3f::new(half, half,-half) + center),
//          Triangle::new_with_position(Vec3f::new(half,-half,-half) + center,
//                                      Vec3f::new(half, half, half) + center,
//                                      Vec3f::new(half,-half, half) + center),
//          Triangle::new_with_position(Vec3f::new(half, half, half) + center,
//                                      Vec3f::new(half, half,-half) + center,
//                                      Vec3f::new(-half, half,-half) + center),
//          Triangle::new_with_position(Vec3f::new(half, half, half) + center,
//                                      Vec3f::new(-half, half,-half) + center,
//                                      Vec3f::new(-half, half, half) + center),
//          Triangle::new_with_position(Vec3f::new(half, half, half) + center,
//                                      Vec3f::new(-half, half, half) + center,
//                                      Vec3f::new(half,-half, half) + center),
//    ])
//    }
//  }
//}
//
#[packed]
struct Voxel_Index
{
  indices: ([Triangle_Index, ..12]),
}
impl Voxel_Index
{
  pub fn new(start: u32) -> Voxel_Index
  {
    let adjusted_start = start * 36; /* Number of verts in a voxel. */
    Voxel_Index
    {
      indices:
      [
        Triangle_Index::new(adjusted_start),
        Triangle_Index::new(adjusted_start + 3),
        Triangle_Index::new(adjusted_start + 6),
        Triangle_Index::new(adjusted_start + 9),
        Triangle_Index::new(adjusted_start + 12),
        Triangle_Index::new(adjusted_start + 15),
        Triangle_Index::new(adjusted_start + 18),
        Triangle_Index::new(adjusted_start + 21),
        Triangle_Index::new(adjusted_start + 24),
        Triangle_Index::new(adjusted_start + 27),
        Triangle_Index::new(adjusted_start + 30),
        Triangle_Index::new(adjusted_start + 33),
      ]
    }
  }
}


