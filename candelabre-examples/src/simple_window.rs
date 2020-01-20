//! This example show how to use the candelabre-windowing lib with only the
//! CandlSurface to create a single window. The window must show a triangles,
//! and can be resized efficiently. Use 'ESC' to quit.
//! 
//! This example is a modified version of the luminance hello_world.rs example
//! https://github.com/phaazon/luminance-rs/tree/master/luminance-examples

use candelabre_windowing::{
    CandlCurrentWrapper, CandlDimension, CandlOptions, CandlSurface
};
use glutin::event::{
    ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent
};
use glutin::event_loop::{ControlFlow, EventLoop};
use luminance::context::GraphicsContext;
use luminance::pipeline::PipelineState;
use luminance::render_state::RenderState;
use luminance::shader::program::Program;
use luminance::tess::{Mode, TessBuilder};
use luminance_derive::{Semantics, Vertex};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Semantics)]
pub enum Semantics {
    #[sem(name = "co", repr = "[f32; 2]", wrapper = "VertexPosition")]
    Position,
    #[sem(name = "color", repr = "[u8; 3]", wrapper = "VertexColor")]
    Color
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Vertex)]
#[vertex(sem = "Semantics")]
struct Vertex {
    pos: VertexPosition,
    #[vertex(normalized = "true")]
    rgb: VertexColor
}

const VS: &'static str = include_str!("../resources/simple-vs.glsl");
const FS: &'static str = include_str!("../resources/simple-fs.glsl");

const OGL_TRIANGLE: [Vertex; 3] = [
    Vertex {pos: VertexPosition::new([-0.5, 0.5]), rgb: VertexColor::new([0, 255, 0])},
    Vertex {pos: VertexPosition::new([-0.0, 0.0]), rgb: VertexColor::new([255, 0, 0])},
    Vertex {pos: VertexPosition::new([0.5, 0.5]), rgb: VertexColor::new([0, 0, 255])}
];

fn main() {
    let el = EventLoop::new();
    let mut surface = CandlSurface::new(
        &el,
        CandlDimension::Classic(800, 400),
        "Candelabre example - Simple window",
        CandlOptions::default()
    ).unwrap();

    let tess = TessBuilder::new(&mut surface)
        .add_vertices(OGL_TRIANGLE)
        .set_mode(Mode::Triangle)
        .build()
        .unwrap();
    let program = Program::<Semantics, (), ()>::from_strings(None, VS, None, FS)
        .expect("program creation")
        .ignore_warnings();

    el.run(move |evt, _, ctrl_flow| {
        match evt {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical_size) => {
                    if let CandlCurrentWrapper::PossiblyCurrent(ctx) = surface.ctx() {
                        ctx.resize(physical_size)
                    }
                }
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    input: KeyboardInput {
                        state: ElementState::Released,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    }, ..
                } => *ctrl_flow = ControlFlow::Exit,
                _ => ()
            },
            Event::RedrawRequested(_) => {
                let back_buffer = surface.back_buffer().unwrap();
                surface.pipeline_builder().pipeline(
                    &back_buffer,
                    &PipelineState::default(),
                    |_, mut shd_gate| {
                        shd_gate.shade(&program, |_, mut rdr_gate| {
                            rdr_gate.render(&RenderState::default(), |mut tess_gate| {
                                tess_gate.render(&tess);
                            });
                        });
                    }
                );
                surface.swap_buffers();
            },
            _ => ()
        }
    });
}
