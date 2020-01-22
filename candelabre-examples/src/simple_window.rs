//! This example show how to use the candelabre-windowing lib with only the
//! CandlSurface to create a single window. The window must show a triangles,
//! and can be resized efficiently. Use 'ESC' to quit and 'SPACE' to change
//! the name of the window.
//! 
//! This example is a modified version of the luminance hello_world.rs example
//! https://github.com/phaazon/luminance-rs/tree/master/luminance-examples

use candelabre_windowing::{
    CandlCurrentWrapper, CandlDimension,
    CandlOptions, CandlSurfaceBuilder
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

mod utils;
use utils::{
    FS, OGL_TRIANGLE, VS,
    Semantics, SurfaceData, SurfaceState,
    get_closure
};

fn main() {
    let el = EventLoop::new();

    let program = Program::<Semantics, (), ()>::from_strings(None, VS, None, FS)
        .expect("program creation")
        .ignore_warnings();

    let mut surface = CandlSurfaceBuilder::new()
        .dim(CandlDimension::Classic(800, 400))
        .title("Candelabre example - Simple window")
        .options(CandlOptions::default())
        .render_closure(get_closure())
        .render_data(SurfaceData::new())
        .state(SurfaceState::default())
        .build(&el)
        .unwrap();

    let tess = TessBuilder::new(&mut surface)
        .add_vertices(OGL_TRIANGLE)
        .set_mode(Mode::Triangle)
        .build()
        .unwrap();

    surface.rdr_data_mut().update(tess);

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
                WindowEvent::KeyboardInput {
                    input: KeyboardInput {
                        state: ElementState::Released,
                        virtual_keycode: Some(VirtualKeyCode::A),
                        ..
                    }, ..
                } => {
                    //
                    // TODO : change the name of the window
                    //
                    //
                }
                _ => ()
            }
            Event::MainEventsCleared => {
                //
                // TODO : mark the window who need a redraw
                //
            }
            Event::RedrawRequested(_) => {
                let back_buffer = surface.back_buffer().unwrap();
                surface.pipeline_builder().pipeline(
                    &back_buffer,
                    &PipelineState::default(),
                    //
                    |_, _| ()
                    //
                    //surface.render_closure()
                    /*
                    |_, mut shd_gate| {
                        shd_gate.shade(&program, |_, mut rdr_gate| {
                            rdr_gate.render(&RenderState::default(), |mut tess_gate| {
                                //
                                //
                                //tess_gate.render(&tess);
                                //
                                //
                            });
                        });
                    }
                    */
                );
                surface.swap_buffers();
            },
            _ => ()
        }
    });
}
