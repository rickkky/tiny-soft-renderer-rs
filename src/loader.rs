use crate::mesh::{Mesh, Vertex};
use nalgebra::Vector4;

pub struct ModelRaw {
    pub document: gltf::Document,

    pub buffers: Vec<gltf::buffer::Data>,

    pub images: Vec<gltf::image::Data>,
}

impl ModelRaw {
    pub fn load_gltf(path: &str) -> Self {
        let (document, buffers, images) = gltf::import(path).unwrap();
        ModelRaw {
            document,
            buffers,
            images,
        }
    }

    pub fn read_meshes(self: &Self) -> Vec<Mesh> {
        let mut meshes = Vec::new();
        for mesh in self.document.meshes() {
            let mut vertices = Vec::new();
            for primitive in mesh.primitives() {
                if primitive.mode() != gltf::mesh::Mode::Triangles {
                    panic!("Only support triangles.");
                }

                let reader = primitive.reader(|buffer| Some(&self.buffers[buffer.index()]));

                let positions: Vec<[f32; 3]> = reader.read_positions().unwrap().collect();
                let indices: Vec<u32> = reader.read_indices().unwrap().into_u32().collect();

                for i in indices {
                    let vertex = Vertex {
                        position: Vector4::new(
                            positions[i as usize][0],
                            positions[i as usize][1],
                            positions[i as usize][2],
                            1.0,
                        ),
                        normal: None,
                        textcoord: None,
                        color: None,
                    };

                    vertices.push(vertex);
                }
            }
            meshes.push(Mesh { vertices });
        }
        meshes
    }
}

pub struct Model {
    pub meshes: Vec<Mesh>,
}

impl Model {
    pub fn new(model_raw: ModelRaw) -> Self {
        let meshes = model_raw.read_meshes();
        Model { meshes }
    }

    pub fn load_gltf(path: &str) -> Self {
        let model_raw = ModelRaw::load_gltf(path);
        Model::new(model_raw)
    }
}
