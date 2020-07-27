use std::collections::HashMap;

/// Use a scene to store information on the 3D environment and draw it on screen.
/// # Example
/// ```
/// use scene::Vertex;
///
/// let (display, event_loop) = opengl::init_context();
/// let program = opengl::init_program(&display);
///
/// let mut scene = Scene::new();
///
/// scene.set_vertices(vec![
///     Vertex::new(-0.5, -0.5, 0.0),
///     Vertex::new(-0.5, 0.5, 0.0),
///     Vertex::new(0.5, 0.5, 0.0),
///     Vertex::new(0.5, -0.5, 0.0),
/// ]);
///
/// scene.set_indices(vec![0, 1, 2, 0, 2, 3]);
///
/// scene.set_uniforms(glium::uniform! {
///     u_color: [1.0, 1.0, 1.0] as [f32; 3]
/// });
///
/// scene.draw(&display, &program); // the magin behind the scenes... ...
/// ```
pub struct Scene<U: glium::uniforms::Uniforms> {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    vertex_buffer: Option<glium::VertexBuffer<Vertex>>,
    index_buffer: Option<glium::IndexBuffer<u32>>,
    uniforms: Option<U>,
    materials: HashMap<u8, Material>,
    id_counter: u8,
}

impl<U: glium::uniforms::Uniforms> Scene<U> {
    pub fn new() -> Scene<U> {
        Scene {
            vertices: vec![],
            indices: vec![],
            materials: HashMap::new(),
            id_counter: 0,
            vertex_buffer: None,
            index_buffer: None,
            uniforms: None,
        }
    }

    pub fn set_vertices(&mut self, vertices: Vec<Vertex>) {
        self.vertices = vertices;
        self.vertex_buffer = None;
        self.index_buffer = None;
    }

    pub fn set_indices(&mut self, indices: Vec<u32>) {
        self.indices = indices;
        self.vertex_buffer = None;
        self.index_buffer = None;
    }

    pub fn set_materials(&mut self, materials: HashMap<u8, Material>) {
        self.materials = materials;
    }

    pub fn add_material(&mut self, material: Material) {
        self.id_counter += 1;
        self.materials.insert(self.id_counter, material);
    }

    pub fn set_uniforms(&mut self, uniforms: U) {
        self.uniforms = Some(uniforms);
    }

    pub fn get_vertex_buffer(&self, display: &glium::Display) -> glium::VertexBuffer<Vertex> {
        glium::VertexBuffer::new(display, &self.vertices).unwrap()
    }

    pub fn get_index_buffer(&self, display: &glium::Display) -> glium::IndexBuffer<u32> {
        if self.indices.len() > 0 {
            glium::index::IndexBuffer::new(
                display,
                glium::index::PrimitiveType::TrianglesList,
                &self.indices,
            )
            .expect("Error generating the index buffer")
        } else {
            panic!("Missing indices");
        }
    }

    fn update_vertex_buffer(&mut self, display: &glium::Display) {
        if self.vertex_buffer.is_none() {
            self.vertex_buffer = Some(self.get_vertex_buffer(display));
        }
    }

    fn update_index_buffer(&mut self, display: &glium::Display) {
        if self.index_buffer.is_none() {
            self.index_buffer = Some(self.get_index_buffer(display));
        }
    }

    pub fn draw(&mut self, display: &glium::Display, program: &glium::Program) {
        self.update_vertex_buffer(display);
        self.update_index_buffer(display);
        if let Some(vb) = &self.vertex_buffer {
            if let Some(ib) = &self.index_buffer {
                if let Some(un) = &self.uniforms {
                    // let's draw something
                    let mut target = display.draw(); // initialize a new FrameBuffer
                    use glium::Surface;
                    target.clear_color(0.0, 0.0, 1.0, 1.0);
                    target
                        .draw(
                            vb,
                            ib,
                            program,
                            un, // &glium::uniforms::EmptyUniforms
                            &Default::default(),
                        )
                        .unwrap();
                    // draw the FrameBuffer and destroy it
                    target.finish().unwrap();
                } else {
                    panic!("Cannot draw before specifing uniforms");
                }
            } else {
                panic!("Impossible to find any index buffer for this scene");
            }
        } else {
            panic!("Impossible to find any vertex buffer for this scene");
        }
    }
}

pub struct Material {
    diffuse: MaterialField<[f32; 3]>,
    specularity: MaterialField<f32>,
}

impl Material {
    pub fn new(diffuse: MaterialField<[f32; 3]>, specularity: MaterialField<f32>) -> Material {
        Material {
            diffuse,
            specularity,
        }
    }
}

pub struct MaterialField<T> {
    value: Option<T>,
    texture: Option<glium::texture::Texture2d>,
}

impl<T> MaterialField<T> {
    pub fn new(value: T) -> MaterialField<T> {
        MaterialField {
            value: Some(value),
            texture: None,
        }
    }

    pub fn set_value(&mut self, value: T) {
        self.value = Some(value);
    }

    pub fn set_texture(&mut self, texture: glium::texture::Texture2d) {
        self.texture = Some(texture);
    }

    pub fn clear_texture(&mut self) {
        self.texture = None;
    }
}

/// We can expand the definition of a Vertex with other informations: normal, uv, color...
/// All this fields implemented with implement_vertex! will then be accessible in the vertex shader as attributes.
/// If you want to acces them in the fragent shader you need to output the value
/// from the vertex shader and after the automatic interpoletion receive them as an input in the fragment shader
#[derive(Copy, Clone)]
pub struct Vertex {
    pub pos: [f32; 3],
    pub uv: [f32; 2],
    pub material_id: u8,
}

glium::implement_vertex!(Vertex, pos, uv, material_id);

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32) -> Vertex {
        Vertex {
            pos: [x, y, z],
            uv: [0.0, 1.0],
            material_id: 0,
        }
    }
}
