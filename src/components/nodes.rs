use gdnative::prelude::*;
use gdnative::{Ref, TRef};

#[derive(Copy, Clone, Debug)]
pub struct NodeTransform2D {
    pub x_pos: f32,
    pub y_pos: f32,
    pub z_index: i64,
    pub z_index_relative: bool,
    pub x_scale: f32,
    pub y_scale: f32,
    pub rotation_degrees: f64,
}

impl Default for NodeTransform2D {
    fn default() -> Self {
        NodeTransform2D {
            y_pos: 0.0,
            x_pos: 0.0,
            z_index: 0,
            z_index_relative: false,
            x_scale: 1.0,
            y_scale: 1.0,
            rotation_degrees: 0.0,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct NodeTransform3D {
    pub x_pos: f32,
    pub y_pos: f32,
    pub z_pos: f32,
    pub x_scale: f32,
    pub y_scale: f32,
    pub z_scale: f32,
    pub x_rotation_degrees: f32,
    pub y_rotation_degrees: f32,
    pub z_rotation_degrees: f32,
}

impl Default for NodeTransform3D {
    fn default() -> Self {
        NodeTransform3D {
            x_pos: 0.0,
            y_pos: 0.0,
            z_pos: 0.0,
            x_scale: 1.0,
            y_scale: 1.0,
            z_scale: 1.0,
            x_rotation_degrees: 0.0,
            y_rotation_degrees: 0.0,
            z_rotation_degrees: 0.0,
        }
    }
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
