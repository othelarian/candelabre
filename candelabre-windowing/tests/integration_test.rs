use glutin::event_loop::EventLoop;
use candelabre_windowing::*;

#[test]
fn create_window() -> Result<(), String> {
    let el = EventLoop::new();
    match CandlSurface::new(
        &el,
        CandlDimension::Classic(800, 400),
        "test candelabre window",
        CandlOptions::default()
    ) {
        Ok(_) => Ok(()),
        Err(e) => Err(String::from(format!("{}", e)))
    }
}

#[test]
fn create_window_manager() {
    //
    //
    //
}

#[test]
fn open_multi_windows() {
    //
    //
    //
}

#[test]
fn swap_between_windows() {
    //
    //
    //
}