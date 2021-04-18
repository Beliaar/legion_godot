use gdnative::prelude::*;
use legion::systems::CommandBuffer;
use legion::{component, system, Entity};

use crate::components::nodes::{NodeComponent, NodePosition3D, NodeScale2D, NodeScale3D};
use crate::components::nodes::{NodePosition2D, NodeTemplate, NodeType};

#[derive(Debug, Clone, PartialEq)]
pub enum ManageErrs {
    CouldNotMakeInstance,
    RootClassNotSpatial(String),
}

#[system(for_each)]
#[filter(!component::<NodeComponent<Node2D>>())]
#[filter(!component::<NodeComponent<Spatial>>())]
pub fn create_node_2d(
    cmd: &mut CommandBuffer,
    #[state] root_node: &Ref<Node>,
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
                unsafe {
                    let node2d = node2d.assume_safe_if_sane().unwrap();
                }
                root_node.add_child(node2d, false);

                let entity = *entity;
                cmd.exec_mut(move |world, _| {
                    let mut entry = world.entry(entity).unwrap();
                    entry.add_component(NodeComponent { node: node2d });
                })
            }
            Err(err) => godot_error!("create_node_2d: Could not instance Child : {:?}", err),
        },
        NodeType::Spatial => {
            match instance_scene::<Spatial>(template) {
                Ok(spatial) => {
                    let spatial: Ref<Spatial> = spatial.into_shared();
                    unsafe {
                        let spatial = spatial.assume_safe_if_sane().unwrap();
                        // node2d.set_scale(Vector2::new(template_data.scale_x, template_data.scale_y));
                    }
                    root_node.add_child(spatial, false);

                    let entity = *entity;
                    cmd.exec_mut(move |world, _| {
                        let mut entry = world.entry(entity).unwrap();
                        entry.add_component(NodeComponent { node: spatial });
                    })
                }
                Err(err) => godot_error!("create_node_2d: Could not instance child : {:?}", err),
            }
        }
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
fn update_position_2d(node_compenent: &NodeComponent<Node2D>, node_position: &NodePosition2D) {
    match node_compenent.get_node() {
        Some(node) => {
            node.set_position(Vector2::new(node_position.x, node_position.y));
            node.set_z_index(node_position.z);
            node.set_z_as_relative(false);
        }
        None => godot_error!("update_position_2d: Could not acquire node"),
    };
}

#[system(for_each)]
fn update_scale_2d(node_compenent: &NodeComponent<Node2D>, node_scale: &NodeScale2D) {
    match node_compenent.get_node() {
        Some(node) => {
            node.set_scale(Vector2::new(node_scale.x, node_scale.y));
        }
        None => godot_error!("update_scale_2d: Could not acquire node"),
    };
}

#[system(for_each)]
fn update_position_3d(node_compenent: &NodeComponent<Spatial>, node_position: &NodePosition3D) {
    match node_compenent.get_node() {
        Some(node) => {
            node.set_translation(Vector3::new(
                node_position.x,
                node_position.y,
                node_position.z,
            ));
        }
        None => godot_error!("update_position_3d: Could not acquire node"),
    };
}

#[system(for_each)]
fn update_scale_3d(node_compenent: &NodeComponent<Spatial>, node_scale: &NodeScale3D) {
    match node_compenent.get_node() {
        Some(node) => {
            node.set_scale(Vector3::new(node_scale.x, node_scale.y, node_scale.z));
        }
        None => godot_error!("update_scale_3d: Could not acquire node"),
    };
}
