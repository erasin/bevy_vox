use anyhow::Result;
use bevy_asset::AssetLoader;
use bevy_render::color::Color;
use bevy_render::{
    mesh::{Mesh, VertexAttribute},
    pipeline::PrimitiveTopology,
};
use dot_vox::DotVoxData;
use std::path::Path;
use thiserror::Error;

#[derive(Default)]
pub struct VoxLoader;

impl AssetLoader<Mesh> for VoxLoader {
    fn from_bytes(&self, _asset_path: &Path, bytes: Vec<u8>) -> Result<Mesh> {
        let mesh = load_vox(bytes)?;
        Ok(mesh)
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

fn load_vox(bytes: Vec<u8>) -> Result<Mesh, VoxError> {
    // let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    let data = match dot_vox::load_bytes(&bytes) {
        Ok(d) => d,
        Err(e) => {
            return Err(VoxError::FailErr(e.to_string()));
        }
    };

    load_node(data)
}

// use bevy_render::mesh::shape::Cube;

fn load_node(data: DotVoxData) -> Result<Mesh, VoxError> {
    let size = 0.5;

    let vertices_one = &[
        // top (0., 0., size)
        ([-size, -size, size], [0., 0., size], [0., 0.]),
        ([size, -size, size], [0., 0., size], [size, 0.]),
        ([size, size, size], [0., 0., size], [size, size]),
        ([-size, size, size], [0., 0., size], [0., size]),
        // bottom (0., 0., -size)
        ([-size, size, -size], [0., 0., -size], [size, 0.]),
        ([size, size, -size], [0., 0., -size], [0., 0.]),
        ([size, -size, -size], [0., 0., -size], [0., size]),
        ([-size, -size, -size], [0., 0., -size], [size, size]),
        // right (size, 0., 0.)
        ([size, -size, -size], [size, 0., 0.], [0., 0.]),
        ([size, size, -size], [size, 0., 0.], [size, 0.]),
        ([size, size, size], [size, 0., 0.], [size, size]),
        ([size, -size, size], [size, 0., 0.], [0., size]),
        // left (-size, 0., 0.)
        ([-size, -size, size], [-size, 0., 0.], [size, 0.]),
        ([-size, size, size], [-size, 0., 0.], [0., 0.]),
        ([-size, size, -size], [-size, 0., 0.], [0., size]),
        ([-size, -size, -size], [-size, 0., 0.], [size, size]),
        // front (0., size, 0.)
        ([size, size, -size], [0., size, 0.], [size, 0.]),
        ([-size, size, -size], [0., size, 0.], [0., 0.]),
        ([-size, size, size], [0., size, 0.], [0., size]),
        ([size, size, size], [0., size, 0.], [size, size]),
        // back (0., -size, 0.)
        ([size, -size, size], [0., -size, 0.], [0., 0.]),
        ([-size, -size, size], [0., -size, 0.], [size, 0.]),
        ([-size, -size, -size], [0., -size, 0.], [size, size]),
        ([size, -size, -size], [0., -size, 0.], [0., size]),
    ];

    let vertices = data.models[0]
        .voxels
        .iter()
        .flat_map(|voxel| {
            vertices_one
                .iter()
                .map(|(p, n, v)| {
                    (
                        [
                            voxel.x as f32 + p[0],
                            voxel.y as f32 + p[1],
                            voxel.z as f32 + p[2],
                        ],
                        *n,
                        *v,
                    )
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    for (position, normal, uv) in vertices.iter() {
        positions.push(*position);
        normals.push(*normal);
        uvs.push(*uv);
    }

    let indices_one: Vec<u32> = vec![
        0, 1, 2, 2, 3, 0, // top
        4, 5, 6, 6, 7, 4, // bottom
        8, 9, 10, 10, 11, 8, // right
        12, 13, 14, 14, 15, 12, // left
        16, 17, 18, 18, 19, 16, // front
        20, 21, 22, 22, 23, 20, // back
    ];

    let indices = data.models[0]
        .voxels
        .iter()
        .enumerate()
        .flat_map(|(i, _voxel)| {
            indices_one
                .iter()
                .map(|indice| i as u32 * 24 + indice)
                .collect::<Vec<_>>()
        })
        .collect();

    let m = Mesh {
        primitive_topology: PrimitiveTopology::TriangleList,
        attributes: vec![
            VertexAttribute::position(positions),
            VertexAttribute::normal(normals),
            VertexAttribute::uv(uvs),
        ],
        indices: Some(indices),
    };

    Ok(m)
}

#[allow(dead_code)]
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
