use bevy::prelude::*;
use bevy_vox::VoxPlugin;

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins)
        .add_plugins(VoxPlugin { swap_yz: true })
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 0.5,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, rotate_model)
        .run();
}

#[derive(Component)]
struct VoxModel;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // add entities to the world
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("2x2x2.vox"),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        VoxModel,
    ));

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

fn rotate_model(mut query: Query<&mut Transform, With<VoxModel>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        transform.rotation = Quat::from_euler(EulerRot::XYZ, 0.0, time.elapsed_seconds(), 0.0);
    }
}
