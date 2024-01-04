use statrs::distribution::Continuous;
use crate::Pdf;

pub struct Gamma {
    dist: statrs::distribution::Gamma,
}
impl Gamma {
    #[inline(always)]
    pub fn new(shape: f64, rate: f64) -> Self {
        Self {
            dist: statrs::distribution::Gamma::new(shape, rate).unwrap(),
        }
    }
}

impl Pdf for Gamma {
    #[inline(always)]
    fn prob(&self, x: f64) -> f64 {
        self.dist.pdf(x)
    }
}