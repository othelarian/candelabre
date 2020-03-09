//! # Hello!
//! 
//! 
//! ???

use candelabre_core::{CandlRenderer, CandlUpdate};

pub struct SurfaceState();

impl CandlUpdate<()> for SurfaceState {
    fn update(&mut self, _: ()) {}
}

pub struct CandlRender {}

impl CandlRenderer<CandlRender, SurfaceState, ()> for CandlRender {
    fn init() -> CandlRender {
        //
        //
        CandlRender {}
    }

    fn finalize(&mut self) {
        //
        //
    }

    fn set_scale_factor(&mut self, scale_factor: f64) {
        //
        //
    }

    fn set_size(&mut self, nsize: (u32, u32)) {
        //
        //
    }

    fn draw_frame(&mut self, _state: &SurfaceState) {
        //
        //
    }
}

impl CandlRender {
    /// create a new renderer
    pub fn new() -> Self { CandlRender::init() }
}
