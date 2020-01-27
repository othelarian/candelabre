//! # Welcome!
//!
//! The purpose of this crate is to provide a way to easily handle windows,
//! just one or a multitude of them. After creating the window, it's simple to
//! use the candelabre-core and candelabre-widget to quickly construct a
//! beautiful GUI.
//! 
//! # A bit of history
//! 
//! At the beginning, this crate was designed to use glutin v0.22 with
//! [luminance](https://github.com/phaazon/luminance-rs/), but due to some
//! foolish idea from the developper, the initial goal slide a bit. Now the
//! purpose of candelabre-windowing is mostly to support candelabre-widget, but
//! you can still use it solely with `gl`, `rgl`, or any lib who play with
//! OpenGL.
//! 
//! # What's inside this crate?
//! 
//! This crate provide a two elements:
//!
//! * `CandlSurface`, a window type who generate a surface for using OpenGL
//! * `CandlManager`, a window manager to enable using multiple windows in a
//! single application / thread
//! * `CandlWindow`, a trait on which `CandlSurface` is based on
//!
//! ## `CandlSurface`
//! 
//! Core of this lib, it's a simple way to get an OpenGL context and then use
//! luminance [luminance](https://github.com/phaazon/luminance-rs/). Initially
//! it's a copy of luminance-glutin lib, modified to be able to work with the
//! CandlManager.
//! 
//! ## `CandlManager`
//! 
//! When dealing with multiple windows in a single application, it quickly
//! become complex and error prone. You can only use one OpenGL context at a
//! time, and must so you need to swap between each contexts when you update
//! what you display. With `CandlManager`, you have what you need to help you
//! in this tedious task. It take the responsability to make the swap for you,
//! and track each window you link to it.
//! 
//! ## `CandlWindow`
//! 
//! This trait was added to let developpers create their own implementation of
//! `CandlSurface`, like using luminance to handle the OpenGL context, or get
//! rid of the stateful capability of the `CandlSurface`. If an application
//! need a simple way to handle a window, maybe the `CandlWindow` is the right
//! tool to do it.
//! 
//! ## About data in `CandlSurface` and `CandlManager`
//! 
//! It's possible to add data into the `CandlSurface` and the `CandlManager`.
//! The purpose of this data is to make this structures stateful, but there is
//! some limitations. It's strongly discouraged to save OpenGL data, like the
//! tess, the program, or the shader, in the data of the structures, this isn't
//! the purpose of this data, and can lead to useless complexity due to the
//! borrowing and onwership when it come to the render phase.
//! 

#![deny(missing_docs)]

use candelabre_core::CandlGraphics;
use gl;
use glutin::{
    Api, ContextBuilder, GlProfile, GlRequest, NotCurrent,
    PossiblyCurrent, WindowedContext
};
use glutin::dpi::LogicalSize;
use glutin::event_loop::EventLoop;
use glutin::window::{Fullscreen, WindowBuilder, WindowId};
use std::collections::HashMap;
use std::fmt;
use std::os::raw::c_void;
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

/// Tracking the context status
///
/// When working with OpenGL context it's important to know if the context you
/// working with is the current one or not. If you're using only one window,
/// it's ok to avoid this enum and only use `PossiblyCurrent`, because the
/// context status will never change. But if you need multiple windows, you
/// need to know if the context you want to work with is the current one, and
/// if not you need to change that. The `CandlManager` is here to do that for
/// you, and use `CandlCurrentWrapper` to do so.
#[derive(Debug)]
pub enum CandlCurrentWrapper {
    /// OpenGL context is probably current
    PossiblyCurrent(WindowedContext<PossiblyCurrent>),
    /// OpenGL context is not current
    NotCurrent(WindowedContext<NotCurrent>)
}

// =======================================================================
// =======================================================================
//               CandlWindow
// =======================================================================
// =======================================================================

/// Window trait
///
/// This trait can be used to create a new kind of `CandlSurface`, with a
/// deeper connection with your code.
pub trait CandlWindow {
    /// code to init the basis of a window with an OpenGL context
    fn init<T>(
        el: &EventLoop<T>,
        dim: CandlDimension,
        title: &str,
        options: CandlOptions
    ) -> Result<WindowedContext<PossiblyCurrent>, CandlError> where T: 'static {
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
        Ok(ctx)
    }

    /// get the OpenGL context wrapper
    fn ctx(&mut self) -> CandlCurrentWrapper;

    /// get a reference to the OpenGL context wrapper
    fn ctx_ref(&self) -> &CandlCurrentWrapper;

    /// change the OpenGL context
    fn set_ctx(&mut self, nctx: CandlCurrentWrapper);

    /// swap the buffer
    fn swap_buffers(&mut self);
}

