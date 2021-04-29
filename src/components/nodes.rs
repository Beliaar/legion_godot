use gdnative::prelude::*;
use gdnative::{Ref, TRef};

#[derive(Copy, Clone, Debug, Default)]
pub struct Position2D {
    pub x: f32,
    pub y: f32,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct ZIndex {
    pub value: i64,
    pub is_relative: bool,
}

#[derive(Copy, Clone, Debug)]
pub struct Scale2D {
    pub x: f32,
    pub y: f32,
}

impl Default for Scale2D {
    fn default() -> Self {
        Scale2D { x: 1.0, y: 1.0 }
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Rotation2D(pub f64);

#[derive(Copy, Clone, Debug, Default)]
pub struct Position3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Copy, Clone, Debug)]
pub struct Scale3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Default for Scale3D {
    fn default() -> Self {
        Self {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Rotation3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NodeType {
    Node2D,
    Spatial,
}

#[derive(Clone, Debug)]
pub struct NodeTemplate {
    pub node_type: NodeType,
    pub scene_file: String,
}

#[derive(Clone, Debug)]
pub struct NodeComponent<T: SubClass<Node>> {
    pub node: Ref<T>,
}

pub type NodeComponent2D = NodeComponent<Node2D>;

impl NodeComponent2D {
    pub fn get_node(&self) -> Option<TRef<'_, Node2D>> {
        unsafe { self.node.assume_safe_if_sane() }
    }
}

unsafe impl Send for NodeComponent2D {}

unsafe impl Sync for NodeComponent2D {}

pub type NodeComponentSpatial = NodeComponent<Spatial>;

impl NodeComponentSpatial {
    pub fn get_node(&self) -> Option<TRef<'_, Spatial>> {
        unsafe { self.node.assume_safe_if_sane() }
    }
}

unsafe impl Send for NodeComponentSpatial {}

unsafe impl Sync for NodeComponentSpatial {}
