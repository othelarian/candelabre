use glutin::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use candelabre_windowing::{CandlDimension, CandlOptions, CandlSurface};

fn main() {
    let el = EventLoop::new();
    let _surface = CandlSurface::new(
        &el,
        CandlDimension::Classic(800, 400),
        "Candelabre example - Simple window",
        CandlOptions::default()
    ).unwrap();
    el.run(move |evt, _, ctrl_flow| {
        match evt {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                //WindowEvent::Resized(physical_size) =>
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    input: KeyboardInput {
                        state: ElementState::Released,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    }, ..
                } => *ctrl_flow = ControlFlow::Exit,
                //
                //
                _ => ()
            },
            Event::RedrawRequested(_) => {
                //
                //
            },
            _ => ()
        }
    });
}
