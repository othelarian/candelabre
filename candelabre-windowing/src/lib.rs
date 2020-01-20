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

/// The display surface
///
/// The first core element of this crate, the CandlSurface is a window with an
/// OpenGL context, and some options. It sounds very simple, and in fact it is.
/// Look for the example to see how to use it.
pub struct CandlSurface {
    ctx: CandlCurrentWrapper,
    gfx_state: Rc<RefCell<GraphicsState>>
}

unsafe impl GraphicsContext for CandlSurface {
    fn state(&self) -> &Rc<RefCell<GraphicsState>> { &self.gfx_state }
}

impl CandlSurface {
    /// creation of a CandlSurface
    pub fn new<T>(
        el: &EventLoop<T>,
        dim: CandlDimension,
        title: &str,
        options: CandlOptions
    ) -> Result<Self, CandlError> {
        CandlSurface::window_builder(el, dim, title, options, false)
    }

    /// internal builder for the window
    fn window_builder<T>(
        el: &EventLoop<T>,
        dim: CandlDimension,
        title: &str,
        options: CandlOptions,
        multi: bool
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
            gfx_state: Rc::new(RefCell::new(gfx_state))
        })
    }

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

pub struct CandlManager {
    current: Option<WindowId>,
    surfaces: HashMap<WindowId, Takeable<CandlSurface>>
}

impl CandlManager {
    /// constructor for the manager
    pub fn new() -> Self { CandlManager { current: None, surfaces: HashMap::default() } }

    /// create a new window, tracked by the manager
    /// 
    /// For internal reason, it isn't possible to add a `CandlSurface` manually
    /// created to the manager, it's mandatory to use the `create_window()`
    /// method instead.
    pub fn create_window<T>(
        &mut self,
        el: &EventLoop<T>,
        dim: CandlDimension,
        title: &str,
        options: CandlOptions
    ) -> Result<WindowId, CandlError> {
        let mut surface = CandlSurface::window_builder(el, dim, title, options, true)?;
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
    pub fn get_current() {
        //
        //
    }
}
