use candelabre_windowing::*;
use glutin::event_loop::EventLoop;
use std::marker::PhantomData;

// ===========================================================
// struct for the test
// ===========================================================

struct FakeGraphics<S: CandlUpdate<M>, M> {
    phantom_s: PhantomData<S>,
    phantom_m: PhantomData<M>
}

impl<S, M> CandlRenderer<FakeGraphics<S, M>, S, M> for FakeGraphics<S, M>
where S: CandlUpdate<M> {
    fn init() -> Self {
        Self {
            phantom_s: PhantomData,
            phantom_m: PhantomData
        }
    }

    fn finalize(&mut self) {}

    fn set_scale_factor(&mut self, _: f64) {}

    fn set_size(&mut self, _: (u32, u32)) {}

    fn draw_frame(&mut self, _: &S) {}
}

type NoStateFakeGraphics = FakeGraphics<CandlNoState, ()>;

struct FakeState {
    #[allow(dead_code)]
    value: u32
}

impl CandlUpdate<()> for FakeState {
    fn update(&mut self, _: ()) {}
}

impl FakeState {
}

type FakeStateFakeGraphics = FakeGraphics<FakeState, ()>;

type FakeSurface = CandlSurface<NoStateFakeGraphics, CandlNoState, ()>;

// ===========================================================
// integrations test
// ===========================================================

#[test]
fn use_surface_builder() -> Result<(), String> {
    let el = EventLoop::new();
    let builder = CandlSurfaceBuilder::new()
        .dim(CandlDimension::FullscreenSpecific(900, 700))
        .title("This is a test")
        .options(CandlOptions::default().set_cursor_mode(CursorMode::Invisible))
        .render(NoStateFakeGraphics::init())
        .no_state();
    match builder.build(&el) {
        Ok(_) => Ok(()),
        Err(e) => Err(String::from(format!("{}", e)))
    }
}

#[test]
fn create_window() -> Result<(), String> {
    let el = EventLoop::new();
    match <CandlSurface<NoStateFakeGraphics, CandlNoState, ()>>::new(
        &el,
        el.primary_monitor().video_modes().next().unwrap(),
        CandlDimension::Classic(800, 400),
        "test candelabre window",
        CandlOptions::default(),
        NoStateFakeGraphics::init()
    ) {
        Ok(_) => Ok(()),
        Err(e) => Err(String::from(format!("{}", e)))
    }
}

#[test]
fn create_window_with_state() -> Result<(), String> {
    let el = EventLoop::new();
    match CandlSurface::new_with_state(
        &el,
        el.primary_monitor().video_modes().next().unwrap(),
        CandlDimension::Fullscreen,
        &String::from("test window with data"),
        CandlOptions::default(),
        FakeStateFakeGraphics::init(),
        FakeState { value: 42 }
    ) {
        Ok(_) => Ok(()),
        Err(e) => Err(String::from(format!("{}", e)))
    }
}

#[test]
fn open_multi_windows() -> Result<(), String> {
    let el = EventLoop::new();
    let mut win_manager: CandlManager<FakeSurface, ()> = CandlManager::new();
    for win_idx in 0..3 {
        &win_manager.create_window::<_, FakeSurface>(
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
