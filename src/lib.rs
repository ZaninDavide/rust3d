use glium::glutin;

pub fn init_context() -> (glium::Display, glutin::event_loop::EventLoop<()>) {
    let event_loop = glutin::event_loop::EventLoop::new(); // WARN: was mutable oin the guide
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    (display, event_loop)
}

pub fn init_program(display: &glium::Display) -> glium::Program {
    let vertex_shader = include_str!("shaders/vertex.shader");
    let fragment_shader = include_str!("shaders/fragment.shader");
    let program =
        glium::Program::from_source(display, vertex_shader, fragment_shader, None).unwrap();

    program
}
