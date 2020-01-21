//! # Welcome!
//!
//! This crate provide a two elements:
//!
//! * `CandlSurface`, a window type who generate a surface for
//! [luminance](https://github.com/phaazon/luminance-rs/)
//! * `CandlManager`, a window manager to enable using multiple windows in a
//! single application / thread
//!
//! # CandlSurface
//! 
//! Core of this lib, it's a simple way to get an OpenGL context and then use
//! luminance [luminance](https://github.com/phaazon/luminance-rs/). Initially
//! it's a copy of luminance-glutin lib, modified to be able to work with the
//! CandlManager.
//! 
//! # CandlManager
//! 
//! When dealing with multiple windows in a single application, it quickly
//! become complex and error prone. You can only use one OpenGL context at a
//! time, and must so you need to swap between each contexts when you update
//! what you display. With `CandlManager`, you have what you need to help you
//! in this tedious task. It take the responsability to make the swap for you,
//! and track each window you link to it.
//! 
//! # About data in `CandlSurface` and `CandlManager`
//! 
//! It's possible to add data into the `CandlSurface` and the `CandlManager`.
//! The purpose of this data is to make this structures stateful, but there is
//! some limitations. It's strongly discouraged to save OpenGL data, like the
//! tess, the program, or the shader, in the data of the structures, this isn't
//! the purpose of this data, and can lead to useless complexity due to the
//! borrowing and onwership when it come to the render phase.
//! 
//! # Ideas of improvements
//! 
//! The first idea for improving this library is to encapsulate OpenGL data
//! into the structures, maybe with a method to implement, or a closure. If
//! you have an idea, feel free to open an issue!

#![deny(missing_docs)]

use gl;
use glutin::{
    Api, ContextBuilder, GlProfile, GlRequest, NotCurrent,
    PossiblyCurrent, WindowedContext
};
use glutin::dpi::LogicalSize;
use glutin::event_loop::EventLoop;
use glutin::window::{Fullscreen, WindowBuilder, WindowId};
use luminance::context::GraphicsContext;
use luminance::framebuffer::Framebuffer;
use luminance::state::{GraphicsState, StateQueryError};
use luminance::texture::{Dim2, Flat};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::os::raw::c_void;
use std::rc::Rc;
use takeable_option::Takeable;

pub use glutin::{ContextError, CreationError};

/// The error of Candelabre Windowing
/// 
/// All the possible errors you can meet with this crate are from this type.
/// One type to rule them all, one type to find them.
#[derive(Debug)]
pub enum CandlError {
    /// OpenGL context creation error
    CreationError(CreationError),
    /// OpenGL context usage error
    ContextError(ContextError),
    /// luminance gfx state creation error
    GraphicsStateError(StateQueryError),
    /// Candelabre internal error
    InternalError(&'static str)
}

impl fmt::Display for CandlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            CandlError::CreationError(ref e) =>
                write!(f, "Candelabre Surface creation error: {}", e),
            CandlError::ContextError(ref e) =>
                write!(f, "OpenGL context usage error: {}", e),
            CandlError::GraphicsStateError(ref e) =>
                write!(f, "Luminance GraphicsState creation error: {}", e),
            CandlError::InternalError(e) =>
                write!(f, "Candelabre internal error: {}", e)
        }
    }
}

impl From<CreationError> for CandlError {
    fn from(e: CreationError) -> Self { CandlError::CreationError(e) }
}

impl From<ContextError> for CandlError {
    fn from(e: ContextError) -> Self { CandlError::ContextError(e) }
}

/// Window dimensions
///
/// This type is an extract from
/// [luminance-windowing](https://docs.rs/luminance-windowing/0.8.1/luminance_windowing/)
/// to avoid the call of this crate and separate a little bit more luminance from
/// candelabre. The idea is to maybe be able to use candelabre without luminance.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CandlDimension {
    /// cassic windowed mode
    Classic(u32, u32),
    /// fullscreen (for only one monitor)
    Fullscreen,
    /// fullscreen mode but with specific dimensions
    FullscreenSpecific(u32, u32)
}

