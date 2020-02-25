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

/// Renderer Trait
/// 
/// This trait must be used by any structure which want to fill the gap between
/// a `CandlWindow` and OpenGL.
pub trait CandlRenderer<R> {
    /// init the renderer
    fn init() -> R;

    /// call from `CandlSurface` after the gl initialization
    fn finalize(&mut self);

    /// set the scale factor when it changed
    fn set_scale_factor(&mut self, scale_factor: f64);

    /// set the size of the window / surface holding the OpenGL context
    fn set_size(&mut self, nsize: (u32, u32));

    /// call for redraw the current OpenGL context
    fn draw_frame(&self);

    /// when a window is resize, the renderer must follow
    fn resize(&mut self, nsize: (u32, u32), scale_factor: f64) {
        self.set_size(nsize);
        //
        println!("sf: {}", scale_factor);
        //
        self.set_scale_factor(scale_factor);
    }
}

// =======================================================================
// =======================================================================
//               CandlGraphics
// =======================================================================
// =======================================================================


pub use self::candl_graphics::{
    CandlGraphics,
    CandlGraphicsError,
    CandlProgram,
    CandlShader,
    CandlShaderVariant
};

mod candl_graphics {
    use super::CandlRenderer;
    use gl::{self, types::GLuint};
    use std::ffi::CString;
    use std::ptr::null;
    
    /// candelabre shader type
    /// 
    /// Because each shader is different, each shader needs its own variation.
    #[derive(Debug, PartialEq)]
    pub enum CandlShaderVariant {
        VertexShader,
        GeometryShader,
        FragmentShader
    }

    impl CandlShaderVariant {
        fn get_glenum(&self) -> gl::types::GLenum {
            match self {
                Self::VertexShader => gl::VERTEX_SHADER,
                Self::GeometryShader => gl::GEOMETRY_SHADER,
                Self::FragmentShader => gl::FRAGMENT_SHADER
            }
        }
    }

    /// candelabre shader
    /// 
    /// To be able to draw something, the renderer need some shaders. This is the
    /// purpose of this structure
    #[derive(Debug)]
    pub struct CandlShader {
        variant: CandlShaderVariant,
        ptr: GLuint
    }

    impl CandlShader {
        /// create a new shader
        pub fn new(variant: CandlShaderVariant, src: &str) -> Result<Self, &'static str> {
            unsafe {
                let ptr = gl::CreateShader(variant.get_glenum());
                if ptr == 0 { return Err("bad pointer generated"); }
                let mut pragma = String::from("#version 330 core\n#extension GL_ARB_separate_shader_objects : require\n");
                pragma.push_str(src);
                let c_src = CString::new(pragma.as_bytes()).unwrap();
                gl::ShaderSource(ptr, 1, [c_src.as_ptr()].as_ptr(), null());
                gl::CompileShader(ptr);
                let mut compiled: gl::types::GLint = gl::FALSE.into();
                gl::GetShaderiv(ptr, gl::COMPILE_STATUS, &mut compiled);
                if compiled == gl::TRUE.into() {
                    Ok(CandlShader {variant, ptr})
                } else {
                    gl::DeleteShader(ptr);
                    Err("failed to compile the shader")
                }
            }
        }

        /// get the GL pointer to the shader
        pub fn get_ptr(&self) -> GLuint { self.ptr.clone() }

