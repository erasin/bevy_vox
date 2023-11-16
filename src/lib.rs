mod loader;

use bevy::prelude::{App, AssetApp, Plugin};
pub use loader::VoxLoader;

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
        app.preregister_asset_loader::<VoxLoader>(&["vox"]);
    }

    fn finish(&self, app: &mut App) {
        app.register_asset_loader(VoxLoader {
            swap_yz: self.swap_yz,
        });
    }
}