/// Cursor mode
///
/// This type is an extract from
/// [luminance-windowing](https://docs.rs/luminance-windowing/0.8.1/luminance_windowing/)
/// simplify to better match glutin cursor visibility.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CursorMode {
    /// cursor visible
    Visible,
    /// cursor invisible
    Invisible
}

/// Window options
///
/// This type is an extract from
/// [luminance-windowing](https://docs.rs/luminance-windowing/0.8.1/luminance_windowing/)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CandlOptions {
    cursor_mode: CursorMode,
    samples: Option<u32>
}

impl Default for CandlOptions {
    /// Default:
    /// 
    /// Default options for a window, with cursor visible
    fn default() -> Self {
        CandlOptions {
            cursor_mode: CursorMode::Visible,
            samples: None
        }
    }
}

impl CandlOptions {
    /// choose the cursor visibility
    pub fn set_cursor_mode(self, mode: CursorMode) -> Self {
        CandlOptions { cursor_mode: mode, ..self }
    }

    /// get the cursor current visiblity
    pub fn cursor_mode(&self) -> CursorMode { self.cursor_mode }

    /// choose the number of samples for multisampling
    pub fn set_samples<S: Into<Option<u32>>>(self, samples: S) -> Self {
        CandlOptions { samples: samples.into(), ..self }
    }

    /// get the number of samples for multisampling
    pub fn samples(&self) -> Option<u32> { self.samples }
}

/// Surface builder
/// 
/// This builder help create a new `CandlSurface` in a more idiomatic way
pub struct CandlSurfaceBuilder<'a, F, R, D> where F: FnOnce() {
    dim: CandlDimension,
    title: &'a str,
    options: CandlOptions,
    render_fn: Option<F>,
    render_data: Option<R>,
    state: Option<D>
}

impl<'a, F, R, D> CandlSurfaceBuilder<'a, F, R, D> where F: FnOnce() {
    /// builder constructor
    ///
    /// By default, the builder set the window dimension to Classic(800, 400)
    /// and with no name
    pub fn new() -> Self {
        CandlSurfaceBuilder {
            dim: CandlDimension::Classic(800, 400),
            title: "",
            options: CandlOptions::default(),
            render_fn: None,
            render_data: None,
            state: None
        }
    }

    /// modify the starting dimension
    pub fn dim(&mut self, dim: CandlDimension) { self.dim = dim; }

    /// set a title ("" by default)
    pub fn title(&mut self, title: &'a str) { self.title = title; }

    /// modify the options
    pub fn options(&mut self, options: CandlOptions) { self.options = options; }

    /// set render closure
    pub fn render_closure(&mut self) {
        //
        // TODO
        //
    }

    /// set render data
    /// 
    /// This function have a second implicit purpose: set up the data type of
    /// the render data the surface can use
    pub fn render_data(&mut self) {
        //
        // TODO
        //
    }

    /// change the initial state
    pub fn state(&mut self, init_state: D) { self.state = Some(init_state); }

    /// try to build the surface
    pub fn build(mut self) -> Result<CandlSurface<F, R, D>, CandlError> {
        //
        //
        Err(CandlError::InternalError("NOT IMPLEMENTED"))
        //
    }
}

/// The display surface
///
/// The first core element of this crate, the CandlSurface is a window with an
/// OpenGL context, and some options. It sounds very simple, and in fact it is.
/// Look for the example to see how to use it.
/// 
/// A data type can be associated to the surface, to make it stateful. It isn't
/// mandatory, but useful.
/// 
/// The basic constructor automatically associate the type `()` to the data
/// type of the surface, and a second constructor called `new_with_data()` is
/// here to let the advanced user specify the data type and the initial datas
/// associated with the surface.
pub struct CandlSurface<F, R, D> where F: FnOnce() {
    ctx: CandlCurrentWrapper,
    gfx_state: Rc<RefCell<GraphicsState>>,
    render_fn: F,
    render_data: R,
    state: D
}

unsafe impl<F, R, D> GraphicsContext for CandlSurface<F, R, D> where F: FnOnce() {
    fn state(&self) -> &Rc<RefCell<GraphicsState>> { &self.gfx_state }
}

impl<F, R> CandlSurface<F, R, ()> where F: FnOnce() {
    /// creation of a CandlSurface
    pub fn new<T>(
        el: &EventLoop<T>,
        dim: CandlDimension,
        title: &str,
        options: CandlOptions
    ) -> Result<Self, CandlError> {
        //CandlSurface::window_builder(el, dim, title, options, false, ())
        //
        Err(CandlError::InternalError("NOT IMPLEMENTED"))
        //
    }
}

impl<F, R, D> CandlSurface<F, R, D> where F: FnOnce() {
    /*
    /// constructor with data
    ///
    /// This constructor can be used to associate a data type to the window.
    /// The data type must be specified.
    pub fn new_with_data<T>(
        el: &EventLoop<T>,
        dim: CandlDimension,
        title: &str,
        options: CandlOptions,
        init_state: D
    ) -> Result<Self, CandlError> {
        CandlSurface::window_builder(el, dim, title, options, false, init_state)
    }
    */

