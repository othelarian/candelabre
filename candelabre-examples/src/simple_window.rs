//! This example show how to use the candelabre-windowing lib with only the
//! CandlSurface to create a single window. The window must show a triangles,
//! and can be resized efficiently. Use 'ESC' to quit, 'SPACE' to define
//! randomly a new clear color (background color of the context), 'C' to
//! randomly generate a new color for the triangle , and 'A' to
//! change the name of the window.

use candelabre_windowing::{
    CandlDimension, CandlOptions, CandlRenderer,
    CandlSurfaceBuilder, CandlWindow
};
use candelabre_windowing::glutin::event::{
    ElementState, Event, KeyboardInput, StartCause,
    VirtualKeyCode, WindowEvent
};
use candelabre_windowing::glutin::event_loop::{ControlFlow, EventLoop};

mod utils;
use utils::{DemoSurface, Message, SurfaceState, SurfaceDrawer};

const TITLES_LIST: [&str; 5] = [
    "Candelabre example - Simple window",
    "The first example of candelabre",
    "The purpose of this example is to show",
    "the usage of candelabre libs",
    "from OpenGL to candelabre ;-)"
];

fn main() {
    let el = EventLoop::new();
    let mut surface: DemoSurface = CandlSurfaceBuilder::new()
        .dim(CandlDimension::Classic(800, 400))
        .title(TITLES_LIST[0])
        .options(CandlOptions::default())
        .render(SurfaceDrawer::init())
        .state(SurfaceState::default())
        .video_mode(el.primary_monitor().video_modes().next().unwrap())
        .build(&el)
        .unwrap();

    el.run(move |evt, _, ctrl_flow| {
        *ctrl_flow = ControlFlow::Wait;
        match evt {
            Event::LoopDestroyed => return,
            Event::NewEvents(StartCause::Init) =>
                *ctrl_flow = ControlFlow::Wait,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical_size) => surface.resize(physical_size),
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
                        virtual_keycode: Some(keycode),
                        ..
                    }, ..
                } => match keycode {
                    VirtualKeyCode::Space => {
                        surface.update(Message::RandomBgColor);
                        surface.ask_redraw();
                    }
                    VirtualKeyCode::A => {
                        surface.update(Message::IncValue);
                        surface.title(TITLES_LIST[surface.state().get_value() as usize]);
                    }
                    VirtualKeyCode::C => {
                        surface.update(Message::RandomTriangleColor);
                        surface.ask_redraw();
                    }
                    _ => ()
                }
                _ => ()
            }
            Event::MainEventsCleared => {
                if surface.check_redraw() { surface.request_redraw(); }
            }
            Event::RedrawRequested(_) => surface.draw(),
            _ => ()
        }
    });
}
