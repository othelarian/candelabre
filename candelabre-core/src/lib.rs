//! # Welcome!
//! 
//! This crate serve as a base layer for candelabre windowing and widgets
//! crates, and provide them some useful tools to interact in a safer way with
//! OpenGL contex:
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

#[deny(missing_docs)]

use gl;

/// Renderer Trait
pub trait CandlRenderer<R> {
    /// init the renderer
    fn init() -> R;

    /// call for redraw the current OpenGL context
    fn draw_frame(&self);
}

/// Structure to handle all direct OpenGL operations
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

    fn draw_frame(&self) {
        //
        //gl.draw_frame([1.0, 0.5, 0.7, 1.0]);
        unsafe {
            gl::ClearColor(
                self.clear_color[0],
                self.clear_color[1],
                self.clear_color[2],
                self.clear_color[3]
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
        if self.clear_color != new_color {
            unsafe { gl::ClearColor(new_color[0], new_color[1], new_color[2], new_color[3]); }
        }
        self.clear_color = new_color;
    }
}
