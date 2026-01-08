use core::f32;

use super::render::*;
use cgmath::{self, Vector3, vec3};

// let fov_y = (2.0 * (((fov_x * 0.5).to_radians()).tan() / (16.0 / 9.0)).atan()).to_degrees();

fn rotate(shit: Vec<Vertex>, angles: Vector3<f32>, center: Vector3<f32>) -> Vec<Vertex> {
    shit.iter().map(|v| v.rotate(angles, center)).collect()
}

/// frustum to like opengl idk
fn convert_perspective(shit: Vec<Vertex>, camera: Camera) -> Vec<Vertex> {
    shit
}

/// culling
fn cull(shit: Vec<Vertex>, camera: Camera) -> Vec<Vertex> {
    shit
}

/// # render pipeline
/// 1. realtive to the camera (cam.pos - thing.pos or reversed idk)
/// 2. rotation
/// 3. culling
/// 4. perspective conversion
pub fn thing(vertexes: Vec<Vertex>, camera: Camera) -> Vec<RenderReadyVertex> {
    let vertexes: Vec<Vertex> = vertexes
        .iter()
        .map(|v| Vertex {
            position: camera.position - v.position,
            tex_coords: v.tex_coords,
        })
        .collect();

    let vertexes = rotate(vertexes, camera.rotation, vec3(0.0, 0.0, 0.0));
    let vertexes = cull(vertexes, camera);
    let vertexes = convert_perspective(vertexes, camera);

    vertexes
        .iter()
        .map(|v| v.to_reader_ready(v.tex_coords))
        .collect()
}

///  implied angle is 90 degress
fn shoulder_from(side: f32, angle: f32) -> f32 {
    let a: f32;
    let c = side;

    // angles
    let aang = angle;
    let bang: f32 = 90.0;
    let cang = 180.0 - bang - aang;

    a = c * aang.to_radians().sin() / cang.to_radians().sin();
    return a;
}
