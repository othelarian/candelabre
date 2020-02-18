//! This example show how to use candelabre-windowing lib to generate multiple
//! windows and play with them. Here how to use it:
//! 
//! * 'ESC' to close a window
//! * 'A' to add a new window
//! * 'SPACE' to generate randomly a new background color for the current window

use candelabre_core::{CandlGraphics, CandlRenderer};
use candelabre_windowing::{
    CandlCurrentWrapper, CandlDimension, CandlManager,
    CandlOptions, CandlSurface
};
use glutin::event::{
    ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent
};
use glutin::event_loop::{ControlFlow, EventLoop};

mod utils;
//use utils::{FS, OGL_TRIANGLE, Semantics, VS};
use utils::{FS, VS};

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
    let mut win_manager = CandlManager::new_with_state(0);

    // first window
    let win_id = win_manager.create_window_with_state(
        &el,
        CandlDimension::Classic(800, 400),
        "first window",
        CandlOptions::default(),
        CandlGraphics::init(),
        Some(WindowData::default())
    ).unwrap();

    //
    //
    println!("empty? {}", win_manager.is_empty());
    println!("window ids: {:?}", win_manager.list_window_ids());
    //

    el.run(move |evt, _, ctrl_flow| {
        match evt {
            Event::LoopDestroyed => return,
            Event::WindowEvent {event, window_id} => match event {
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
                } => win_manager.remove_window(window_id),
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
                //let surface = win_manager.get_current(win_id.clone()).unwrap();
                //
                //
                //
            }
            _ => ()
        }
        if win_manager.is_empty() { *ctrl_flow = ControlFlow::Exit }
        else { *ctrl_flow = ControlFlow::Wait }
    });
}
