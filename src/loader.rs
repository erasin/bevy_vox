use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext},
    color::Color,
    math::Vec3,
    mesh::Mesh,
    pbr::{MeshMaterial3d, StandardMaterial},
    platform::collections::HashSet,
    prelude::{Cuboid, Mesh3d, Visibility, World},
    reflect::TypePath,
    scene::Scene,
    transform::components::Transform,
};
use dot_vox::{DotVoxData, Voxel};
use thiserror::Error;

#[derive(TypePath, Default)]
pub struct VoxLoader {
    /// MagicaVoxel considers Z as the vertical dimension. Setting this to true will use Y as height
    pub swap_yz: bool,
}

impl AssetLoader for VoxLoader {
    type Asset = Scene;
    type Settings = ();
    type Error = VoxError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, VoxError> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        load_vox(&bytes, load_context, self.swap_yz).await
    }

    fn extensions(&self) -> &[&str] {
        &["vox"]
    }
}

#[derive(Error, Debug)]
pub enum VoxError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("Invalid Vox file: {0}")]
    FailErr(String),
}

async fn load_vox<'a, 'b>(
    bytes: &'a [u8],
    load_context: &'a mut LoadContext<'b>,
    swap_yz: bool,
) -> Result<Scene, VoxError> {
    let data: DotVoxData = match dot_vox::load_bytes(bytes) {
        Ok(d) => d,
        Err(e) => {
            return Err(VoxError::FailErr(e.to_string()));
        }
    };

    let colors: Vec<usize> = data
        .models
        .iter()
        .flat_map(|model| {
            model
                .voxels
                .iter()
                .map(|&voxel| voxel.i as usize)
                .collect::<Vec<usize>>()
        })
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    data.palette
        .iter()
        .enumerate()
        .filter(|(index, _palette)| colors.contains(index))
        .for_each(|(index, palette)| {
            let color = VoxColor::from(*palette).into();
            let palette_label = palette_label(index);
            load_context.add_labeled_asset(
                palette_label,
                StandardMaterial {
                    base_color: color,
                    ..Default::default()
                },
            );
        });

    let size = Vec3::splat(1.0);
    load_context.add_labeled_asset("cube".to_owned(), Mesh::from(Cuboid::from_size(size)));

    let mut world = World::default();
    for model in data.models.iter() {
        world
            .spawn((Transform::default(), Visibility::default()))
            .with_children(|parent| {
                for &vox in model.voxels.iter() {
                    let mut vt: VoxelTransform = vox.into();
                    if swap_yz {
                        vt.swap_yz();
                    }

                    parent.spawn((
                        Mesh3d(load_context.get_label_handle("cube".to_owned())),
                        // mesh: mesh,
                        MeshMaterial3d::<StandardMaterial>(
                            load_context.get_label_handle(palette_label(vox.i as usize)),
                        ),
                        Transform::from(vt),
                    ));
                }
            });
    }

    Ok(Scene::new(world))
}

fn palette_label(index: usize) -> String {
    format!("palette{}", index)
}
/// Voxel Color <-----> Bevy Color
struct VoxColor(bevy::color::Srgba);

impl From<dot_vox::Color> for VoxColor {
    fn from(value: dot_vox::Color) -> Self {
        VoxColor(bevy::color::Srgba::rgba_u8(
            value.r, value.g, value.b, value.a,
        ))
    }
}

impl From<VoxColor> for Color {
    fn from(val: VoxColor) -> Self {
        val.0.into()
    }
}

/// Voxel Pos <----> Bevy Transform
struct VoxelTransform {
    x: u8,
    y: u8,
    z: u8,
}

impl VoxelTransform {
    fn swap_yz(&mut self) {
        (self.y, self.z) = (self.z, self.y);
    }
}

impl From<Voxel> for VoxelTransform {
    fn from(value: Voxel) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

impl From<VoxelTransform> for Transform {
    fn from(val: VoxelTransform) -> Self {
        Transform::from_xyz(val.x as f32, val.y as f32, val.z as f32)
    }
}
