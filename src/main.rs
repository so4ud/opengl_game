use glium;
use glium::*;

fn main() {
    let event_loop = glium::winit::event_loop::EventLoop::builder()
        .build()
        .expect("event loop building");
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("kys >_<")
        .build(&event_loop);

    #[derive(Copy, Clone)]
    struct RenderReadyVertex {
        position: [f32; 3],
        color: [f32; 3],
    }
    implement_vertex!(RenderReadyVertex, position, color);

    let shape = vec![
        RenderReadyVertex {
            position: [-0.5, -0.5, 0.5],
            color: [1.0, 0.0, 0.0],
        },
        RenderReadyVertex {
            position: [0.0, 0.5, 0.5],
            color: [0.0, 1.0, 0.0],
        },
        RenderReadyVertex {
            position: [0.5, -0.25, 0.5],
            color: [0.0, 0.0, 1.0],
        },
    ];
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

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
                        t += 0.02;
                        let x = t.sin() * 0.5;

                        let mut target = display.draw();
                        target.clear_color(0.15, 0.15, 0.15, 1.0);

                        let uniforms = uniform! {
                            matrix: [
                                [1.0, 0.0, 0.0, 0.0],
                                [0.0, 1.0, 0.0, 0.0],
                                [0.0, 0.0, 1.0, 0.0],
                                [  x, 0.0, 0.0, 1.0f32],
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
