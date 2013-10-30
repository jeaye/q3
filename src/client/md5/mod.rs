/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: shared/obj/md5/mod.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      An aggregator of MD5 items.
*/

#[link(name = "md5", vers = "0.2")];
#[crate_type = "lib"];

#[feature(globs)];
#[feature(macro_rules)];
#[feature(managed_boxes)];

extern mod opengles;
extern mod stb_image;

extern mod log;
extern mod math;
extern mod console;
extern mod gl;

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
pub mod model;
#[path = "model/model_renderer.rs"]
pub mod model_renderer;
#[path = "model/joint.rs"]
pub mod joint;
#[path = "model/weight.rs"]
pub mod weight;
#[path = "model/mesh.rs"]
pub mod mesh;
#[path = "model/mesh_renderer.rs"]
pub mod mesh_renderer;
#[path = "model/vertex.rs"]
pub mod vertex;
#[path = "model/triangle.rs"]
pub mod triangle;

/* Animation */
#[path = "animation/joint_info.rs"]
pub mod joint_info;
#[path = "animation/bound.rs"]
pub mod bound;
#[path = "animation/frame.rs"]
pub mod frame;
#[path = "animation/skeleton.rs"]
pub mod skeleton;
#[path = "animation/animation.rs"]
pub mod animation;

