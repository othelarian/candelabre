//! # Welcome!
//! 
//! This crate serve as a base layer for candelabre windowing and widgets
//! crates, and provide them some useful tools to interact in a safer way with
//! OpenGL contex.
//! 
//! # `CandlRenderer`
//! 
//! This trait is used by candelabre-windowing surface and manager to handle
//! OpenGL context safely. Why this trait? Simple: being able to use
//! `CandlGraphics`, the renderer which come along with candelabre project, or
//! make your own renderer if you think yours will be better (I think it's
//! quite easy, try it).
//! 
//! # `CandlGraphics`
//! 
//! This is currently the core element of this crate, and provide to a
//! `CandlSurface` the basic to interact with OpenGL.
//! 
//! # `CandlUpdate`
//! 
//! Not a real OpenGL necessity, `CandlUpdate` is here to support the capacity
//! of candelabre-windowing elements to be stateful. This trait define the
//! way a state must be set up inside a `CandlSurface` to maximize the usage of
//! `CandlGraphics`.

#[deny(missing_docs)]

use gl;

/// Renderer Trait
/// 
/// This trait must be used by any structure which want to fill the gap between
/// a `CandlWindow` and OpenGL.
pub trait CandlRenderer<R> {
    /// init the renderer
    fn init() -> R;

    /// call from `CandlSurface` after the gl initialization
    fn finalize(&mut self);

    /// call for redraw the current OpenGL context
    fn draw_frame(&self);
}

// =======================================================================
// =======================================================================
//               CandlGraphics
// =======================================================================
// =======================================================================

//
// TODO : make the tree
//
//

//Candl


/// Candelabre Graphics
/// 
/// Structure to handle all direct OpenGL operations. It's the foundation stone
/// for candelabre-widget.
#[derive(Debug)]
pub struct CandlGraphics {
    clear_color: [f32; 4]
    //
}

impl CandlRenderer<CandlGraphics> for CandlGraphics {
    fn init() -> CandlGraphics {
        //
        //
        Self {
            clear_color: [0.0, 0.0, 0.0, 1.0]
            //
        }
    }

    fn finalize(&mut self) {}

    fn draw_frame(&self) {
        //
        unsafe {
            gl::ClearColor(
                self.clear_color[0], self.clear_color[1],
                self.clear_color[2], self.clear_color[3]
            );
            gl::Clear(gl::COLOR_BUFFER_BIT);
            //
            //
        }
        //
    }
}

impl CandlGraphics {
    /// apply the clear color
    pub fn apply_clear_color(&mut self, new_color: [f32; 4]) {
        /*
        if self.clear_color != new_color {
            unsafe { gl::ClearColor(new_color[0], new_color[1], new_color[2], new_color[3]); }
        }
        */
        self.clear_color = new_color;
    }
}

/// State Trait
/// 
/// When a surface become stateful, there is a way to do it, and it goes with
/// this trait. It's the bridge between the state and the renderer.
pub trait CandlUpdate<M, R: CandlRenderer<R>> {
    fn update(&mut self, message: M, renderer: &mut R);
}
