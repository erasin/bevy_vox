mod loader;
pub use loader::*;

use bevy_app::prelude::*;
use bevy_asset::AddAsset;

/// Adds support for Vox file loading to Apps
#[derive(Default)]
pub struct VoxPlugin {
    /// MagicaVoxel considers Z as the vertical dimension. Setting this to true will use Y as height
    pub swap_yz: bool,
}

impl Plugin for VoxPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset_loader(VoxLoader {
            swap_yz: self.swap_yz,
        });
    }
}
