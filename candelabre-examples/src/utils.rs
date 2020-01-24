/*
#[derive(Clone, Copy, Debug, Eq, PartialEq, Semantics)]
pub enum Semantics {
    #[sem(name = "co", repr = "[f32; 2]", wrapper = "VertexPosition")]
    Position,
    #[sem(name = "color", repr = "[u8; 3]", wrapper = "VertexColor")]
    Color
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Vertex)]
#[vertex(sem = "Semantics")]
pub struct Vertex {
    pos: VertexPosition,
    #[vertex(normalized = "true")]
    rgb: VertexColor
}

pub const VS: &'static str = include_str!("../resources/simple-vs.glsl");
pub const FS: &'static str = include_str!("../resources/simple-fs.glsl");

pub const OGL_TRIANGLE: [Vertex; 3] = [
    Vertex {pos: VertexPosition::new([-0.5, 0.5]), rgb: VertexColor::new([0, 255, 0])},
    Vertex {pos: VertexPosition::new([-0.0, 0.0]), rgb: VertexColor::new([255, 0, 0])},
    Vertex {pos: VertexPosition::new([0.5, 0.5]), rgb: VertexColor::new([0, 0, 255])}
];

*/

pub struct SurfaceState {
    redraw: bool,
    bgcol: [f32; 4]
}

impl Default for SurfaceState {
    fn default() -> Self { Self {redraw: false, bgcol: [0.0, 0.0, 0.0, 1.0]} }
}
