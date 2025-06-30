use printpdf::Op;

pub mod label;
pub mod table;

pub trait Component {
    fn render_at(&self, x: f32, y: f32) -> Vec<Op>;
}