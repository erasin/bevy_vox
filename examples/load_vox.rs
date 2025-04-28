use bevy::prelude::*;
use bevy_vox::VoxPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(VoxPlugin { swap_yz: true })
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 0.5,
            affects_lightmapped_meshes: true,
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
        SceneRoot(asset_server.load("2x2x2.vox")),
        Transform::from_xyz(-1.0, 0.0, 0.0),
        VoxModel,
    ));

    // light
    commands.spawn((
        PointLight {
            intensity: 3_000_000.0,
            ..Default::default()
        },
        Transform::from_xyz(3.0, -3.5, 4.5),
    ));

    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(6.0, -6.0, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn rotate_model(mut query: Query<&mut Transform, With<VoxModel>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        transform.rotation = Quat::from_euler(EulerRot::XYZ, 0.0, time.elapsed_secs(), 0.0);
    }
}