        /// check variant of the shader
        pub fn check_variant(&self, variant: CandlShaderVariant) -> bool {
            self.variant == variant
        }
    }

    /// candelabre program
    /// 
    /// With some shaders you can build a program, which then will be used by
    /// an OpenGL context to render your creation into your window. You can
    /// attach a lot of different shaders to a program, but to make things
    /// working, and because CandlGraphics isn't a fully featured renderer, the
    /// CandlProgram only takes vertex, geometry and fragment shaders.
    #[derive(Debug)]
    pub struct CandlProgram {
        ptr: GLuint,
        fs: Option<GLuint>,
        //gs: Option<GLuint>,
        vs: Option<GLuint>
    }

    impl CandlProgram {
        /// create a new program
        pub fn new(fs: Option<&CandlShader>, /*gs: Option<&CandlShader>,*/ vs: Option<&CandlShader>)
        -> Result<CandlProgram, &'static str> {
            if fs.is_some() && fs.as_ref().unwrap().check_variant(CandlShaderVariant::FragmentShader) {
                Err("the fragment shader (fs) is not of the right variant")
            //} else if gs.is_some() && gs.as_ref().unwrap().check_variant(CandlShaderVariant::GeometryShader) {
            //    Err("the geometry shader (gs) is not of the right variant")
            } else if vs.is_some() && vs.as_ref().unwrap().check_variant(CandlShaderVariant::VertexShader) {
                Err("the vertex shader (vs) is not of the right variant")
            } else {
                //
                unsafe {
                    let ptr = gl::CreateProgram();
                    let fs = CandlProgram::attach_shader(ptr.clone(), fs);
                    //let gs = CandlProgram::attach_shader(program.clone(), gs);
                    let vs = CandlProgram::attach_shader(ptr.clone(), vs);
                    gl::LinkProgram(ptr);
                    //
                    //gl::UseProgram(program);
                    //
                    Ok(Self {ptr, fs, /*gs,*/ vs})
                }
            }
        }

        /// get the fragment shader pointer
        pub fn get_fs(&self) -> &Option<GLuint> { &self.fs }

        /// get the geometry shader pointer
        //pub fn get_gs(&self) -> &Option<GLuint> { &self.gs }

        /// get the vertex shader pointer
        pub fn get_vs(&self) -> &Option<GLuint> { &self.vs }

        /// get the pointer to a program
        pub fn get_ptr(&self) -> GLuint { self.ptr.clone() }

        fn attach_shader(program: GLuint, sh: Option<&CandlShader>) -> Option<GLuint> {
            if let Some(sh) = sh {
                let ptr = sh.get_ptr();
                unsafe { gl::AttachShader(program, ptr); }
                Some(ptr)
            }
            else { None }
        }
    }

    /// CandlGraphics error
    /// 
    /// Because a renderer can generate error, let's do this properly
    #[derive(Debug)]
    pub enum CandlGraphicsError {
        ShaderError(&'static str),
        ProgramError(&'static str)
    }

    /// Candelabre Graphics
    /// 
    /// Structure to handle all direct OpenGL operations. It's the foundation stone
    /// for candelabre-widget.
    #[derive(Debug)]
    pub struct CandlGraphics<F: Fn()> {
        clear_color: [f32; 4],
        size: (u32, u32),
        scale_factor: f64,
        //
        shaders: Vec<CandlShader>,
        programs: Vec<CandlProgram>,
        //
        draw_fun: Option<F>
    }

    impl<F: Fn()> CandlRenderer<CandlGraphics<F>> for CandlGraphics<F> {
        fn init() -> CandlGraphics<F> {
            Self {
                clear_color: [0.0, 0.0, 0.0, 1.0],
                size: (0, 0),
                scale_factor: 0.0,
                shaders: vec!(),
                programs: vec!(),
                draw_fun: None
            }
        }

        fn finalize(&mut self) {}

        fn set_scale_factor(&mut self, scale_factor: f64) {
            self.scale_factor = scale_factor;
        }

        fn set_size(&mut self, nsize: (u32, u32)) { self.size = nsize; }

        fn draw_frame(&self) {
            unsafe {
                gl::ClearColor(
                    self.clear_color[0], self.clear_color[1],
                    self.clear_color[2], self.clear_color[3]
                );
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }
            if let Some(fun) = &self.draw_fun {
                (fun)();
            }
        }
    }

    impl<F: Fn()> CandlGraphics<F> {
        /// redefine the drawing closure
        pub fn set_draw_fun(&mut self, draw_fun: F) { self.draw_fun = Some(draw_fun); }

        /// generate a new shader and save it in CandlGraphics
        pub fn gen_shader(&mut self, variant: CandlShaderVariant, src: &str)
        -> Result<usize, CandlGraphicsError> {
            match CandlShader::new(variant, src) {
                Ok(shader) => Ok(self.add_shader(shader)),
                Err(err) => Err(CandlGraphicsError::ShaderError(err))
            }
        }

        /// adding a shader to CandlGraphics
        pub fn add_shader(&mut self, shader: CandlShader) -> usize {
            self.shaders.push(shader);
            self.shaders.len() - 1
        }

        /// get a reference to a shader
        pub fn get_shader(&self, id: usize) -> &CandlShader { &self.shaders[id] }

        /// removing a shader
        pub fn remove_shader(&mut self, id: usize) -> CandlShader {
            self.shaders.remove(id)
        }

        /// generate a new program, the usize in the Option is the index inside
        /// the shaders vector in CandlGraphics, so there is no need to
        /// generate shaders outside CandlGraphics, just push the id the
        /// `gen_shaders` and `add_shaders` returns.
        pub fn gen_program(&mut self, fs: Option<usize>, vs: Option<usize>)
        -> Result<usize, CandlGraphicsError> {
            let fs = if let Some(sh) = fs { Some(self.get_shader(sh)) } else { None };
            let vs = if let Some(sh) = vs { Some(self.get_shader(sh)) } else { None };
            match CandlProgram::new(fs, vs) {
                Ok(program) => Ok(self.add_program(program)),
                Err(err) => Err(CandlGraphicsError::ProgramError(err))
            }
        }

        /// add a program in CandlGraphics
        pub fn add_program(&mut self, program: CandlProgram) -> usize {
            self.programs.push(program);
            self.programs.len() - 1
        }

        /// get a reference to a program
        pub fn get_program(&self, id: usize) -> &CandlProgram { &self.programs[id] }

        /// remove a program
        pub fn remove_program(&mut self, id: usize) -> CandlProgram {
            self.programs.remove(id)
        }

        pub fn use_program(&self,id: usize) {
            unsafe { gl::UseProgram(self.get_program(id).get_ptr()); }
        }

        /// apply the clear color
        pub fn apply_clear_color(&mut self, new_color: [f32; 4]) {
            self.clear_color = new_color;
        }
    }
}

/// State Trait
/// 
/// When a surface become stateful, there is a way to do it, and it goes with
/// this trait. It's the bridge between the state and the renderer.
pub trait CandlUpdate<M, R: CandlRenderer<R>> {
    fn update(&mut self, message: M, renderer: &mut R);
}
