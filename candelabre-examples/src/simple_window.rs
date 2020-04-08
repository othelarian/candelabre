//! This example show how to use the candelabre-windowing lib with only the
//! CandlSurface to create a single window. The window must show a triangles,
//! and can be resized efficiently. Use 'ESC' to quit, 'SPACE' to define
//! randomly a new clear color (background color of the context), and 'A' to
//! change the name of the window.

use candelabre_experiment::{CandlGraphics, CandlShaderVariant};
use candelabre_windowing::{
    CandlDimension, CandlOptions, CandlRenderer,
    CandlSurfaceBuilder, CandlWindow
};
use glutin::event::{
    ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent
};
use glutin::event_loop::{ControlFlow, EventLoop};

mod utils;
use utils::{FS, Message, new_nb, SurfaceState, SurfaceDrawer, VS};
/*
use utils::{
    FS, OGL_TRIANGLE, VS,
    Semantics, SurfaceData, SurfaceState,
    get_closure
};
*/

const TITLES_LIST: [&str; 5] = [
    "Candelabre example - Simple window",
    "The first example of candelabre",
    "The purpose of this example is to show",
    "the usage of candelabre libs",
    "from OpenGL to candelabre ;-)"
];

fn main() {
    let el = EventLoop::new();

    /*
    let program = Program::<Semantics, (), ()>::from_strings(None, VS, None, FS)
        .expect("program creation")
        .ignore_warnings();
    */

    let graphics = CandlGraphics::<SurfaceDrawer, _, _, _>::init();

    let mut surface = CandlSurfaceBuilder::new()
        .dim(CandlDimension::Classic(800, 400))
        .title(TITLES_LIST[0])
        .options(CandlOptions::default())
        .render(graphics)
        .state(SurfaceState::default())
        .video_mode(el.primary_monitor().video_modes().next().unwrap())
        .build(&el)
        .unwrap();

    let graphics = surface.render_mut();
    let _vs_id = graphics.gen_shader(CandlShaderVariant::VertexShader, VS).unwrap();
    let _fs_id = graphics.gen_shader(CandlShaderVariant::FragmentShader, FS).unwrap();
    //let program =
    //graphics.gen_program(Some(fs_id), Some(vs_id)).unwrap();
    /*
    let tess = TessBuilder::new(&mut surface)
        .add_vertices(OGL_TRIANGLE)
        .set_mode(Mode::Triangle)
        .build()
        .unwrap();
    */

    el.run(move |evt, _, ctrl_flow| {
        *ctrl_flow = ControlFlow::Wait;
        match evt {
            Event::LoopDestroyed => return,
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
                        //graphics.apply_clear_color([new_nb(), new_nb(), new_nb(), 1.0]);
                        let bgcol = [new_nb(), new_nb(), new_nb(), 1.0];
                        surface.render_mut().apply_clear_color(bgcol);
                        surface.ask_redraw();
                    }
                    VirtualKeyCode::A => {
                        surface.update(Message::IncValue);
                        surface.title(TITLES_LIST[surface.state().get_value() as usize]);
                    }
                    _ => ()
                }
                _ => ()
            }
            Event::MainEventsCleared => {
                if surface.check_redraw() { surface.request_redraw(); }
            }
            Event::RedrawRequested(_) => {
                //
                surface.draw();
                //
                /*
                let back_buffer = surface.back_buffer().unwrap();
                surface.pipeline_builder().pipeline(
                    &back_buffer,
                    &PipelineState::default(),
                    //
                    |_, _| ()
                    //
                    //surface.render_closure()
                    /*
                    |_, mut shd_gate| {
                        shd_gate.shade(&program, |_, mut rdr_gate| {
                            rdr_gate.render(&RenderState::default(), |mut tess_gate| {
                                //
                                //
                                //tess_gate.render(&tess);
                                //
                                //
                            });
                        });
                    }
                    */
                );
                surface.swap_buffers();
                */
            },
            _ => ()
        }
    });
}
