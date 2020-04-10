use candelabre_windowing::{CandlRenderer, CandlSurface, CandlUpdate};
use gl;
use nvg_gl::Renderer as NvgRenderer;
use nvg::Context as NvgContext;
use rand::Rng;

#[allow(dead_code)]
pub const VS: &'static str = include_str!("../resources/simple-vs.glsl");
#[allow(dead_code)]
pub const FS: &'static str = include_str!("../resources/simple-fs.glsl");

fn new_rf() -> f32 {
    (rand::thread_rng().gen_range(0, 101) as f32) / 100.0
}

fn new_ri() -> u8 { rand::thread_rng().gen_range(0, 255) }

#[allow(dead_code)]
pub struct SurfaceState {
    bg_color: [f32; 3],
    triangle_color: [u8; 3],
    value: u32
}

impl Default for SurfaceState {
    fn default() -> Self {
        Self {
            bg_color: [0.0, 0.0, 0.0],
            triangle_color: [255, 100, 0],
            value: 0
        }
    }
}

#[allow(dead_code)]
pub enum Message {
    IncValue,
    RandomBgColor,
    RandomTriangleColor
}

impl CandlUpdate<Message> for SurfaceState {
    fn update(&mut self, message: Message) {
        match message {
            Message::IncValue =>
                self.value = if self.value == 4 { 0 } else { self.value+1 },
            Message::RandomBgColor =>
                self.bg_color = [new_rf(), new_rf(), new_rf()],
            Message::RandomTriangleColor =>
                self.triangle_color = [new_ri(), new_ri(), new_ri()]
        }
    }
}

#[allow(dead_code)]
impl SurfaceState {
    pub fn get_value(&self) -> u32 { self.value.clone() }

    pub fn get_bg_color(&self) -> &[f32; 3] { &self.bg_color }

    pub fn get_triangle_color(&self) -> &[u8; 3] { &self.triangle_color }
}

#[allow(dead_code)]
pub struct SurfaceDrawer {
    context: Option<NvgContext<NvgRenderer>>,
    size: (u32, u32),
    factor: f64
}

impl CandlRenderer<SurfaceDrawer, SurfaceState, Message> for SurfaceDrawer {
    fn init() -> Self {
        Self {
            context: None,
            size: (0, 0),
            factor: 0.0
        }
    }

    fn finalize(&mut self) {
        let renderer = NvgRenderer::create().unwrap();
        let context = NvgContext::create(renderer).unwrap();
        self.context = Some(context);
    }

    fn set_scale_factor(&mut self, scale_factor: f64) { self.factor = scale_factor; }

    fn set_size(&mut self, size: (u32, u32)) {
        self.size = size;
        let (w, h) = size;
        unsafe { gl::Viewport(0, 0, w as i32, h as i32); }
    }

    fn draw_frame(&mut self, state: &SurfaceState) {
        let bg_col = state.get_bg_color();
        let tr_col = state.get_triangle_color();
        let (w, h) = self.size;
        unsafe {
            gl::ClearColor(bg_col[0], bg_col[1], bg_col[2], 1.0);
            gl::Clear(
                gl::COLOR_BUFFER_BIT |
                gl::DEPTH_BUFFER_BIT |
                gl::STENCIL_BUFFER_BIT
            );
        }
        if let Some(ctxt) = &mut self.context {
            ctxt.begin_frame(
                nvg::Extent::new(w as f32, h as f32),
                self.factor as f32
            ).unwrap();
            ctxt.save();
            ctxt.begin_path();
            ctxt.move_to(nvg::Point::new(50.0, 50.0));
            ctxt.line_to(nvg::Point::new(150.0, 50.0));
            ctxt.line_to(nvg::Point::new(50.0, 150.0));
            ctxt.close_path();
            ctxt.fill_paint(nvg::Color::rgb_i(tr_col[0], tr_col[1], tr_col[2]));
            ctxt.fill().unwrap();
            ctxt.restore();
            ctxt.end_frame().unwrap();
        }
    }
}

#[allow(dead_code)]
pub type DemoSurface = CandlSurface<SurfaceDrawer, SurfaceState, Message>;
