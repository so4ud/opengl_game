use cgmath::{self, Matrix3, Rad, Vector2, Vector3};
use glium::*;

#[derive(Copy, Clone)]
pub struct RenderReadyVertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}
implement_vertex!(RenderReadyVertex, position, tex_coords);

#[derive(Copy, Clone)]
pub struct Camera {
    pub fov: Vector2<f32>,
    pub position: Vector3<f32>,
    pub rotation: Vector3<f32>,
}

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: Vector3<f32>,
    pub tex_coords: [f32; 2],
}

pub struct Triangle {
    vertices: Vector3<Vertex>,
    normal: Vector3<f32>,
}

pub struct Mesh {
    triangles: Vec<Triangle>,
    // one day it will be a texture but not today
    texture: u32,
}

impl Vertex {
    pub fn to_reader_ready(self, tex_coords: [f32; 2]) -> RenderReadyVertex {
        RenderReadyVertex {
            position: [self.position.z, self.position.y, self.position.x],
            tex_coords,
        }
    }

    pub fn rotate(mut self, angles: Vector3<f32>, center: Vector3<f32>) -> Self {
        self.position = rotate3d(self.position, angles, center);
        return self;
    }

    /// no recurtion until mearrige LOLOLOLOLOLOLOLOLOLOLOL
    /// adds `shift_by` to the posiiton member effectivly "shifting" the vertex
    pub fn shift(mut self, shift_by: Vector3<f32>) -> Self {
        self.position = self.position + shift_by;

        return self;
    }

    pub fn empty_w_pos(pos: Vector3<f32>) -> Self {
        Self {
            position: pos,
            tex_coords: [0.0, 0.0],
        }
    }
}

impl Triangle {
    pub fn rotate(mut self, angles: Vector3<f32>, center: Vector3<f32>) -> Self {
        self.vertices.x = self.vertices.x.rotate(angles, center);
        self.vertices.y = self.vertices.y.rotate(angles, center);
        self.vertices.z = self.vertices.z.rotate(angles, center);

        return self;
    }

    pub fn shift(mut self, shift_by: Vector3<f32>) -> Self {
        self.vertices.x = self.vertices.x.shift(shift_by);
        self.vertices.y = self.vertices.y.shift(shift_by);
        self.vertices.z = self.vertices.z.shift(shift_by);

        return self;
    }
}

/// rotate a 3d point around an arbitrarry center in 3 axies, set `center` to 0 0 0 if you wish to rotate around the origin point
pub fn rotate3d(
    mut point: Vector3<f32>,
    angles: Vector3<f32>,
    center: Vector3<f32>,
) -> Vector3<f32> {
    point -= center;

    let mat_x = Matrix3::from_angle_x(Rad {
        0: angles.x.to_radians(),
    });
    let mat_y = Matrix3::from_angle_x(Rad {
        0: angles.y.to_radians(),
    });
    let mat_z = Matrix3::from_angle_x(Rad {
        0: angles.z.to_radians(),
    });

    point = mat_x * mat_y * mat_z * point;

    point += center;

    return point;
}

// finish this idk, make like a negative shift ig
// pub fn prepare_vertecies(
//     cam_position: Vector3<f32>,
//     cam_rotation: Vector3<f32>,
//     mut mesh: Vec<Mesh>,
// ) -> Vec<RenderReadyVertex> {
//     for i in mesh {
//         for x in i.triangles {
//             x.shift();
//         }
//     }

//     return vec![RenderReadyVertex];
// }
