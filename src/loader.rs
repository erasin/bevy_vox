use anyhow::Result;
use bevy_asset::{AssetLoader, AssetPath, LoadContext, LoadedAsset};
use bevy_ecs::{World, WorldBuilderSource};
use bevy_math::Vec3;
use bevy_pbr::prelude::{PbrBundle, StandardMaterial};
use bevy_render::{
    color::Color,
    mesh::{shape::Cube, Mesh},
};
use bevy_scene::Scene;
use bevy_transform::{
    hierarchy::BuildWorldChildren,
    prelude::{GlobalTransform, Transform},
};
use bevy_utils::BoxedFuture;
use dot_vox::DotVoxData;
use thiserror::Error;

#[derive(Default)]
pub struct VoxLoader;

impl AssetLoader for VoxLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<()>> {
        Box::pin(async move { Ok(load_vox(bytes, load_context).await?) })
    }

    fn extensions(&self) -> &[&str] {
        static EXTENSIONS: &[&str] = &["vox"];
        EXTENSIONS
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
) -> Result<(), VoxError> {
    let mut world = World::default();
    let world_builder = &mut world.build();

    let data: DotVoxData = match dot_vox::load_bytes(&bytes) {
        Ok(d) => d,
        Err(e) => {
            return Err(VoxError::FailErr(e.to_string()));
        }
    };

    let size = 1.0;

    let mut color_use: Vec<usize> = Vec::new();

    for model in data.models.iter() {
        for vox in model.voxels.iter() {
            let index = vox.i as usize;
            if !color_use.contains(&index) {
                color_use.push(index);
            }
        }
    }

    for (index, palette) in data.palette.iter().enumerate() {
        if color_use.contains(&index) {
            let color = palette_to_color(*palette);
            let palette_label = palette_label(index);
            load_context.set_labeled_asset(
                &palette_label,
                LoadedAsset::new(StandardMaterial {
                    albedo: color,
                    ..Default::default()
                }),
            );
        }
    }

    let mesh: Mesh = Mesh::from(Cube { size });
    load_context.set_labeled_asset("cube", LoadedAsset::new(mesh));

    for model in data.models.iter() {
        world_builder
            .spawn((Transform::default(), GlobalTransform::default()))
            .with_children(|parent| {
                for vox in model.voxels.iter() {
                    let vox_asset_path = AssetPath::new_ref(load_context.path(), Some("cube"));

                    let material_label = palette_label(vox.i as usize);
                    let material_asset_path =
                        AssetPath::new_ref(load_context.path(), Some(&material_label));

                    parent.spawn(PbrBundle {
                        mesh: load_context.get_handle(vox_asset_path),
                        material: load_context.get_handle(material_asset_path),
                        transform: Transform::from_translation(Vec3::new(
                            vox.x as f32,
                            vox.y as f32,
                            vox.z as f32,
                        )),
                        ..Default::default()
                    });
                }
            });
    }

    load_context.set_default_asset(LoadedAsset::new(Scene::new(world)));

    Ok(())
}

fn palette_label(index: usize) -> String {
    format!("palette{}", index)
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
