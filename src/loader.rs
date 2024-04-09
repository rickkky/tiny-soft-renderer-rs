use crate::mesh::{Mesh, Vertex};
use nalgebra::Vector4;

pub struct ModelRaw {
    pub document: gltf::Document,

    pub buffers: Vec<gltf::buffer::Data>,

    pub images: Vec<gltf::image::Data>,
}

pub fn load_gltf(path: &str) -> ModelRaw {
    let (document, buffers, images) = gltf::import(path).unwrap();
    ModelRaw {
        document,
        buffers,
        images,
    }
}

pub fn load_meshes(model: ModelRaw) -> Vec<Mesh> {
    let mut meshes = Vec::new();
    for mesh in model.document.meshes() {
        let mut vertices = Vec::new();
        for primitive in mesh.primitives() {
            if primitive.mode() != gltf::mesh::Mode::Triangles {
                panic!("Only support triangles.");
            }

            let reader = primitive.reader(|buffer| Some(&model.buffers[buffer.index()]));

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
