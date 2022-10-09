mod loader;

pub use loader::VoxLoader;

use bevy_app::{App, Plugin};
use bevy_asset::AddAsset;

/// Adds support for Vox file loading to Apps
#[derive(Default)]
pub struct VoxPlugin {
    /// MagicaVoxel considers Z as the vertical dimension. Setting this to true will use Y as height
    pub swap_yz: bool,
}

impl VoxPlugin {
    pub fn swap() -> Self {
        Self { swap_yz: false }
    }
}

impl Plugin for VoxPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset_loader(VoxLoader {
            swap_yz: self.swap_yz,
        });
    }
}
