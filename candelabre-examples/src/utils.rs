/*
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Vertex)]
#[vertex(sem = "Semantics")]
pub struct Vertex {
    pos: VertexPosition,
    #[vertex(normalized = "true")]
    rgb: VertexColor
}

pub const OGL_TRIANGLE: [Vertex; 3] = [
    Vertex {pos: VertexPosition::new([-0.5, 0.5]), rgb: VertexColor::new([0, 255, 0])},
    Vertex {pos: VertexPosition::new([-0.0, 0.0]), rgb: VertexColor::new([255, 0, 0])},
    Vertex {pos: VertexPosition::new([0.5, 0.5]), rgb: VertexColor::new([0, 0, 255])}
];

*/

use rand::Rng;

pub const VS: &'static str = include_str!("../resources/simple-vs.glsl");
pub const FS: &'static str = include_str!("../resources/simple-fs.glsl");

#[allow(dead_code)]
pub fn new_nb() -> f32 {
    (rand::thread_rng().gen_range(0, 100) as f32) / 100.0
}

#[allow(dead_code)]
pub struct SurfaceState {
    redraw: bool,
    value: u32,
    bgcol: [f32; 4]
}

impl Default for SurfaceState {
    fn default() -> Self {
        Self {
            redraw: false,
            value: 0,
            bgcol: [0.0, 0.0, 0.0, 1.0]
        }
    }
}

#[allow(dead_code)]
impl SurfaceState {
    pub fn ask_draw(&mut self) { self.redraw = true; }
    pub fn need_redraw(&self) -> bool { self.redraw }
    pub fn draw_asked(&mut self) { self.redraw = false; }

    pub fn value(&mut self) -> &mut u32 { &mut self.value }
}
