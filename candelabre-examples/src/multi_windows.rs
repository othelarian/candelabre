//! This example show how to use candelabre-windowing lib to generate multiple
//! windows and play with them. ???

use candelabre_windowing::{
    CandlCurrentWrapper, CandlDimension, CandlManager,
    CandlOptions, CandlSurface
};
use glutin::event_loop::EventLoop;

mod utils;
use utils::{FS, VS};

fn main() {
    let el = EventLoop::new();
    //
    let mut win_manager = CandlManager::new();
    //
    win_manager.create_window(
        &el,
        CandlDimension::Classic(800, 400),
        "test",
        CandlOptions::default()
    );
    //
    /*
    let surface = CandlSurface::new(
        &el,
        //
    )
    */
    //
    //
    //
    el.run(move |evt, _, ctrl_flow| {
        //
        //
    });
}
