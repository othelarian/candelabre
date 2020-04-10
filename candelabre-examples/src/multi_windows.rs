//! This example show how to use candelabre-windowing lib to generate multiple
//! windows and play with them. Here how to use it:
//! 
//! * 'ESC' to close a window
//! * 'A' to add a new window
//! * 'SPACE' to generate randomly a new background color for the current window
//! * 'C' to randomly change the color of the triangle of the current window

use candelabre_windowing::{
    CandlDimension, CandlManager, CandlOptions,
    CandlRenderer, CandlWindow
};
use glutin::event::{
    ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent
};
use glutin::event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget};
use glutin::monitor::VideoMode;

mod utils;
use utils::{DemoSurface, SurfaceDrawer, SurfaceState, Message};

fn add_win(
    manager: &mut CandlManager<DemoSurface, u32>,
    el: &EventLoopWindowTarget<()>,
    video_mode: VideoMode,
    title_nb: u32
) {
    manager.create_window_with_state(
        &el,
        video_mode,
        CandlDimension::Classic(800, 400),
        &format!("multi window #{}", title_nb),
        CandlOptions::default(),
        SurfaceDrawer::init(),
        SurfaceState::default()
    ).unwrap();
}

fn main() {
    let el = EventLoop::new();
    let video_mode = el.primary_monitor().video_modes().next().unwrap();
    let mut win_manager = CandlManager::new_with_state(1);

    // first window
    add_win(&mut win_manager, &el, video_mode, 0);

    el.run(move |evt, el_wt, ctrl_flow| {
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
                } => {
                    win_manager.remove_window(window_id);
                    if win_manager.is_empty() { *ctrl_flow = ControlFlow::Exit; }
                }
                WindowEvent::KeyboardInput {
                    input: KeyboardInput {
                        state: ElementState::Released,
                        virtual_keycode: Some(keycode),
                        ..
                    }, ..
                } => match keycode {
                    VirtualKeyCode::Space => {
                        let surface = win_manager.get_current(window_id).unwrap();
                        surface.update(Message::RandomBgColor);
                        surface.ask_redraw();
                    }
                    VirtualKeyCode::A => {
                        let video_mode = win_manager
                            .get_current(window_id)
                            .unwrap()
                            .get_window()
                            .unwrap()
                            .primary_monitor()
                            .video_modes()
                            .next()
                            .unwrap();
                        let nb = win_manager.state().clone();
                        {
                            let state = win_manager.state_mut();
                            *state = *state + 1;
                        }
                        add_win(&mut win_manager, &el_wt, video_mode.clone(), nb);
                    }
                    VirtualKeyCode::C => {
                        let surface = win_manager.get_current(window_id).unwrap();
                        surface.update(Message::RandomTriangleColor);
                        surface.ask_redraw();
                    }
                    _ => ()
                }
                _ => ()
            }
            Event::MainEventsCleared => {
                for wid in win_manager.list_window_ids() {
                    let surface = win_manager.get_current(wid).unwrap();
                    if surface.check_redraw() { surface.request_redraw(); }
                }
            }
            Event::RedrawRequested(win_id) =>
                win_manager.get_current(win_id).unwrap().draw(),
            _ => ()
        }
        if win_manager.is_empty() { *ctrl_flow = ControlFlow::Exit }
        else { *ctrl_flow = ControlFlow::Wait }
    });
}
