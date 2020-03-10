use candelabre_core::{
    CandlGraphics, CandlGraphicsDrawer, CandlRenderer, CandlUpdate
};
use candelabre_windowing::*;
use glutin::event_loop::EventLoop;

struct CandlFakeDrawer;

impl CandlGraphicsDrawer<CandlNoState, (), ()> for CandlFakeDrawer {
    fn execute(&self, _: Option<&CandlNoState>, _: Option<&()>) {}
}

type CandlFakeGraphics = CandlGraphics<CandlFakeDrawer, CandlNoState, (), ()>;

#[test]
fn use_surface_builder() -> Result<(), String> {
    let el = EventLoop::new();
    let builder = CandlSurfaceBuilder::new()
        .dim(CandlDimension::FullscreenSpecific(900, 700))
        .title("This is a test")
        .options(CandlOptions::default().set_cursor_mode(CursorMode::Invisible))
        .render(CandlFakeGraphics::init())
        .no_state();
    match builder.build(&el) {
        Ok(_) => Ok(()),
        Err(e) => Err(String::from(format!("{}", e)))
    }
}

#[test]
fn create_window() -> Result<(), String> {
    let el = EventLoop::new();
    match <CandlSurface<CandlFakeGraphics, CandlNoState, ()>>::new(
        &el,
        el.primary_monitor().video_modes().next().unwrap(),
        CandlDimension::Classic(800, 400),
        "test candelabre window",
        CandlOptions::default(),
        CandlGraphics::init()
    ) {
        Ok(_) => Ok(()),
        Err(e) => Err(String::from(format!("{}", e)))
    }
}

struct CandlStateDrawer;

impl CandlGraphicsDrawer<CandlTestState, (), ()> for CandlStateDrawer {
    fn execute(&self, _: Option<&CandlTestState>, _: Option<&()>) {}
}

struct CandlTestState {
    pub value: i32
}

impl CandlUpdate<()> for CandlTestState {
    fn update(&mut self, _: ()) {}
}

type CandlStateGraphics = CandlGraphics<CandlStateDrawer, CandlTestState, (), ()>;

#[test]
fn create_window_with_state() -> Result<(), String> {
    let el = EventLoop::new();
    match CandlSurface::new_with_state(
        &el,
        el.primary_monitor().video_modes().next().unwrap(),
        CandlDimension::Fullscreen,
        &String::from("test window with data"),
        CandlOptions::default(),
        CandlStateGraphics::init(),
        CandlTestState { value: 42 }
    ) {
        Ok(_) => Ok(()),
        Err(e) => Err(String::from(format!("{}", e)))
    }
}

#[test]
fn open_multi_windows() -> Result<(), String> {
    let el = EventLoop::new();
    let mut win_manager: CandlManager<CandlSurface<CandlFakeGraphics, CandlNoState, ()>, ()> = CandlManager::new();
    for win_idx in 0..3 {
        &win_manager.create_window::<_, CandlSurface<CandlFakeGraphics, CandlNoState, ()>>(
            &el,
            el.primary_monitor().video_modes().next().unwrap(),
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
        win_manager.get_current(idx.clone()).unwrap();
    }
    for idx in &ids {
        win_manager.remove_window(idx.clone());
    }
    if win_manager.is_empty() { Ok(()) }
    else { Err(String::from("Test failed: CandlManager not empty!")) }
}
