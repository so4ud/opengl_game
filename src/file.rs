use super::render::*;

pub fn shitbutt(dick: Vec<Vertex>, cam: Camera) -> Vec<Vertex> {
    let mut result = vec![];
    result.reserve(dick.len());

    for i in 0..result.len() {
        result[i] = dick[i];
        result[i].position = cam.position - result[i].position;
    }

    return result;
}
