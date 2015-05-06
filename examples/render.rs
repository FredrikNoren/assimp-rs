extern crate assimp;
extern crate cgmath;
#[macro_use]
extern crate glium;
extern crate glutin;

use assimp::{Importer, LogStream};

use cgmath::{perspective, Matrix4, deg, Vector3, Point3};
use glutin::{Api, GlRequest};
use glium::{DisplayBuild, Surface};

macro_rules! default {
    ($i:ident { $($k:ident: $v:expr),* }) => (
        $i { $($k: $v),* , ..Default::default() }
    )
}

fn main() {
    let display = glutin::WindowBuilder::new()
        .with_depth_buffer(24)
        .with_gl(GlRequest::Specific(Api::OpenGl, (3, 0)))
        .build_glium()
        .unwrap();

    #[derive(Copy, Clone, Debug)]
    struct Vertex3 {
        position: [f32; 3],
        normal: [f32; 3]
    }
    implement_vertex!(Vertex3, position, normal);

    // Setup logging
    LogStream::set_verbose_logging(true);
    let mut log_stream = LogStream::stdout();
    log_stream.attach();

    // Load shaders
    let program = program!(&display,
        130 => {
            vertex: "
                #version 130

                uniform mat4 persp_matrix;
                uniform mat4 view_matrix;

                in vec3 position;
                in vec3 normal;

                out vec3 v_normal;

                void main() {
                    v_normal = normal;
                    gl_Position = persp_matrix * view_matrix * vec4(position * 0.005, 1.0);
                }
            ",

            fragment: "
                #version 130

                in vec3 v_normal;
                out vec4 f_color;

                void main() {
                    f_color = vec4(v_normal, 1.0);
                }
            ",
        }
    ).unwrap();

    let mut importer = Importer::new();
    importer.triangulate(true);
    let scene = importer.read_file("examples/spider.obj").unwrap();

    let mut vertex_buffers = Vec::new();
    let mut index_buffers = Vec::new();

    for mesh in scene.meshes() {
        // Normals iterator should always be the same length as vertices iterator
        let normals = mesh.normals();
        let verts: Vec<Vertex3> = mesh.vertices().iter().enumerate().map(|(i, &vert)|
            Vertex3 { position: vert.into(), normal: normals[i].into() }
        ).collect();

        // Create vertex buffer
        let vb = glium::VertexBuffer::new(&display, verts);
        vertex_buffers.push(vb);

        // Safe to assume all faces are triangles due to import options
        let mut indices = Vec::with_capacity(mesh.num_faces() as usize * 3);
        for face in mesh.faces() {
            indices.push(face[0]);
            indices.push(face[1]);
            indices.push(face[2]);
        }

        let ib = glium::IndexBuffer::new(&display, glium::index::TrianglesList(indices));
        index_buffers.push(ib);
    }

    // Setup perspective camera
    let eye = Point3::new(0.0, 1.0, 3.0);
    let pos = Point3::new(0.0, 0.0, 0.0);
    let up = Vector3::new(0.0, 1.0, 0.0);
    let persp_matrix: Matrix4<f32> = cgmath::perspective(deg(60.0), 1.333, 0.1, 1000.0);
    let view_matrix = Matrix4::look_at(&eye, &pos, &up);

    let uniforms = uniform! {
        persp_matrix: persp_matrix,
        view_matrix: view_matrix
    };

    // Main loop
    while !display.is_closed() {
        // "process" all events
        for _ in display.poll_events() { }

        let mut target = display.draw();
        target.clear_color_and_depth((0.1, 0.1, 0.1, 1.0), 1.0);

        let params = glium::DrawParameters {
            depth_test: glium::DepthTest::IfLess,
            depth_write: true,
            ..std::default::Default::default()
        };

        for i in 0..vertex_buffers.len() {
            target.draw(&vertex_buffers[i],
                        &index_buffers[i],
                        &program,
                        &uniforms,
                        &params).unwrap();
        }

        target.finish();
    }
}
