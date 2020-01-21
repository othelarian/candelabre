//! This example show how to use candelabre-windowing lib to generate multiple
//! windows and play with them. Here how to use it:
//! 
//! * 'ESC' to close a window
//! * 'A' to add a new window
//! * 'SPACE' to generate randomly a new background color for the current window

use candelabre_windowing::{
    CandlCurrentWrapper, CandlDimension, CandlManager,
    CandlOptions, CandlSurface
};
use glutin::event::{
    ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent
};
use glutin::event_loop::{ControlFlow, EventLoop};
use luminance::context::GraphicsContext;
use luminance::pipeline::PipelineState;
use luminance::render_state::RenderState;
use luminance::shader::program::Program;
use luminance::tess::{Mode, Tess, TessBuilder};

mod utils;
use utils::{FS, OGL_TRIANGLE, Semantics, VS};

struct WindowData {
    pub redraw: bool,
    pub bgcol: [f32; 4]
}

impl Default for WindowData {
    fn default() -> Self {
        WindowData { redraw: false, bgcol: [0.0, 0.0, 0.0, 1.0] }
    }
}

fn main() {
    let el = EventLoop::new();
    let mut win_manager = CandlManager::new_with_data(0);
    // only data to compute the state

    let win_id = win_manager.create_window_with_data(
        &el,
        CandlDimension::Classic(800, 400),
        "first window",
        CandlOptions::default(),
        WindowData::default()
    ).unwrap();
    //
    // TODO : set up the WindowData tess for the previously created surface
    //

    let program = Program::<Semantics, (), ()>::from_strings(None, VS, None, FS)
        .expect("program creation")
        .ignore_warnings();

    el.run(move |evt, _, ctrl_flow| {
        match evt {
            Event::LoopDestroyed => return,
            Event::WindowEvent {event, ..} => match event {
                WindowEvent::Resized(physical_size) => {
                    //
                    // TODO : resize the current window
                    //
                }
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    input: KeyboardInput {
                        state: ElementState::Released,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    }, ..
                } => { // close the current window
                    //
                    // TODO : close the current window
                    //
                }
                WindowEvent::KeyboardInput {
                    input: KeyboardInput {
                        state: ElementState::Released,
                        virtual_keycode: Some(VirtualKeyCode::Space),
                        ..
                    }, ..
                } => { // change the background of the current window
                    //
                    // TODO : change randomly the background color
                    //
                }
                WindowEvent::KeyboardInput {
                    input: KeyboardInput {
                        state: ElementState::Released,
                        virtual_keycode: Some(VirtualKeyCode::A),
                        ..
                    }, ..
                } => { // create a new window
                    //
                    // TODO : add a new window
                    //
                }
                _ => ()
            }
            Event::MainEventsCleared => {
                //
                // TODO : mark the window who need a redraw
                //
            }
            Event::RedrawRequested(win_id) => {
                let surface = win_manager.get_current(win_id.clone()).unwrap();
                let back_buffer = surface.back_buffer().unwrap();
                //
                let bgcol = surface.data().bgcol.clone();
                //let tess = surface.data().tess.as_ref().unwrap();
                //
                surface.pipeline_builder().pipeline(
                    &back_buffer,
                    &PipelineState::default().set_clear_color(bgcol),
                    |_, mut shd_gate| {
                        shd_gate.shade(&program, |_, mut rdr_gate| {
                            rdr_gate.render(&RenderState::default(), |mut tess_gate| {
                                //tess_gate.render(tess);
                            });
                        });
                    }
                );
                surface.swap_buffers();
            }
            _ => ()
        }
        if win_manager.is_empty() { *ctrl_flow = ControlFlow::Exit }
        else { *ctrl_flow = ControlFlow::Wait }
    });
}
