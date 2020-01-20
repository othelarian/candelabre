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
fn open_multi_windows() -> Result<(), String> {
    let el = EventLoop::new();
    let mut win_manager = CandlManager::new();
    for win_idx in 0..3 {
        &win_manager.create_window(
            &el,
            CandlDimension::Classic(800, 400),
            &format!("test candelabre multi window: #{}", win_idx+1),
            CandlOptions::default()
        ).unwrap();
    }
    let ids = win_manager.list_window_ids();
    for idx in &ids {
        win_manager.get_current(idx.clone()).unwrap();
    }
    for idx in &ids {
        win_manager.remove_window(idx.clone());
    }
    if win_manager.is_empty() { Ok(()) }
    else { Err(String::from("Test failed: CandlManager not empty!")) }
}
