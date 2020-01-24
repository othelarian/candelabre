//! # Welcome!
//! 
//! This crate serve as a base layer for candelabre windowing and widgets
//! crates, and provide them some useful tools to interact in a safer way with
//! OpenGL context.
//! 
//! # `CandlGraphics`
//! 
//! This is currently the core element of this crate, and provide to a
//! `CandlSurface` the basic to interact with OpenGL.

#[deny(missing_docs)]

use gl;

/// Structure to handle all direct OpenGL operations
pub struct CandlGraphics {
    //gl: gl::Gl,
    clear_color: [f32; 4]
    //
}

impl CandlGraphics {
    /// build a new graphics from OpenGL
    pub fn new() -> Self {
        //
        //
        //
        Self {
            //
            clear_color: [0.0, 0.0, 0.0, 1.0]
            //
        }
    }

    /// draw in the back buffer
    pub fn draw_frame(&self) {
        //
        //
    }




    /// apply the clear color
    pub fn apply_clear_color(&mut self, new_color: [f32; 4]) {
        if self.clear_color != new_color {
            unsafe { gl::ClearColor(new_color[0], new_color[1], new_color[2], new_color[3]); }
        }
        self.clear_color = new_color;
    }
}