// =======================================================================
// =======================================================================
//               CandlSurfaceBuilder
// =======================================================================
// =======================================================================

/// Surface builder
/// 
/// This builder help create a new `CandlSurface` in a more idiomatic way
pub struct CandlSurfaceBuilder<'a, D> {
    dim: CandlDimension,
    title: &'a str,
    options: CandlOptions,
    render: Option<CandlGraphics>,
    state: Option<D>
}

impl<'a, 'b, D> CandlSurfaceBuilder<'a, D> {
    /// builder constructor
    ///
    /// By default, the builder set the window dimension to Classic(800, 400)
    /// and with no name
    pub fn new() -> Self {
        CandlSurfaceBuilder {
            dim: CandlDimension::Classic(800, 400),
            title: "",
            options: CandlOptions::default(),
            render: None,
            state: None
        }
    }

    /// modify the starting dimension
    pub fn dim(self, dim: CandlDimension) -> Self { Self {dim, ..self} }

    /// set a title ("" by default)
    pub fn title(self, title: &'a str) -> Self { Self {title, ..self} }

    /// modify the options
    pub fn options(self, options: CandlOptions) -> Self { Self {options, ..self} }

    /// set render object
    pub fn render(self, render: CandlGraphics) -> Self {
        Self {render: Some(render), ..self}
    }

    /// change the initial state
    pub fn state(self, init_state: D) -> Self {
        Self {state: Some(init_state), ..self}
    }

    /// try to build the surface
    pub fn build<T>(self, el: &EventLoop<T>) -> Result<CandlSurface<D>, CandlError> {
        match self.render {
            None =>
                Err(CandlError::InternalError("You must specify the CandlGraphics!")),
            Some(render) =>
                CandlSurface::window_builder(
                    el,
                    self.dim,
                    self.title,
                    self.options,
                    render,
                    self.state
                )
        }
    }
}

// =======================================================================
// =======================================================================
//               CandlSurface
// =======================================================================
// =======================================================================

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
#[derive(Debug)]
pub struct CandlSurface<D> {
    ctx: Option<CandlCurrentWrapper>,
    render: CandlGraphics,
    state: Option<D>
}

impl<D> CandlWindow for CandlSurface<D> {
    /// get the OpenGL context from the surface
    fn ctx(&mut self) -> CandlCurrentWrapper { self.ctx.take().unwrap() }

    /// get the reference to the OpenGL context
    fn ctx_ref(&self) -> &CandlCurrentWrapper { self.ctx.as_ref().unwrap() }

    /// change the OpenGL context (make current or not current)
    fn set_ctx(&mut self, nctx: CandlCurrentWrapper) { self.ctx = Some(nctx); }

    /// swap the OpenGL back buffer and current buffer
    fn swap_buffers(&mut self) {
        if let CandlCurrentWrapper::PossiblyCurrent(ctx) = self.ctx.as_ref().unwrap() {
            ctx.swap_buffers().unwrap();
        }
    }
}

impl<D> CandlElement<CandlSurface<D>> for CandlSurface<D> {
    /// build method to make `CandlSurface` compatible with the `CandlManager`
    /// 
    /// WARNING: avoid at all cost the use of this method, prefer the builder
    /// instead.
    fn build<T>(
        el: &EventLoop<T>,
        dim: CandlDimension,
        title: &str,
        options: CandlOptions
    ) -> Result<CandlSurface<D>, CandlError> {
        <CandlSurface<D>>::window_builder(el, dim, title, options, CandlGraphics::new(), None)
    }
}

impl<D> CandlSurface<D> {
    /// standard creation of a CandlSurface
    pub fn new<T>(
        el: &EventLoop<T>,
        dim: CandlDimension,
        title: &str,
        options: CandlOptions,
        render: CandlGraphics
    ) -> Result<Self, CandlError> {
        CandlSurface::window_builder(
            el,
            dim,
            title,
            options,
            render,
            None
        )
    }

