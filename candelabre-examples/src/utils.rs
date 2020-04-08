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

use candelabre_experiment::CandlGraphicsDrawer;
use candelabre_windowing::CandlUpdate;

use rand::Rng;

#[allow(dead_code)]
pub const VS: &'static str = include_str!("../resources/simple-vs.glsl");
#[allow(dead_code)]
pub const FS: &'static str = include_str!("../resources/simple-fs.glsl");

#[allow(dead_code)]
pub fn new_nb() -> f32 {
    (rand::thread_rng().gen_range(0, 100) as f32) / 100.0
}

#[allow(dead_code)]
pub struct SurfaceState {
    value: u32,
}

impl Default for SurfaceState {
    fn default() -> Self {
        Self {
            value: 0
        }
    }
}

#[allow(dead_code)]
pub enum Message {
    IncValue
    //
}

impl CandlUpdate<Message> for SurfaceState {
    fn update(&mut self, message: Message) {
        match message {
            Message::IncValue =>
                self.value = if self.value == 4 { 0 } else { self.value+1 }
        }
    }
}

#[allow(dead_code)]
impl SurfaceState {
    pub fn get_value(&self) -> u32 { self.value.clone() }
}

#[allow(dead_code)]
pub struct SurfaceDrawer;

impl CandlGraphicsDrawer<SurfaceState, Message, ()> for SurfaceDrawer {
    fn execute(&self, _: Option<&SurfaceState>, _: Option<&()>) {
        //
        //
    }
}