    /*
    /// internal builder for the window
    fn window_builder<T>(
        el: &EventLoop<T>,
        dim: CandlDimension,
        title: &str,
        options: CandlOptions,
        multi: bool,
        init_state: D
    ) -> Result<Self, CandlError> {
        let win_builder = WindowBuilder::new().with_title(title);
        let win_builder = match dim {
            CandlDimension::Classic(w, h) =>
                win_builder.with_inner_size(LogicalSize::new(w, h)),
            CandlDimension::Fullscreen =>
                win_builder.with_fullscreen(
                    Some(Fullscreen::Exclusive(
                        el.primary_monitor()
                            .video_modes()
                            .next()
                            .unwrap()
                    ))
                ),
            CandlDimension::FullscreenSpecific(w, h) =>
                win_builder.with_inner_size(LogicalSize::new(w, h))
                    .with_fullscreen(
                        Some(Fullscreen::Exclusive(
                            el.primary_monitor()
                                .video_modes()
                                .next()
                                .unwrap()
                        ))
                    )
        };
        let ctx = ContextBuilder::new()
            .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
            .with_gl_profile(GlProfile::Core)
            .with_multisampling(options.samples().unwrap_or(0) as u16)
            .with_double_buffer(Some(true))
            .build_windowed(win_builder, &el)?;
        let ctx = unsafe { ctx.make_current().map_err(|(_, e)| e)? };
        ctx.window().set_cursor_visible(match options.cursor_mode() {
            CursorMode::Visible => true,
            CursorMode::Invisible => false
        });
        gl::load_with(|s| ctx.get_proc_address(s) as *const c_void);
        let gfx_state = if multi {
            GraphicsState::new().map_err(CandlError::GraphicsStateError)?
        } else {
            unsafe {
                GraphicsState::new_multi_contexts()
                    .map_err(CandlError::GraphicsStateError)?
            }
        };
        Ok(CandlSurface {
            ctx: CandlCurrentWrapper::PossiblyCurrent(ctx),
            gfx_state: Rc::new(RefCell::new(gfx_state)),
            //
            //
            state: init_state
        })
    }
    */


