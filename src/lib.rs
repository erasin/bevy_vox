mod loader;
pub use loader::*;

use bevy_app::prelude::*;
use bevy_asset::AddAsset;
use bevy_render::mesh::Mesh;

/// Adds support for Vox file loading to Apps
#[derive(Default)]
pub struct VoxPlugin;

impl Plugin for VoxPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_asset_loader::<Mesh, VoxLoader>();
    }
}
