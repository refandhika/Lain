// use bevy::prelude::*;
// use bevy_vrm::loader::Vrm;

// #[derive(Default)]
// pub struct Model {
//     pub meshes: Vec<MeshData>,
// }

// #[derive(Default)]
// pub struct MeshData {
//     pub vertices: Vec<[f32; 3]>,
//     pub indices: Vec<u32>,
// }

// pub fn load_vrm(asset_server: &AssetServer, path: &str) -> Handle<Vrm> {
//     asset_server.load(path)
// }

// pub fn extract_meshes(vrm_asset: &Vrm) -> Model {
//     let mut meshes = Vec::new();

//     if let Some(skin) = vrm_asset.skins.first() {
//         for primitive in &skin.mesh.primitives {
//             let mut vertices = Vec::new();
//             let mut indices = Vec::new();

//             if let Some(position) = primitive.get_positions() {
//                 vertices = position.iter().map(|&p| [p[0], p[1], p[2]]).collect();
//             }

//             if let Some(index_data) = primitive.get_indices() {
//                 indices = index_data.clone();
//             }

//             meshes.push(MeshData { vertices, indices });
//         }
//     }

//     Model { meshes }
// }