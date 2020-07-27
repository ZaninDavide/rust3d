// #[macro_use]
extern crate glium;

use glium::glutin;
use scene::Vertex;

mod scene;
use scene::Scene;

fn main() {
    // init
    let (display, event_loop) = opengl::init_context();
    let program = opengl::init_program(&display);

    let mut scene = Scene::new();

    // mesh
    scene.set_vertices(vec![
        Vertex::new(-0.5, -0.5, 0.0),
        Vertex::new(-0.5, 0.5, 0.0),
        Vertex::new(0.5, 0.5, 0.0),
        Vertex::new(0.5, -0.5, 0.0),
    ]);
    scene.set_indices(vec![0, 1, 2, 0, 2, 3]);
    scene.set_uniforms(glium::uniform! {
        u_color: [1.0, 1.0, 1.0] as [f32; 3]
    });

    // keep the window open and get events
    event_loop.run(move |ev, _, control_flow| {
        // see https://docs.rs/winit/0.22.2/winit/

        // wait for the next frame
        *control_flow = glutin::event_loop::ControlFlow::Wait; // ControlFlow::Poll;

        // here we receive events
        match ev {
            // windows related events
            glutin::event::Event::WindowEvent { event, .. } => match event {
                // the window will be closed
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                // other events
                _ => return,
            },
            glutin::event::Event::MainEventsCleared => {
                // draw scene
                scene.draw(&display, &program);
            }
            // other kinds of events
            _ => (),
        }
    });
}
