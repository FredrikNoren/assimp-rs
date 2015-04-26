extern crate assimp_sys;
extern crate cgmath;
#[macro_use]
extern crate glium;
extern crate glutin;
extern crate libc;

use assimp_sys::*;
use cgmath::{perspective, Matrix4, deg, Vector3, Point3};
use glutin::{Api, GlRequest};
use std::ptr;
use std::ffi::CString;
use libc::c_uint;

fn main() {
    use glium::{DisplayBuild, Surface};

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

    // Log assimp errors to stdout
    unsafe {
        let stream = aiGetPredefinedLogStream(AiDefaultLogStream::StdOut, ptr::null());
        aiAttachLogStream(&stream);
    }

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

    // Load mesh from file
    let cstr = CString::new("examples/spider.obj").unwrap().as_ptr();
    let scene = unsafe { aiImportFile(cstr, AIPROCESS_TRIANGULATE) };

    let mut vertex_buffers = Vec::new();
    let mut index_buffers = Vec::new();

    if !scene.is_null() {
        // TODO: replace all of this madness with safe code when higher level API is written
        unsafe {
            let mesh_count = (*scene).num_meshes as usize;
            vertex_buffers.reserve(mesh_count);
            index_buffers.reserve(mesh_count);

            for i in 0..mesh_count {
                let p = (*scene).meshes;
                let ref mesh = *(*p.offset(i as isize));

                let mut verts = Vec::with_capacity(mesh.num_vertices as usize);
                for j in 0..mesh.num_vertices {
                    let v = *mesh.vertices.offset(j as isize);
                    let n = *mesh.normals.offset(j as isize);
                    verts.push(Vertex3 {
                        position: [v.x, v.y, v.z],
                        normal: [n.x, n.y, n.z]
                    });
                }

                let vb = glium::VertexBuffer::new(&display, verts);
                vertex_buffers.push(vb);

                // Safe to assume all faces are triangles due to triangulate option specified
                // in scene loading
                let mut indices = Vec::<c_uint>::with_capacity(mesh.num_faces as usize * 3);
                for j in 0..mesh.num_faces {
                    let ref f = *mesh.faces.offset(j as isize);
                    indices.push(*f.indices.offset(0));
                    indices.push(*f.indices.offset(1));
                    indices.push(*f.indices.offset(2));
                }

                let ib = glium::IndexBuffer::new(&display, glium::index::TrianglesList(indices));
                index_buffers.push(ib);
            }
        }
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

    // Release assimp resources
    unsafe {
        aiReleaseImport(scene);
        aiDetachAllLogStreams();
    }
}
