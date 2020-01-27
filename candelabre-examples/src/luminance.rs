//! Example to show the usage of `CandlSurface` and `CandlManager` with
//! luminance as OpenGL backend. 'ESC' to close a window.

use candelabre_windowing::{
    CandlCurrentWrapper, CandlDimension, CandlElement, CandlError,
    CandlManager, CandlOptions, CandlWindow
};
use glutin::event::{
    ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent
};
use glutin::event_loop::{ControlFlow, EventLoop};
use luminance::context::GraphicsContext;
use luminance::state::GraphicsState;
use std::cell::RefCell;
use std::rc::Rc;

struct LumSurface {
    ctx: Option<CandlCurrentWrapper>,
    gfx_state: Rc<RefCell<GraphicsState>>
}

unsafe impl GraphicsContext for LumSurface {
    fn state(&self) -> &Rc<RefCell<GraphicsState>> { &self.gfx_state }
}

impl CandlWindow for LumSurface {
    fn ctx(&mut self) -> CandlCurrentWrapper { self.ctx.take().unwrap() }

    fn ctx_ref(&self) -> &CandlCurrentWrapper { self.ctx.as_ref().unwrap() }

    fn set_ctx(&mut self, nctx: CandlCurrentWrapper) { self.ctx = Some(nctx) }

    fn swap_buffers(&mut self) {
        if let CandlCurrentWrapper::PossiblyCurrent(ctx) = self.ctx.as_ref().unwrap() {
            ctx.swap_buffers().unwrap();
        }
    }
}

impl CandlElement<LumSurface> for LumSurface {
    fn build<T>(
        el: &EventLoop<T>,
        dim: CandlDimension,
        title: &str,
        options: CandlOptions
    ) -> Result<LumSurface, CandlError> {
        let ctx = LumSurface::init(el, dim, title, options)?;
        let ctx = Some(CandlCurrentWrapper::PossiblyCurrent(ctx));
        //let gfx_state = Rc::new(RefCell::new(GraphicsState::new().unwrap()));
        let gfx_state = unsafe {
            Rc::new(RefCell::new(GraphicsState::new_multi_contexts().unwrap()))
        };
        Ok(LumSurface {ctx, gfx_state})
    }
}

fn main() {
    let el = EventLoop::new();
    let mut win_manager: CandlManager<LumSurface, ()> = CandlManager::new();
    for idx in 0..3 {
        &win_manager.create_window::<_, LumSurface>(
            &el,
            CandlDimension::Classic(800, 400),
            &format!("test luminance #{}", idx+1),
            CandlOptions::default()
        ).unwrap();
    }
    //
    //
    //
    el.run(move |evt, _, ctrl_flow| {
        match evt {
            Event::LoopDestroyed => return,
            Event::WindowEvent {event, window_id} => match event {
                WindowEvent::Resized(physical_size) => {
                    //
                    // TODO
                    //
                }
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    input: KeyboardInput {
                        state: ElementState::Released,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    }, ..
                } => win_manager.remove_window(window_id),
                _ => ()
            }
            Event::RedrawRequested(win_id) => {
                //
                // TODO
                //
            }
            _ => ()
        }
        if win_manager.is_empty() { *ctrl_flow = ControlFlow::Exit }
        else { *ctrl_flow = ControlFlow::Wait }
    });
}
