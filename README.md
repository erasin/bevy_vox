Load MagicaVoxel Vox file for [bevy](https://github.com/bevyengine/bevy/) engine.




**Example**

```rust
 use bevy::prelude::*;
use bevy_vox::*;

fn main() {
    App::build()
        .add_default_plugins()
        .add_plugin(VoxPlugin)
        .add_startup_system(setup.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // add entities to the world
    commands
        // mesh
        .spawn(PbrComponents {
            // load a mesh from vox
            mesh: asset_server.load("assets/2x2x2.vox").unwrap(),
            // create a material for the mesh
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..Default::default()
        })
        // light
        .spawn(LightComponents {
            transform: Transform::from_translation(Vec3::new(10.0, -3.0, -8.0)),
            ..Default::default()
        })
        // camera
        .spawn(Camera3dComponents {
            transform: Transform::new(Mat4::face_toward(
                Vec3::new(6.0, -6.0, 6.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
            )),
            ..Default::default()
        });
}
```