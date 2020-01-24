//! ???

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

    /// draw in the framebuffer
    pub fn draw_frame(&self) {
        //
        //
    }




    /// apply the clear color
    unsafe fn apply_clear_color(&mut self, new_color: [f32; 4]) {
        if self.clear_color != new_color {
            gl::ClearColor(new_color[0], new_color[1], new_color[2], new_color[3]);
        }
        self.clear_color = new_color;
    }
}
