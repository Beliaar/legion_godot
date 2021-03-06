use gdnative::prelude::*;
use legion::systems::CommandBuffer;
use legion::{component, system, Entity};

use crate::components::nodes::{
    NodeComponent2D, NodeComponentSpatial, NodeTemplate, NodeType, Position2D, Position3D,
    Rotation2D, Rotation3D, Scale2D, Scale3D, ZIndex,
};

#[derive(Debug, Clone, PartialEq)]
pub enum ManageErrs {
    CouldNotMakeInstance,
    RootClassNotSpatial(String),
}

// Create 2d nodes from scene files
// Needs to be run as thread local
#[system(for_each)]
#[filter(!component::<NodeComponent2D>())]
#[filter(!component::<NodeComponentSpatial>())]
pub fn create_node_2d(
    cmd: &mut CommandBuffer,
    #[state] root_node: &Ref<Node2D>,
    entity: &Entity,
    template_data: &NodeTemplate,
) {
    let root_node = match unsafe { root_node.assume_safe_if_sane() } {
        Some(node) => node,
        None => {
            godot_warn!("create_node_2d: Root node is not accessible");
            return;
        }
    };

    let template = load_scene(&template_data.scene_file);

    let template = if let Some(template) = &template {
        template
    } else {
        godot_error!(
            "create_node_2d: Could not load scene: {}",
            &template_data.scene_file
        );
        return;
    };

    match template_data.node_type {
        NodeType::Node2D => match instance_scene::<Node2D>(template) {
            Ok(node2d) => {
                let node2d: Ref<Node2D> = node2d.into_shared();
                root_node.add_child(node2d, false);

                let entity = *entity;
                cmd.exec_mut(move |world, _| {
                    let mut entry = world.entry(entity).unwrap();
                    entry.add_component(NodeComponent2D { node: node2d });
                })
            }
            Err(err) => godot_error!("create_node_2d: Could not instance Child : {:?}", err),
        },
        NodeType::Spatial => match instance_scene::<Spatial>(template) {
            Ok(spatial) => {
                let spatial: Ref<Spatial> = spatial.into_shared();
                root_node.add_child(spatial, false);

                let entity = *entity;
                cmd.exec_mut(move |world, _| {
                    let mut entry = world.entry(entity).unwrap();
                    entry.add_component(NodeComponentSpatial { node: spatial });
                })
            }
            Err(err) => godot_error!("create_node_2d: Could not instance child : {:?}", err),
        },
    }
}

pub fn load_scene(path: &str) -> Option<Ref<PackedScene, ThreadLocal>> {
    let scene = ResourceLoader::godot_singleton().load(path, "PackedScene", false)?;

    let scene = unsafe { scene.assume_thread_local() };

    scene.cast::<PackedScene>()
}

#[allow(unused_qualifications)] //It is actually used/needed here, at least according to another rustc error.
fn instance_scene<Root>(scene: &PackedScene) -> Result<Ref<Root, Unique>, ManageErrs>
where
    Root: gdnative::GodotObject<RefKind = ManuallyManaged> + SubClass<Node>,
{
    let instance = scene
        .instance(PackedScene::GEN_EDIT_STATE_DISABLED)
        .ok_or(ManageErrs::CouldNotMakeInstance)?;
    let instance = unsafe { instance.assume_unique() };

    instance
        .try_cast::<Root>()
        .map_err(|instance| ManageErrs::RootClassNotSpatial(instance.name().to_string()))
}

#[system(for_each)]
pub fn update_position_2d(node_component: &NodeComponent2D, position: &Position2D) {
    match node_component.get_node() {
        Some(node) => {
            node.set_position(Vector2::new(position.x, position.y));
        }
        None => godot_error!("update_position_2d: Could not acquire node"),
    };
}

#[system(for_each)]
pub fn update_z_index(node_component: &NodeComponent2D, z_index: &ZIndex) {
    match node_component.get_node() {
        Some(node) => {
            node.set_z_index(z_index.value);
            node.set_z_as_relative(z_index.is_relative);
        }
        None => godot_error!("update_position_2d: Could not acquire node"),
    };
}

#[system(for_each)]
pub fn update_scale_2d(node_component: &NodeComponent2D, scale: &Scale2D) {
    match node_component.get_node() {
        Some(node) => {
            node.set_scale(Vector2::new(scale.x, scale.y));
        }
        None => godot_error!("update_scale_2d: Could not acquire node"),
    };
}

#[system(for_each)]
pub fn update_rotation_2d(node_compenent: &NodeComponent2D, rotation: &Rotation2D) {
    match node_compenent.get_node() {
        Some(node) => {
            node.set_rotation(rotation.0);
        }
        None => godot_error!("update_position_2d: Could not acquire node"),
    };
}

#[system(for_each)]
pub fn update_position_3d(node_compenent: &NodeComponentSpatial, position: &Position3D) {
    match node_compenent.get_node() {
        Some(node) => {
            node.set_translation(Vector3::new(position.x, position.y, position.z));
        }
        None => godot_error!("update_position_3d: Could not acquire node"),
    };
}

#[system(for_each)]
pub fn update_scale_3d(node_compenent: &NodeComponentSpatial, scale: &Scale3D) {
    match node_compenent.get_node() {
        Some(node) => {
            node.set_scale(Vector3::new(scale.x, scale.y, scale.z));
        }
        None => godot_error!("update_position_3d: Could not acquire node"),
    };
}

#[system(for_each)]
pub fn update_rotation_3d(node_compenent: &NodeComponentSpatial, rotation: &Rotation3D) {
    match node_compenent.get_node() {
        Some(node) => {
            node.set_rotation(Vector3::new(rotation.x, rotation.y, rotation.z));
        }
        None => godot_error!("update_position_3d: Could not acquire node"),
    };
}
