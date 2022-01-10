mod loader;
pub use loader::*;

use bevy_app::prelude::*;
use bevy_asset::AddAsset;

/// Adds support for Vox file loading to Apps
#[derive(Default)]
pub struct VoxPlugin;

impl Plugin for VoxPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset_loader::<VoxLoader>();
    }
}
