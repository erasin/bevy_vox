Load MagicaVoxel Vox file for [bevy](https://github.com/bevyengine/bevy/) engine.


| bevy_vox | bevy |
| -------- | ---- |
| 0.4      | 0.6  |
| 0.3      | 0.5  |
| 0.2      | 0.4  |


**Example**

```rust
use bevy::prelude::*;
use bevy_vox::*;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(VoxPlugin)
        .add_startup_system(setup.system())
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // add entities to the world
    commands.spawn_scene(asset_server.load("2x2x2.vox"));
    commands
        // light
        .spawn_bundle(PointLightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 5.0, 4.0)),
            ..Default::default()
        });
    commands
        // camera
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_translation(Vec3::new(6.0, -6.0, 6.0))
                .looking_at(Vec3::default(), Vec3::Y),
            ..Default::default()
        });
}

```