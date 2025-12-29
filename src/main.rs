use cgmath::{Matrix3, Rad, vec3};
use glium;
use glium::*;
use image;

use crate::render::rotate3d;

mod render;

fn main() {
    let event_loop = glium::winit::event_loop::EventLoop::builder()
        .build()
        .expect("event loop building");
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("kys >_<")
        .build(&event_loop);

    let image = image::load(
        std::io::Cursor::new(&include_bytes!("sex.png")),
        image::ImageFormat::Png,
    )
    .unwrap()
    .to_rgba8();
    let image_dimensions = image.dimensions();
    let image =
        glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let texture = glium::Texture2d::new(&display, image).unwrap();

    // We've changed our shape to a rectangle so the image isn't distorted.
    let shape = vec![
        render::RenderReadyVertex {
            position: [49.5, 9.5, 0.0],
            tex_coords: [0.0, 0.0],
        },
        render::RenderReadyVertex {
            position: [50.5, 9.5, 0.0],
            tex_coords: [1.0, 0.0],
        },
        render::RenderReadyVertex {
            position: [50.5, 10.5, 0.0],
            tex_coords: [1.0, 1.0],
        },
        render::RenderReadyVertex {
            position: [50.5, 10.5, 0.0],
            tex_coords: [1.0, 1.0],
        },
        render::RenderReadyVertex {
            position: [49.5, 10.5, 0.0],
            tex_coords: [0.0, 1.0],
        },
        render::RenderReadyVertex {
            position: [49.5, 9.5, 0.0],
            tex_coords: [0.0, 0.0],
        },
    ];
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();

    let vertex_shader_src = include_str!("shaders\\vert.glsl");
    let fragment_shader_src = include_str!("shaders\\frag.glsl");
    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    let mut t: f32 = 0.0;

    #[allow(deprecated)]
    event_loop
        .run(move |ev, window_target| {
            match ev {
                glium::winit::event::Event::WindowEvent { event, .. } => match event {
                    glium::winit::event::WindowEvent::CloseRequested => {
                        window_target.exit();
                    }
                    // We now need to render everyting in response to a RedrawRequested event due to the animation
                    glium::winit::event::WindowEvent::RedrawRequested => {
                        // we update `t`
                        t += 0.0002;
                        let x = t.sin() * 0.5;
                        let y = t.sin() * 360.0 / 100.0;

                        let mut target = display.draw();
                        target.clear_color(0.15, 0.15, 0.15, 1.0);
                        let cam_pos: [f32; 3] = [50.0, 10.0, 0.0];
                        let cam_rot: [f32; 3] = [t * 360.0, 0.0, 0.0];

                        let uniforms = uniform! {
                            matrix: [
                                [1.0, 0.0, 0.0, 0.0],
                                [0.0, 1.0, 0.0, 0.0],
                                [0.0, 0.0, 1.0, 0.0],
                                [ 0.0, 0.0, x, 1.0f32],
                            ],
                            tex: &texture,
                            cam_pos: cam_pos,
                            x: y,
                            r_mat_x: [
                                [1.0, 0.0, 0.0],
                                [0.0, cam_rot[0].to_radians().cos(), -cam_rot[0].to_radians().sin()],
                                [0.0,  cam_rot[0].to_radians().sin(), cam_rot[0].to_radians().cos()],
                            ],
                            r_mat_y: [
                                [cam_rot[1].to_radians().cos(), 0.0, cam_rot[1].to_radians().sin()],
                                [0.0, 1.0, 0.0],
                                [-cam_rot[1].to_radians().sin(), 0.0, cam_rot[1].to_radians().cos()],
                            ],
                            r_mat_z: [
                                [cam_rot[2].to_radians().cos(), -cam_rot[2].to_radians().sin(), 0.0],
                                [cam_rot[2].to_radians().sin(), cam_rot[2].to_radians().cos(), 0.0],
                                [0.0, 0.0, 1.0],
                            ]
                        };

                        target
                            .draw(
                                &vertex_buffer,
                                &indices,
                                &program,
                                &uniforms,
                                &Default::default(),
                            )
                            .unwrap();
                        target.finish().unwrap();
                    }
                    // Because glium doesn't know about windows we need to resize the display
                    // when the window's size has changed.
                    glium::winit::event::WindowEvent::Resized(window_size) => {
                        display.resize(window_size.into());
                    }
                    _ => (),
                },
                // By requesting a redraw in response to a AboutToWait event we get continuous rendering.
                // For applications that only change due to user input you could remove this handler.
                glium::winit::event::Event::AboutToWait => {
                    window.request_redraw();
                }
                _ => (),
            }
        })
        .unwrap();
}
