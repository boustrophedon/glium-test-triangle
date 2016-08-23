#[macro_use] extern crate glium;
#[macro_use] extern crate nalgebra as na;
extern crate obj;

use std::f32::consts::{FRAC_PI_2};

use obj::SimplePolygon;

use na::{Point3, Vector3, Matrix3, Isometry3, PerspectiveMatrix3};
use na::{Eye};

const PI_4: f32 = std::f32::consts::FRAC_PI_4;

#[derive(Copy, Clone)]
struct Vert {
    position: [f32;3],
    normal: [f32;3],
    color: [f32;3],
}

implement_vertex!(Vert, position, normal, color);

fn setup_icosohedron(meshes_dir: String, window: &glium::backend::glutin_backend::GlutinFacade) -> glium::vertex::VertexBuffer<Vert> {
    let mut p = std::path::PathBuf::from(meshes_dir);
    p.push("icosohedron.obj");
    let o = obj::load::<SimplePolygon>(&p).unwrap();

    let green = [0f32, 1.0, 0.0];

    let size = o.position().len();
    let mut data = Vec::with_capacity(size);

    for ob in o.object_iter() {
        for g in ob.group_iter() {
            for tri in g.indices() {
                data.push(
                    Vert {
                        position: o.position()[tri[0].0],
                        normal: o.normal()[(tri[0].2).unwrap()],
                        color: green.clone(),
                    }
                );
                data.push(
                    Vert {
                        position: o.position()[tri[1].0],
                        normal: o.normal()[(tri[1].2).unwrap()],
                        color: green.clone(),
                    }
                );
                data.push(
                    Vert {
                        position: o.position()[tri[2].0],
                        normal: o.normal()[(tri[2].2).unwrap()],
                        color: green.clone(),
                    }
                );
            }
        }
    }

    glium::vertex::VertexBuffer::new(window, &data).unwrap()
}

fn setup_plane(window: &glium::backend::glutin_backend::GlutinFacade) -> glium::vertex::VertexBuffer<Vert> {
    let tr = [1f32, 1.0, 0.0];
    let tl = [-1f32, 1.0, 0.0];
    let bl = [-1f32, -1.0, 0.0];
    let br = [1f32, -1.0, 0.0];

    let n = [0f32, 0.0, 1.0];

    let white = [1f32, 1.0, 1.0];

    let data = &[
        Vert {
            position: tr,
            normal: n,
            color: white,
        },
        Vert {
            position: tl,
            normal: n,
            color: white,
        },
        Vert {
            position: bl,
            normal: n,
            color: white,
        },

        Vert {
            position: tr,
            normal: n,
            color: white,
        },
        Vert {
            position: bl,
            normal: n,
            color: white,
        },
        Vert {
            position: br,
            normal: n,
            color: white,
        },
    ];
    glium::vertex::VertexBuffer::new(window, data).unwrap()
}


fn setup_shaders(window: &glium::backend::glutin_backend::GlutinFacade) -> glium::program::Program {
    let v_shader = "
        #version 150

        uniform mat4 perspective;
        uniform mat4 mv;
        uniform mat4 m_invt;
        uniform vec3 light_dir;

        in vec3 position;
        in vec3 normal;
        in vec3 color;

        out vec3 f_color;

        void main() {
            gl_Position = perspective*mv*vec4(position, 1.0);

            vec3 mv_normal = normalize(vec3(m_invt * vec4(normal, 0.0)));
            float diffuse = 0.6*max( dot(mv_normal, light_dir), 0.1);

            f_color = diffuse*color;
        }
    ";
    let f_shader = "
        #version 150

        in vec3 f_color;

        void main() {
            gl_FragColor = vec4(f_color, 1.0);
        }
    ";
    glium::program::Program::from_source(window, v_shader, f_shader, None).unwrap()
}


