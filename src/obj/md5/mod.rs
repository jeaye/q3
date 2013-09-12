/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: obj/md5/mod.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      An aggregator of MD5 items.
*/

/* Model */
pub use self::model::Model;
pub use self::model_renderer::Model_Renderer;
pub use self::joint::Joint;
pub use self::weight::Weight;
pub use self::mesh::Mesh;
pub use self::mesh_renderer::Mesh_Renderer;
pub use self::triangle::Triangle;
pub use self::vertex::Vertex;

/* Animation */
pub use self::joint_info::Joint_Info;
pub use self::bound::Bound;
pub use self::frame::{ Base_Frame, Frame_Data };
pub use self::skeleton::{ Skeleton_Joint, Frame_Skeleton };
pub use self::animation::Animation;

/* Model */
#[path = "model/model.rs"]
mod model;
#[path = "model/model_renderer.rs"]
mod model_renderer;
#[path = "model/joint.rs"]
mod joint;
#[path = "model/weight.rs"]
mod weight;
#[path = "model/mesh.rs"]
mod mesh;
#[path = "model/mesh_renderer.rs"]
mod mesh_renderer;
#[path = "model/vertex.rs"]
mod vertex;
#[path = "model/triangle.rs"]
mod triangle;

/* Animation */
#[path = "animation/joint_info.rs"]
mod joint_info;
#[path = "animation/bound.rs"]
mod bound;
#[path = "animation/frame.rs"]
mod frame;
#[path = "animation/skeleton.rs"]
mod skeleton;
#[path = "animation/animation.rs"]
mod animation;

