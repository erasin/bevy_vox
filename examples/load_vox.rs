use bevy::prelude::*;
use bevy_vox::VoxPlugin;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(VoxPlugin::default())
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 0.5,
        })
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // add entities to the world
    commands.spawn_bundle(SceneBundle {
        scene: asset_server.load("2x2x2.vox"),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });

    // light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(3.0, 1.2, 2.5),
        ..Default::default()
    });

    // camera
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(6.0, -6.0, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}
