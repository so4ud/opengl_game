use cgmath::{self, Vector3};
use glium::*;

#[derive(Copy, Clone)]
pub struct RenderReadyVertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}
implement_vertex!(RenderReadyVertex, position, tex_coords);

struct Vertex {
    position: Vector3<f32>,
}

struct Traingle {
    vertices: Vector3<Vertex>,
    normal: Vector3<f32>,
}

impl Vertex {
    pub fn to_reader_ready(self, tex_coords: [f32; 2]) -> RenderReadyVertex {
        RenderReadyVertex {
            position: [self.position.z, self.position.y, self.position.x],
            tex_coords,
        }
    }
}

pub fn prepare_vertecies(
    cam_position: Vector3<f32>,
    cam_rotation: Vector3<f32>,
    trinalges: Vec,
) -> RenderReadyVertex {
}
