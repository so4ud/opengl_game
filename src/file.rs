use std::vec;

use super::render::*;
use cgmath::{self, Vector3, vec3};

// 1. realtive to the camera (cam.pos - thing.pos or reversed idk)
// 2. rotation
// 3. perspective conversion
// 4. culling (last)

fn rotate(shit: Vec<Vertex>, angles: Vector3<f32>, center: Vector3<f32>) -> Vec<Vertex> {
    shit.iter().map(|v| v.rotate(angles, center)).collect()
}

/// frustum to like opengl idk
fn convert_perspective(shit: Vec<Vertex>, camera: Camera) -> Vec<Vertex> {}

fn cull(shit: Vec<Vertex>, camera: Camera) -> Vec<Vertex> {
    shit
}

pub fn thing(vertexes: Vec<Vertex>, camera: Camera) -> Vec<RenderReadyVertex> {
    let vertexes: Vec<Vertex> = vertexes
        .iter()
        .map(|v| Vertex {
            position: camera.position - v.position,
            tex_coords: v.tex_coords,
        })
        .collect();

    let vertexes = rotate(vertexes, camera.rotation, vec3(0.0, 0.0, 0.0));
    let vertexes = convert_perspective(vertexes, camera);
    let vertexes = cull(vertexes, camera);

    let fov_x = 90.0;
    #[allow(unused)]
    let fov_y = fov_x / (16.0 / 9.0); // ~ 50.625

    vertexes
        .iter()
        .map(|v| v.to_reader_ready(v.tex_coords))
        .collect()
}