fn main() {
    use glium::{DisplayBuild,Surface};


    let meshes_dir = match std::env::args().nth(1) {
        Some(dir) => dir,
        None => {
            println!("Usage: ./triangle [path_to_meshes_directory]");
            return;
        },
    };

    const WIDTH: u32 = 1280;
    const HEIGHT: u32 = 720;
    const ASPECT: f32 = 1.78;

    let params = glium::DrawParameters {
        backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            .. Default::default()
        },
        .. Default::default()
    };

    let window = glium::glutin::WindowBuilder::new()
        .with_dimensions(WIDTH, HEIGHT)
        .with_depth_buffer(24)
        .with_title(format!("Hello world"))
        .build_glium()
        .unwrap();

    let icosohedron = setup_icosohedron(meshes_dir, &window);
    let plane = setup_plane(&window);
    let program = setup_shaders(&window);
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let persp = PerspectiveMatrix3::new(ASPECT, PI_4, 1.0, 100.0).to_matrix();

    let mut position = Point3::new(0.0f32, 1.0f32, 5.0f32);
    let dir = Vector3::new(0.0f32, 0.0f32, -1.0f32);
    let up = Vector3::new(0.0f32, 1.0f32, 0.0f32);

    let mut t = 0f32;

    let mut exit = false;
    loop {
        for event in window.poll_events() {
            use glium::glutin::Event;
			use glium::glutin::VirtualKeyCode as KC;
            match event {
                Event::Closed => (exit = true),
				Event::KeyboardInput(state, _, key) => {
                    if state == glium::glutin::ElementState::Pressed {
                        match key.unwrap() {
                            KC::PageUp => {position = position + Vector3::new(0f32, 0.2f32, 0f32);},
                            KC::PageDown => {position = position + Vector3::new(0f32, -0.2f32, 0f32);},
							KC::Left => position = position + Vector3::new(-0.2f32, 0f32, 0f32),
							KC::Right => position = position + Vector3::new(0.2f32, 0f32, 0f32),
							KC::Up => position = position + Vector3::new(0f32, 0f32, -0.2f32),
							KC::Down => position = position + Vector3::new(0f32, 0f32, 0.2f32),
                            KC::Space => {println!("hello");}
                            KC::Escape => {exit = true;},
                            _ => ()
                        }
                    }
				}
                _ => ()
            }
        }

        t += 0.05;

        let mut frame = window.draw();
        frame.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

        // camera
        let view = Isometry3::look_at_rh(&position, &(position + dir), &up);
        let view = na::to_homogeneous(&view);
        let light_dir = [0.5, 0.866025, 0f32];

        // draw tetrahedrons
        for i in 0..10 {
            let model = Isometry3::new(Vector3::new(0f32, 0.0, -5f32*i as f32), Vector3::new(0f32, 1.0, 0.0) * t);
            let model = na::to_homogeneous(&model);
            let modelview = view*model;

            let uniforms = uniform! {
                perspective: persp.as_ref().clone(),
                mv: modelview.as_ref().clone(),
                m_invt: na::transpose(&na::inverse(&model).unwrap()).as_ref().clone(),
                light_dir: light_dir,
            };

            frame.draw(&icosohedron, &indices, &program, &uniforms,
                                &params).unwrap();
        }

        // draw plane below icosohedron
        let scale = 50f32;
        let plane_scale = na::to_homogeneous(&(Matrix3::<f32>::new_identity(3)*scale));
        let plane_rot = na::to_homogeneous(&(Isometry3::new(Vector3::new(0f32, -1.0, 0.0), Vector3::new(1f32, 0.0, 0.0)*-FRAC_PI_2)));
        let plane_model = plane_rot*plane_scale;
        let modelview = view*plane_model;

        let uniforms = uniform! {
            perspective: persp.as_ref().clone(),
            mv: modelview.as_ref().clone(),
            m_invt: na::transpose(&na::inverse(&plane_model).unwrap()).as_ref().clone(),
            light_dir: light_dir,
        };
        frame.draw(&plane, &indices, &program, &uniforms,
                            &params).unwrap();


        frame.finish().unwrap();
        window.swap_buffers().unwrap();


        if exit {
            break;
        }
    }
}
