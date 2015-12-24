#[macro_use] extern crate glium;

#[derive(Copy, Clone)]
struct Vert {
    position: [f32; 3],
}

implement_vertex!(Vert, position);


fn setup_buffer(window: &glium::backend::glutin_backend::GlutinFacade) -> glium::vertex::VertexBuffer<Vert> {
    let data = &[
        Vert {
            position: [0.5, -0.5, 0.0],
        },
        Vert {
            position: [0.0, 0.5, 0.0],
        },
        Vert {
            position: [-0.5, -0.5, 0.0],
        },
    ];
    glium::vertex::VertexBuffer::new(window, data).unwrap()
}

fn setup_shaders(window: &glium::backend::glutin_backend::GlutinFacade) -> glium::program::Program {
    let v_shader = " 
        #version 150 
        in vec3 position; 
        void main() { 
            gl_Position = vec4(position, 1.0); 
        } 
    ";
    let f_shader = " 
        #version 150 
        void main() { 
            gl_FragColor = vec4(0.0, 1.0, 0.0, 1.0); 
        } 
    ";
    glium::program::Program::from_source(window, v_shader, f_shader, None).unwrap()
}


fn main() {
    use glium::{DisplayBuild,Surface};

    let window = glium::glutin::WindowBuilder::new()
        .with_dimensions(1080, 720)
        .with_title(format!("Hello world"))
        .build_glium()
        .unwrap();


    let triangle = setup_buffer(&window);
    let program = setup_shaders(&window);
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let mut exit = false;
    loop {
        for event in window.poll_events() {
            use glium::glutin::Event;
            match event {
                Event::Closed => (exit = true),
                _ => ()
            }
        }

        let mut frame = window.draw();
        frame.clear_color(0.0, 0.0, 0.0, 0.0);
        frame.draw(&triangle, &indices, &program, &glium::uniforms::EmptyUniforms,
                            &Default::default()).unwrap();
        frame.finish().unwrap();
        window.swap_buffers().unwrap();


        if exit {
            break;
        }
    }
}
