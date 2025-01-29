use bevy::prelude::*;
use bevy_vrm::{
    VrmPlugin,
    loader::Vrm
};

mod model;
use model::*;

fn main() {
    env_logger::init();

    App::new()
        .add_plugins(DefaultPlugins) // Bevy's default plugins (includes window, rendering, etc.)
        .add_plugin(VrmPlugin) // Add VRM support
        .add_startup_system(setup)
        .add_system(process_loaded_vrm)
        .run();
}

/// Store VRM handle for retrieval later.
#[derive(Resource)]
struct VrmModelHandle(Handle<Vrm>);

/// Bevy setup system: Create window & load VRM.
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Load VRM model asynchronously
    let vrm_handle = load_vrm(&asset_server, "assets/character.vrm");

    // Store handle for later retrieval
    commands.insert_resource(VrmModelHandle(vrm_handle.clone()));

    // Spawn scene (VRM will be auto-processed by Bevy)
    commands.spawn(SceneBundle {
        scene: vrm_handle,
        ..Default::default()
    });

    println!("VRM model loading...");
}

/// Extract and render the VRM once it's loaded.
fn process_loaded_vrm(
    vrm_assets: Res<Assets<Vrm>>,
    vrm_handle: Res<VrmModelHandle>,
    mut meshes: ResMut<Assets<Mesh>>, // Bevy’s Mesh resource
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    if let Some(vrm_asset) = vrm_assets.get(&vrm_handle.0) {
        println!("Processing VRM model...");

        let model = extract_meshes(vrm_asset);

        for mesh in model.meshes {
            let mesh_handle = meshes.add(Mesh::from(shape::Cube { size: 1.0 })); // Placeholder for VRM mesh data
            let material_handle = materials.add(StandardMaterial::default());

            // Spawn 3D model into Bevy's scene
            commands.spawn(PbrBundle {
                mesh: mesh_handle,
                material: material_handle,
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..Default::default()
            });
        }

        println!("VRM Model Loaded!");
    }
}
