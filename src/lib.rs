pub mod gamma;
pub mod gaussian;
pub mod sampler;

pub trait Pdf {
    fn prob(&self, x: f64) -> f64;
}