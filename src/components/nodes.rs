use gdnative::{Ref, TRef};
use gdnative::prelude::*;

#[derive(Copy, Clone)]
pub struct NodePosition2D {
	pub x: f32,
	pub y: f32,
	pub z: i64,
}

#[derive(Copy, Clone)]
pub struct NodePosition3D {
	pub x: f32,
	pub y: f32,
	pub z: f32,
}

#[derive(Copy, Clone)]
pub struct NodeScale2D {
	pub x: f32,
	pub y: f32,
	pub z: f32,
}

#[derive(Copy, Clone)]
pub struct NodeScale3D {
	pub x: f32,
	pub y: f32,
	pub z: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub enum NodeType {
    Node2D,
    Spatial
}

#[derive(Clone)]
pub struct NodeTemplate {
    pub node_type: NodeType,
    pub scene_file: String,
}

#[derive(Clone)]
pub struct NodeComponent<T: SubClass<Node>> {
    pub node: Ref<T>,
}

impl NodeComponent<Node2D> {
    pub fn get_node(&self) -> Option<TRef<'_, Node2D>> {
        unsafe { self.node.assume_safe_if_sane() }
    }
}

unsafe impl Send for NodeComponent<Node2D> {}

unsafe impl Sync for NodeComponent<Node2D> {}

impl NodeComponent<Spatial> {
	pub fn get_node(&self) -> Option<TRef<'_, Spatial>> {
		unsafe { self.node.assume_safe_if_sane() }
	}
}

unsafe impl Send for NodeComponent<Spatial> {}

unsafe impl Sync for NodeComponent<Spatial> {}