    /*


    /// get the data as a immutable reference
    pub fn data(&self) -> &D { &self.data }

    /// get the data as a mutable reference
    pub fn data_mut(&mut self) -> &mut D { &mut self.data }

    /// get the OpenGL context from the surface
    pub fn ctx(&mut self) -> &mut CandlCurrentWrapper { &mut self.ctx }

    /// get the back buffer of the surface
    pub fn back_buffer(&mut self) -> Result<Framebuffer<Flat, Dim2, (), ()>, CandlError> {
        match &self.ctx {
            CandlCurrentWrapper::PossiblyCurrent(ctx) => {
                let (w, h) = ctx.window().inner_size().into();
                Ok(Framebuffer::back_buffer(self, [w, h]))
            }
            CandlCurrentWrapper::NotCurrent(_) =>
                Err(CandlError::InternalError("using back buffer of not current OpenGL context"))
        }
    }

    /// swap the OpenGL back buffer and current buffer
    pub fn swap_buffers(&mut self) {
        if let CandlCurrentWrapper::PossiblyCurrent(ctx) = &self.ctx {
            ctx.swap_buffers().unwrap();
        }
    }



    */
}

/// Tracking the context status
///
/// When working with OpenGL context it's important to know if the context you
/// working with is the current one or not. If you're using only one window,
/// it's ok to avoid this enum and only use `PossiblyCurrent`, because the
/// context status will never change. But if you need multiple windows, you
/// need to know if the context you want to work with is the current one, and
/// if not you need to change that. The `CandlManager` is here to do that for
/// you, and use `CandlCurrentWrapper` to do so.
pub enum CandlCurrentWrapper {
    /// OpenGL context is probably current
    PossiblyCurrent(WindowedContext<PossiblyCurrent>),
    /// OpenGL context is not current
    NotCurrent(WindowedContext<NotCurrent>)
}




// ============================================================================




/*


/// The window manager
///
/// Second core element of this lib, the `CandlManager` is the tool to bring
/// all your app's windows under its command. It's main purpose is to remove
/// the burden of OpenGL contexts swapping, and a way to easily manage multiple
/// windows in a application. Its usage is pretty simple:
/// 
/// 1. create it
/// 2. insert new window in it
/// 3. call `get_current()` to swap contexts and get the window you can work with
/// 4. done
/// 
/// Check the
/// [candelabre examples](https://github.com/othelarian/candelabre/tree/master/candelabre-examples)
/// to see it in action.
pub struct CandlManager<D, M> {
    current: Option<WindowId>,
    surfaces: HashMap<WindowId, Takeable<CandlSurface<D>>>,
    data: M
}

impl<D> CandlManager<D, ()> {
    /// most default constructor for the manager
    pub fn new() -> Self {
        CandlManager { current: None, surfaces: HashMap::default(), data: () }
    }
}

impl<M> CandlManager<(), M> {
    /// create a new window, tracked by the manager
    /// 
    /// For internal reason, it isn't possible to add a `CandlSurface` manually
    /// created to the manager, it's mandatory to use the `create_window()`
    /// method instead.
    /// 
    /// This method is the most basic one, creating a surface with no data
    /// associated.
    /// 
    /// WARNING : the first surface created with the manager decide of all the
    /// surfaces data type of the manager, it isn't in the scope of this lib to
    /// handle the complexity of multiple data type across an hashmap of
    /// surfaces.
    pub fn create_window<T>(
        &mut self,
        el: &EventLoop<T>,
        dim: CandlDimension,
        title: &str,
        options: CandlOptions
    ) -> Result<WindowId, CandlError> {
        self.create_window_with_data(el, dim, title, options, ())
    }
}

impl<D, M> CandlManager<D, M> {
    /// constructor for the manager with data type link to it
    pub fn new_with_data(init_data: M) -> Self {
        CandlManager {
            current: None,
            surfaces: HashMap::default(),
            data: init_data
        }
    }

    /// create a new window with surface associated data type
    pub fn create_window_with_data<T>(
        &mut self,
        el: &EventLoop<T>,
        dim: CandlDimension,
        title: &str,
        options: CandlOptions,
        init_data: D
    ) -> Result<WindowId, CandlError> {
        let mut surface = CandlSurface::window_builder(el, dim, title, options, true, init_data)?;
        match &surface.ctx() {
            CandlCurrentWrapper::PossiblyCurrent(ctx) => {
                let win_id = ctx.window().id();
                if let Some(old_id) = self.current.take() {
                    if let Some(old_surface) = self.surfaces.get_mut(&old_id) {
                        let mut old_win = Takeable::take(old_surface);
                        if let CandlCurrentWrapper::PossiblyCurrent(ctx) = old_win.ctx {
                            let nctx = unsafe { ctx.treat_as_not_current() };
                            old_win.ctx = CandlCurrentWrapper::NotCurrent(nctx);
                        }
                        *old_surface = Takeable::new(old_win);
                    }
                }
                self.surfaces.insert(win_id, Takeable::new(surface));
                self.current = Some(win_id);
                Ok(win_id)
            }
            CandlCurrentWrapper::NotCurrent(_) =>
                Err(CandlError::InternalError(
                    "Surface creation from manager generated a not current context"
                ))
        }
    }

    /// vector with all the WindowId managed by the CandlManager
    pub fn list_window_ids(&self) -> Vec<WindowId> { self.surfaces.keys().cloned().collect() }

    /// remove a window from the manager
    /// 
    /// If you don't call this method after closing a window, the OpenGL
    /// context continue to exist, and can lead to memory leaks.
    pub fn remove_window(&mut self, id: WindowId) {
        if Some(id) == self.current { self.current.take(); }
        self.surfaces.remove(&id);
    }

    /// check if there is still living windows, or if the manager is empty
    ///
    /// The purpose of this method is to check if the application can be close,
    /// from an OpenGL perspective.
    pub fn is_empty(&self) -> bool { self.surfaces.is_empty() }

    /// get a mutable reference to the current surface
    /// 
    /// This method is the most important of the manager. At first, there is a
    /// check for the asked window to see if it's the current one, and if not
    /// the method try to swap the OpenGL contexts to make the asked window
    /// current, and make the old current context not current.
    pub fn get_current(&mut self, id: WindowId)
    -> Result<&mut CandlSurface<D>, ContextError> {
        let res = if Some(id) != self.current {
            let ncurr_ref = self.surfaces.get_mut(&id).unwrap();
            let mut ncurr_surface = Takeable::take(ncurr_ref);
            match ncurr_surface.ctx {
                CandlCurrentWrapper::PossiblyCurrent(_) => {
                    *ncurr_ref = Takeable::new(ncurr_surface);
                    Ok(())
                }
                CandlCurrentWrapper::NotCurrent(nctx) => unsafe {
                    match nctx.make_current() {
                        Err((rctx, err)) => {
                            match rctx.make_not_current() {
                                Ok(rctx) => {
                                    ncurr_surface.ctx = CandlCurrentWrapper::NotCurrent(rctx);
                                    *ncurr_ref = Takeable::new(ncurr_surface);
                                    Err(err)
                                }
                                Err((_, err2)) =>
                                    panic!("Couldn't make current and not current: {}, {}", err, err2)
                            }
                        }
                        Ok(rctx) => {
                            ncurr_surface.ctx = CandlCurrentWrapper::PossiblyCurrent(rctx);
                            *ncurr_ref = Takeable::new(ncurr_surface);
                            Ok(())
                        }
                    }
                }
            }
        }
        else {
            let ncurr_ref = self.surfaces.get_mut(&id).unwrap();
            let ncurr_surface = Takeable::take(ncurr_ref);
            match &ncurr_surface.ctx {
                CandlCurrentWrapper::PossiblyCurrent(_) => {
                    *ncurr_ref = Takeable::new(ncurr_surface);
                    Ok(())
                }
                CandlCurrentWrapper::NotCurrent(_) => panic!()
            }
        };
        match res {
            Ok(()) => {
                if Some(id) != self.current {
                    if let Some(old_id) = self.current.take() {
                        let old_ref = self.surfaces.get_mut(&old_id).unwrap();
                        let mut old_surface = Takeable::take(old_ref);
                        if let CandlCurrentWrapper::PossiblyCurrent(octx) = old_surface.ctx {
                            unsafe { old_surface.ctx = CandlCurrentWrapper::NotCurrent(octx.treat_as_not_current()); }
                        }
                        *old_ref = Takeable::new(old_surface);
                    }
                    self.current = Some(id);
                }
                Ok(self.surfaces.get_mut(&id).unwrap())
            }
            Err(err) => {
                if let Some(old_id) = self.current.take() {
                    let old_ref = self.surfaces.get_mut(&old_id).unwrap();
                    let mut old_surface = Takeable::take(old_ref);
                    if let CandlCurrentWrapper::PossiblyCurrent(octx) = old_surface.ctx {
                        unsafe {
                            match octx.make_not_current() {
                                Err((_, err2)) =>
                                    panic!("make current and make not current panic: {}, {}", err, err2),
                                Ok(octx) =>
                                    old_surface.ctx = CandlCurrentWrapper::NotCurrent(octx)
                            }
                        }
                    }
                    *old_ref = Takeable::new(old_surface);
                }
                Err(err)
            }
        }
    }

    /// get the data from the manager as an immutable reference
    pub fn data(&self) -> &M { &self.data }

    /// get the data from the manager as a mutable reference
    pub fn data_mut(&mut self) -> &mut M { &mut self.data }
}


*/

