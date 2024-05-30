use nalgebra::{Vector2, Vector3, Vector4};
use std::collections::HashMap;
use tinyrenderer::{common::color::Color, rasterizer::texture::Texture};

#[derive(Debug)]
pub struct GltfImport {
    pub document: gltf::Document,

    pub buffers: Vec<gltf::buffer::Data>,

    pub images: Vec<gltf::image::Data>,
}

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: Vector4<f32>,

    pub normal: Option<Vector3<f32>>,

    pub color: Option<Color>,

    pub tex_coord: Option<Vector2<f32>>,
}

#[derive(Debug, Clone)]
pub struct Primitive {
    pub vertices: Vec<Vertex>,

    pub texture_index: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct Mesh {
    pub primitives: Vec<Primitive>,
}

pub fn load_meshs(document: &gltf::Document, buffers: &Vec<gltf::buffer::Data>) -> Vec<Mesh> {
    let mut meshs: Vec<Mesh> = Vec::new();

    for mesh in document.meshes() {
        meshs.push(Mesh {
            primitives: load_primitives(mesh, &buffers),
        });
    }

    meshs
}

pub fn load_primitives(mesh: gltf::Mesh, buffers: &Vec<gltf::buffer::Data>) -> Vec<Primitive> {
    let mut primitives: Vec<Primitive> = Vec::new();

    for primitive in mesh.primitives() {
        if primitive.mode() != gltf::mesh::Mode::Triangles {
            panic!("Only support triangles.");
        }

        let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

        let Some(iter) = reader.read_indices() else {
            continue;
        };

        let indices = iter.into_u32();

        let mut positions: Option<Vec<[f32; 3]>> = None;
        let mut normals: Option<Vec<[f32; 3]>> = None;
        let mut colors: Option<Vec<[f32; 3]>> = None;
        let mut tex_coords: Option<Vec<[f32; 2]>> = None;

        for (semantic, _) in primitive.attributes() {
            match semantic {
                gltf::Semantic::Positions => {
                    if let Some(iter) = reader.read_positions() {
                        positions = Some(iter.collect());
                    } else {
                        continue;
                    }
                }
                gltf::Semantic::Normals => {
                    if let Some(iter) = reader.read_normals() {
                        normals = Some(iter.collect());
                    }
                }
                gltf::Semantic::Colors(set) => {
                    if let Some(iter) = reader.read_colors(set) {
                        colors = Some(iter.into_rgb_f32().collect());
                    }
                }
                gltf::Semantic::TexCoords(set) => {
                    if let Some(iter) = reader.read_tex_coords(set) {
                        tex_coords = Some(iter.into_f32().collect());
                    }
                }
                _ => {}
            }
        }

        let positions = if let Some(positions) = positions {
            positions
        } else {
            continue;
        };

        let mut vertices: Vec<Vertex> = Vec::new();

        for i in indices {
            let position = {
                let p = positions[i as usize];
                Vector4::new(p[0], p[1], p[2], 1.0)
            };
            let normal = if let Some(normals) = &normals {
                Some(Vector3::from(normals[i as usize]))
            } else {
                None
            };
            let color = if let Some(colors) = &colors {
                let c = colors[i as usize];
                Some(Color::new(c[0], c[1], c[2], 1.0))
            } else {
                None
            };
            let tex_coord = if let Some(tex_coords) = &tex_coords {
                let t = tex_coords[i as usize];
                Some(Vector2::new(t[0], t[1]))
            } else {
                None
            };
            vertices.push(Vertex {
                position,
                normal,
                color,
                tex_coord,
            });
        }

        let texture_index = if let Some(info) = primitive
            .material()
            .pbr_metallic_roughness()
            .base_color_texture()
        {
            Some(info.texture().index())
        } else {
            None
        };

        primitives.push(Primitive {
            vertices,
            texture_index,
        });
    }

    primitives
}

pub fn load_textures(
    document: &gltf::Document,
    images: &Vec<gltf::image::Data>,
) -> HashMap<usize, Texture<Color>> {
    let mut texture_map = HashMap::new();

    for texture in document.textures() {
        let image = texture.source();
        let data = &images[image.index()];
        let index = texture.index();
        let texture = Texture::<Color>::from(data);
        texture_map.insert(index, texture);
    }

    texture_map
}
