//! # Welcome!
//!
//! this crate was initially created to make the bridge between candelabre
//! windowing and widget crates, and was called "core", but I realized it
//! wasn't a really good idea, and tended to complexify a project, with
//! always a need to call candelabre-core just for two traits.
//! 
//! So now, it's the crate for testing new things in candelabre project!
//! ^^

#![deny(missing_docs)]

// =======================================================================
// =======================================================================
//               CandlGraphics
// =======================================================================
// =======================================================================


pub use self::candl_graphics::{
    CandlGraphics,
    CandlGraphicsDrawer,
    CandlGraphicsError,
    CandlProgram,
    CandlShader,
    CandlShaderVariant
};

mod candl_graphics {
    use candelabre_windowing::CandlRenderer;
    use candelabre_windowing::CandlUpdate;
    use gl::{self, types::GLuint};
    use std::ffi::CString;
    use std::marker::PhantomData;
    use std::ptr::null;
    
    /// candelabre shader type
    /// 
    /// Because each shader is different, each shader needs its own variation.
    #[derive(Debug, PartialEq)]
    pub enum CandlShaderVariant {
        /// vertex shader
        VertexShader,
        /// geometry shader
        GeometryShader,
        /// fragment shader
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
        /// error with the shader (creation, compilation, etc)
        ShaderError(&'static str),
        /// error with the program generation
        ProgramError(&'static str)
    }

    /// Draw function trait
    /// 
    /// There is several way to handle a redraw function, by using a closure is
    /// the most common, but the harder to implement and, in my own opinion,
    /// it's also the less readable. So instead of using a closure, candelabre
    /// use a trait the user must implement and send to CandlGraphics. It's a
    /// different way to approach the same problem, and it's only a choice of
    /// the candelabre author, so feel free to ask about this choice, the 
    /// dialog is open.
    pub trait CandlGraphicsDrawer<S: CandlUpdate<M>, M, O> {
        /// the only method needed is the draw_fun
        fn execute(&self, state: Option<&S>, opts: Option<&O>);
    }

    /// Candelabre Graphics
    /// 
    /// Structure to handle all direct OpenGL operations. It's the foundation stone
    /// for candelabre-widget.
    #[derive(Debug)]
    pub struct CandlGraphics<F, S, M, O>
    where F: CandlGraphicsDrawer<S, M, O>, S: CandlUpdate<M> {
        clear_color: [f32; 4],
        size: (u32, u32),
        scale_factor: f64,
        //
        shaders: Vec<CandlShader>,
        programs: Vec<CandlProgram>,
        //
        draw_fun: Option<F>,
        _state: PhantomData<S>,
        _message: PhantomData<M>,
        _opts: PhantomData<O>
    }

    impl<F, S, M, O> CandlRenderer<CandlGraphics<F, S, M, O>, S, M> for CandlGraphics<F, S, M, O>
    where F: CandlGraphicsDrawer<S, M, O>, S: CandlUpdate<M> {
        fn init() -> CandlGraphics<F, S, M, O> {
            Self {
                clear_color: [0.0, 0.0, 0.0, 1.0],
                size: (0, 0),
                scale_factor: 0.0,
                shaders: vec!(),
                programs: vec!(),
                draw_fun: None,
                _state: PhantomData,
                _message: PhantomData,
                _opts: PhantomData
            }
        }

        fn finalize(&mut self) {}

        fn set_scale_factor(&mut self, scale_factor: f64) {
            self.scale_factor = scale_factor;
        }

        fn set_size(&mut self, nsize: (u32, u32)) { self.size = nsize; }

        fn draw_frame(&mut self, state: &S) {
            unsafe {
                gl::ClearColor(
                    self.clear_color[0], self.clear_color[1],
                    self.clear_color[2], self.clear_color[3]
                );
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }
            if let Some(fun) = &self.draw_fun {
                fun.execute(Some(state), None);
            }
        }
    }

    impl<F, S, M, O> CandlGraphics<F, S, M, O>
    where F: CandlGraphicsDrawer<S, M, O>, S: CandlUpdate<M> {
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

        /// make use of a program already added
        pub fn use_program(&self,id: usize) {
            unsafe { gl::UseProgram(self.get_program(id).get_ptr()); }
        }

        /// apply the clear color
        pub fn apply_clear_color(&mut self, new_color: [f32; 4]) {
            self.clear_color = new_color;
        }
    }
}
