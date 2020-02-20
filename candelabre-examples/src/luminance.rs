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
use glutin::dpi::PhysicalSize;
use glutin::window::WindowId;
use luminance::context::GraphicsContext;
use luminance::framebuffer::Framebuffer;
use luminance::pipeline::PipelineState;
use luminance::shader::program::Program;
use luminance::render_state::RenderState;
use luminance::state::GraphicsState;
use luminance::tess::{Mode, Tess, TessBuilder};
use luminance::texture::{Dim2, Flat};
use luminance_derive::{Semantics, Vertex};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

// ============================================================================
// structure to handle the window and to enable the use of the manager

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

    fn resize(&mut self, nsize: PhysicalSize<u32>) {
        if let CandlCurrentWrapper::PossiblyCurrent(ctx) = &self.ctx_ref() {
            ctx.resize(nsize);
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

impl LumSurface {
    fn back_buffer(&mut self) -> Framebuffer<Flat, Dim2, (), ()> {
        match self.ctx() {
            CandlCurrentWrapper::PossiblyCurrent(ctx) => {
                let (w, h) = ctx.window().inner_size().into();
                self.set_ctx(CandlCurrentWrapper::PossiblyCurrent(ctx));
                Framebuffer::back_buffer(self, [w, h])
            }
            CandlCurrentWrapper::NotCurrent(_) => panic!()
        }
    }
}

// ============================================================================
// structure to handle data for the window (Tess, triangles, etc)

mod utils;
use utils::{FS, VS};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Semantics)]
pub enum Semantics {
    #[sem(name = "co", repr = "[f32; 2]", wrapper = "VertexPosition")]
    Position,
    #[sem(name = "color", repr = "[u8; 3]", wrapper = "VertexColor")]
    Color
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Vertex)]
#[vertex(sem = "Semantics")]
struct Vertex {
    pos: VertexPosition,
    #[vertex(normalized = "true")]
    rgb: VertexColor
}

const TRIANGLE: [Vertex; 3] = [
    Vertex {pos: VertexPosition::new([-0.5, 0.5]), rgb: VertexColor::new([0, 255, 0])},
    Vertex {pos: VertexPosition::new([-0.0, 0.0]), rgb: VertexColor::new([255, 0, 0])},
    Vertex {pos: VertexPosition::new([0.5, 0.5]), rgb: VertexColor::new([0, 0, 255])}
];

struct LumData {
    pub tess: Tess,
    pub program: Program<Semantics, (), ()>
}

impl LumData {
    fn new(surface: &mut LumSurface) -> LumData {
        let tess = TessBuilder::new(surface)
            .add_vertices(TRIANGLE)
            .set_mode(Mode::Triangle)
            .build()
            .unwrap();
        let program = Program::<Semantics, (), ()>::from_strings(None, VS, None, FS)
            .expect("program creation")
            .ignore_warnings();
        LumData { tess, program }
    }
}

// ============================================================================
// main function

fn main() {
    let el = EventLoop::new();
    let mut win_manager: CandlManager<LumSurface, ()> = CandlManager::new();
    let mut win_datas = HashMap::<WindowId, LumData>::default();
    for idx in 0..3 {
        let wid = &win_manager.create_window::<_, LumSurface>(
            &el,
            CandlDimension::Classic(800, 400),
            &format!("test luminance #{}", idx+1),
            CandlOptions::default()
        ).unwrap();
        win_datas.insert(
            wid.clone(),
            LumData::new(win_manager.get_current(wid.clone()).unwrap())
        );
    }
    el.run(move |evt, _, ctrl_flow| {
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
                } => win_manager.remove_window(window_id),
                _ => ()
            }
            Event::RedrawRequested(win_id) => {
                let surface = win_manager.get_current(win_id.clone()).unwrap();
                let back_buffer = surface.back_buffer();
                let win_data = win_datas.get(&win_id).unwrap();
                surface.pipeline_builder().pipeline(
                    &back_buffer,
                    &PipelineState::default(),
                    |_, mut shd_gate| {
                        shd_gate.shade(&win_data.program, |_, mut rdr_gate| {
                            rdr_gate.render(&RenderState::default(), |mut tess_gate| {
                                tess_gate.render(&win_data.tess);
                            });
                        });
                    }
                );
                surface.swap_buffers();
            }
            _ => ()
        }
        if win_manager.is_empty() { *ctrl_flow = ControlFlow::Exit }
        else { *ctrl_flow = ControlFlow::Wait }
    });
}