    /// constructor with data
    ///
    /// This constructor can be used to associate a data type to the window.
    /// The data type must be specified.
    pub fn new_with_data<T>(
        el: &EventLoop<T>,
        dim: CandlDimension,
        title: &str,
        options: CandlOptions,
        render: CandlGraphics,
        init_state: D
    ) -> Result<Self, CandlError> {
        CandlSurface::window_builder(
            el,
            dim,
            title,
            options,
            render,
            Some(init_state)
        )
    }

    /// internal builder for the window
    fn window_builder<T>(
        el: &EventLoop<T>,
        dim: CandlDimension,
        title: &str,
        options: CandlOptions,
        render: CandlGraphics,
        init_state: Option<D>
    ) -> Result<Self, CandlError> {
        let ctx = <CandlSurface<D>>::init(el, dim, title, options)?;
        let ctx = Some(CandlCurrentWrapper::PossiblyCurrent(ctx));
        Ok(CandlSurface {ctx, render, state: init_state})
    }

    /// change the title of the window
    pub fn title(&mut self, new_title: &str) {
        match self.ctx.as_ref().unwrap() {
            CandlCurrentWrapper::PossiblyCurrent(ctx) =>
                ctx.window().set_title(new_title),
            CandlCurrentWrapper::NotCurrent(ctx) =>
                ctx.window().set_title(new_title)
        };
    }

    /// get the render object (immutable way)
    pub fn render(&self) -> &CandlGraphics { &self.render }

    /// get the render object (mutable way)
    pub fn render_mut(&mut self) -> &mut CandlGraphics { &mut self.render }

    /// get the data as a immutable reference
    pub fn state(&self) -> &Option<D> { &self.state }

    /// get the data as a mutable reference
    pub fn state_mut(&mut self) -> &mut Option<D> { &mut self.state }

    /// requesting redraw for the window
    pub fn request_redraw(&mut self) {
        match self.ctx.as_ref().unwrap() {
            CandlCurrentWrapper::PossiblyCurrent(ctx) =>
                ctx.window().request_redraw(),
            CandlCurrentWrapper::NotCurrent(_) => ()
        }
    }

    /*
    //
    // TODO : do something with this
    //
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
    */

    /// draw on the surface
    pub fn draw(&mut self) {
        //
        self.render.draw_frame();
        //
        //(self.render_fn)();
        //
        //let back_buffer = self.back_buffer().unwrap();
        //let rdr_data = self.rdr_data();
        //
        /*
        self.pipeline_builder().pipeline(
            &back_buffer,
            //
            // TODO : integrate the pipelineState inside the surface
            //
            &PipelineState::default(),
            //
            c
            //
        );
        */
        //
        self.swap_buffers();
    }
}

// =======================================================================
// =======================================================================
//               CandlManager
// =======================================================================
// =======================================================================

/// `CandlElement` trait
/// 
/// This trait's purpose is to enable using `CandlWindow` in the manager
/// without too many boilerplate in the surface side. With this code it's easy
/// to just implement both `CandlWindow` and `CandlElement` traits on a single
/// type to make it usable with the `CandlManager`.
pub trait CandlElement<W: CandlWindow> {
    /// to use `CandlWindow` with `CandlManager` the type implementing this
    /// trait must implement the window_builder method
    fn build<T>(
        el: &EventLoop<T>,
        dim: CandlDimension,
        title: &str,
        options: CandlOptions) -> Result<W, CandlError>;
}

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
pub struct CandlManager<W: CandlWindow, M> {
//pub struct CandlManager<D, M> {
    current: Option<WindowId>,
    surfaces: HashMap<WindowId, Takeable<W>>,
    data: M
}

impl<W: CandlWindow> CandlManager<W, ()> {
    /// most default constructor for the manager
    pub fn new() -> Self {
        CandlManager { current: None, surfaces: HashMap::default(), data: () }
    }
}

impl<D, M> CandlManager<CandlSurface<D>, M> {
    /// create a new window from a CandlSurfaceBuilder
    pub fn create_window_from_builder<T>(
        &mut self,
        builder: CandlSurfaceBuilder<D>,
        el: &EventLoop<T>
    ) -> Result<WindowId, CandlError> {
        let surface = builder.build(el).unwrap();
        self.add_window(surface)
    }

