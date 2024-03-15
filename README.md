Load MagicaVoxel Vox file for [bevy](https://github.com/bevyengine/bevy/) engine.

| bevy_vox | bevy  |
| -------- | ----- |
| 0.8      | 0.12  |
| 0.7      | 0.10  |
| 0.6      | 0.9   |
| 0.5      | 0.8   |
| 0.4      | 0.6   |
| 0.3      | 0.5   |
| 0.2      | 0.4   |

**Example**

```rust
use bevy::prelude::*;
use bevy_vox::VoxPlugin;

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins)
        .add_plugins(VoxPlugin::default())
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 0.5,
        })
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // add entities to the world
    commands.spawn(SceneBundle {
        scene: asset_server.load("2x2x2.vox"),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 3_000_000.0,
            ..Default::default()
        },
        transform: Transform::from_xyz(3.0, -3.5, 4.5),
        ..Default::default()
    });

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(6.0, -6.0, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}
```