use bevy::prelude::*;
use bevy_vox::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(VoxPlugin)
        .add_startup_system(setup.system())
        .run();
}

fn setup(commands: &mut Commands, asset_server: Res<AssetServer>) {
    // add entities to the world
    commands
        .spawn_scene(asset_server.load("2x2x2.vox"))
        // light
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 5.0, 4.0)),
            ..Default::default()
        })
        // camera
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(6.0, -6.0, 6.0))
                .looking_at(Vec3::default(), Vec3::unit_y()),
            ..Default::default()
        });
}