    /// create a new window with surface associated data type
    pub fn create_window_with_data<T>(
        &mut self,
        el: &EventLoop<T>,
        dim: CandlDimension,
        title: &str,
        options: CandlOptions,
        render: CandlGraphics,
        init_data: Option<D>
    ) -> Result<WindowId, CandlError> {
        let surface = CandlSurface::window_builder(el, dim, title, options, render, init_data)?;
        self.add_window(surface)
    }
}

impl<W: CandlWindow, M> CandlManager<W, M> {
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
    pub fn create_window<T, E: CandlElement<W>>(
        &mut self,
        el: &EventLoop<T>,
        dim: CandlDimension,
        title: &str,
        options: CandlOptions,
    ) -> Result<WindowId, CandlError> {
        let surface = E::build(el, dim, title, options)?;
        self.add_window(surface)
    }

    /// constructor for the manager with data type link to it
    pub fn new_with_data(init_data: M) -> Self {
        CandlManager {
            current: None,
            surfaces: HashMap::default(),
            data: init_data
        }
    }

    /// internal method to truly add the new window
    fn add_window(&mut self, mut surface: W) -> Result<WindowId, CandlError> {
        let surface_ctx = surface.ctx();
        match &surface_ctx {
            CandlCurrentWrapper::PossiblyCurrent(ctx) => {
                let win_id = ctx.window().id();
                if let Some(old_id) = self.current.take() {
                    if let Some(old_surface) = self.surfaces.get_mut(&old_id) {
                        let mut old_win = Takeable::take(old_surface);
                        let ctx_wrapper = old_win.ctx();
                        if let CandlCurrentWrapper::PossiblyCurrent(ctx) = ctx_wrapper {
                            let nctx = unsafe { ctx.treat_as_not_current() };
                            old_win.set_ctx(CandlCurrentWrapper::NotCurrent(nctx));
                        } else {
                            old_win.set_ctx(ctx_wrapper);
                        }
                        *old_surface = Takeable::new(old_win);
                    }
                }
                surface.set_ctx(surface_ctx);
                self.surfaces.insert(win_id, Takeable::new(surface));
                self.current = Some(win_id);
                Ok(win_id)
            }
            CandlCurrentWrapper::NotCurrent(_) => {
                Err(CandlError::InternalError(
                    "Surface creation from manager generated a not current context"
                ))
            }
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
    -> Result<&mut W, ContextError> {
        let res = if Some(id) != self.current {
            let ncurr_ref = self.surfaces.get_mut(&id).unwrap();
            let mut ncurr_surface = Takeable::take(ncurr_ref);
            match ncurr_surface.ctx() {
                CandlCurrentWrapper::PossiblyCurrent(_) => {
                    *ncurr_ref = Takeable::new(ncurr_surface);
                    Ok(())
                }
                CandlCurrentWrapper::NotCurrent(nctx) => unsafe {
                    match nctx.make_current() {
                        Err((rctx, err)) => {
                            match rctx.make_not_current() {
                                Ok(rctx) => {
                                    ncurr_surface.set_ctx(CandlCurrentWrapper::NotCurrent(rctx));
                                    *ncurr_ref = Takeable::new(ncurr_surface);
                                    Err(err)
                                }
                                Err((_, err2)) =>
                                    panic!("Couldn't make current and not current: {}, {}", err, err2)
                            }
                        }
                        Ok(rctx) => {
                            ncurr_surface.set_ctx(CandlCurrentWrapper::PossiblyCurrent(rctx));
                            *ncurr_ref = Takeable::new(ncurr_surface);
                            Ok(())
                        }
                    }
                }
            }
        }
        else {
            let ncurr_ref = self.surfaces.get_mut(&id).unwrap();
            let mut ncurr_surface = Takeable::take(ncurr_ref);
            match &ncurr_surface.ctx() {
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
                        if let CandlCurrentWrapper::PossiblyCurrent(octx) = old_surface.ctx() {
                            unsafe { old_surface.set_ctx(CandlCurrentWrapper::NotCurrent(octx.treat_as_not_current())); }
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
                    if let CandlCurrentWrapper::PossiblyCurrent(octx) = old_surface.ctx() {
                        unsafe {
                            match octx.make_not_current() {
                                Err((_, err2)) =>
                                    panic!("make current and make not current panic: {}, {}", err, err2),
                                Ok(octx) =>
                                    old_surface.set_ctx(CandlCurrentWrapper::NotCurrent(octx))
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
