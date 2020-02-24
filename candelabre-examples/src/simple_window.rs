//! This example show how to use the candelabre-windowing lib with only the
//! CandlSurface to create a single window. The window must show a triangles,
//! and can be resized efficiently. Use 'ESC' to quit, 'SPACE' to define
//! randomly a new clear color (background color of the context), and 'A' to
//! change the name of the window.

use candelabre_core::{
    CandlGraphics, CandlRenderer, CandlShaderVariant
};
use candelabre_windowing::{
    CandlDimension, CandlOptions, CandlSurfaceBuilder, CandlWindow
};
use glutin::event::{
    ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent
};
use glutin::event_loop::{ControlFlow, EventLoop};

mod utils;
use utils::{FS, Message, SurfaceState, VS};
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

    let mut graphics = CandlGraphics::init();
    let vs_id = graphics.gen_shader(CandlShaderVariant::VertexShader, VS).unwrap();
    let fs_id = graphics.gen_shader(CandlShaderVariant::FragmentShader, FS).unwrap();
    let program = graphics.gen_program(Some(fs_id), Some(vs_id)).unwrap();
    //
    fn trya() {
        //
        &graphics.use_program(program);
        //
        println!("eeeeeee");
        //
    }
    //
    graphics.set_draw_fun(trya);

    let mut surface = CandlSurfaceBuilder::new()
        .dim(CandlDimension::Classic(800, 400))
        .title(TITLES_LIST[0])
        .options(CandlOptions::default())
        .render(graphics)
        .state(SurfaceState::default())
        .build(&el)
        .unwrap();

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
                        surface.update(Message::NewBgColor);
                        surface.state_mut().ask_redraw();
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
                if surface.state().need_redraw() {
                    surface.request_redraw();
                    surface.state_mut().draw_asked();
                }
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
