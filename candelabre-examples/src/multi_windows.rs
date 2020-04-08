//! This example show how to use candelabre-windowing lib to generate multiple
//! windows and play with them. Here how to use it:
//! 
//! * 'ESC' to close a window
//! * 'A' to add a new window
//! * 'SPACE' to generate randomly a new background color for the current window

use candelabre_experiment::CandlGraphics;
use candelabre_windowing::{
    CandlDimension, CandlManager, CandlOptions,
    CandlRenderer, CandlSurface, CandlWindow
};
use glutin::event::{
    ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent
};
use glutin::event_loop::{ControlFlow, EventLoop};

mod utils;
use utils::{SurfaceDrawer, SurfaceState, Message};

type Graphics = CandlGraphics<SurfaceDrawer, SurfaceState, Message, ()>;

type Surface = CandlSurface<Graphics, SurfaceState, Message>;

fn add_win(
    manager: &mut CandlManager<Surface, u32>,
    el: &EventLoop<()>,
    title: &str,
) {
    manager.create_window_with_state(
        &el,
        el.primary_monitor().video_modes().next().unwrap(),
        CandlDimension::Classic(800, 400),
        title,
        CandlOptions::default(),
        CandlGraphics::init(),
        SurfaceState::default()
    ).unwrap();
}

fn main() {
    let el = EventLoop::new();
    let mut win_manager = CandlManager::new_with_state(0);

    // first window
    add_win(&mut win_manager, &el, "first window");

    //
    //
    println!("empty? {}", win_manager.is_empty());
    println!("window ids: {:?}", win_manager.list_window_ids());
    //

    el.run(move |evt, _, ctrl_flow| {
        match evt {
            Event::LoopDestroyed => return,
            Event::WindowEvent {event, window_id} => match event {
                WindowEvent::Resized(physical_size) =>
                    win_manager.get_current(window_id).unwrap().resize(physical_size),
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
            Event::RedrawRequested(_win_id) => {
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
