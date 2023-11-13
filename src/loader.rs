use anyhow::Result;

use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext},
    prelude::{
        shape::Cube, BuildWorldChildren, Color, Mesh, PbrBundle, SpatialBundle, StandardMaterial,
        Transform, World,
    },
    scene::Scene,
    utils::BoxedFuture,
};
use dot_vox::{DotVoxData, DEFAULT_PALETTE};
use thiserror::Error;

#[derive(Default)]
pub struct VoxLoader {
    /// MagicaVoxel considers Z as the vertical dimension. Setting this to true will use Y as height
    pub swap_yz: bool,
}

impl AssetLoader for VoxLoader {
    type Asset = Scene;
    type Settings = ();
    type Error = VoxError;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a Self::Settings,
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, VoxError>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader
                .read_to_end(&mut bytes)
                .await
                .map_err(|e| VoxError::FailErr(e.to_string()))?;
            Ok(load_vox(&bytes, load_context, self.swap_yz).await?)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["vox"]
    }
}

#[derive(Error, Debug)]
pub enum VoxError {
    #[error("Invalid Vox file: {0}")]
    FailErr(String),
}

async fn load_vox<'a, 'b>(
    bytes: &'a [u8],
    load_context: &'a mut LoadContext<'b>,
    swap_yz: bool,
) -> Result<Scene, VoxError> {
    let data: DotVoxData = match dot_vox::load_bytes(&bytes) {
        Ok(d) => d,
        Err(e) => {
            return Err(VoxError::FailErr(e.to_string()));
        }
    };

    let size = 1.0;

    let mut colors: Vec<usize> = Vec::new();
    data.models.iter().for_each(|model| {
        model.voxels.iter().for_each(|vox| {
            let index = vox.i as usize;
            if !colors.contains(&index) {
                colors.push(index);
            }
        });
    });

    for (index, palette) in data.palette.iter().enumerate() {
        if colors.contains(&index) {
            let color = palette_to_color(*palette);
            let palette_label = palette_label(index);

            load_context.add_labeled_asset(
                palette_label,
                StandardMaterial {
                    base_color: color,
                    ..Default::default()
                },
            );
        }
    }

    load_context.add_labeled_asset("cube".to_owned(), Mesh::from(Cube { size }));

    let mut world = World::default();
    for model in data.models.iter() {
        world
            .spawn(SpatialBundle::default())
            .with_children(|parent| {
                for vox in model.voxels.iter() {
                    let (x, y, z) = if swap_yz {
                        (vox.x, vox.z, vox.y)
                    } else {
                        (vox.x, vox.y, vox.z)
                    };

                    parent.spawn(PbrBundle {
                        mesh: load_context.get_label_handle("cube".to_owned()),
                        material: load_context.get_label_handle(palette_label(vox.i as usize)),
                        transform: Transform::from_xyz(x as f32, y as f32, z as f32),
                        ..Default::default()
                    });
                }
            });
    }

    Ok(Scene::new(world))
}

fn palette_label(index: usize) -> String {
    format!("palette{}", index)
}

#[allow(dead_code)]
fn palette_to_colors(palette: Vec<u32>) -> Vec<Color> {
    let ps = if palette.is_empty() {
        DEFAULT_PALETTE.clone()
    } else {
        palette
    };

    ps.iter().map(|p| palette_to_color(*p)).collect()
}

fn palette_to_color(from: u32) -> Color {
    let (a, b, g, r) = (
        from >> 24u32 & 0xFF,
        from >> 16u32 & 0xFF,
        from >> 8u32 & 0xFF,
        from & 0xFF,
    );

    Color::rgba(
        r as f32 / 255.0,
        g as f32 / 255.0,
        b as f32 / 255.0,
        a as f32 / 255.0,
    )
}
