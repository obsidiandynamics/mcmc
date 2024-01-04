use statrs::distribution::Continuous;
use crate::Pdf;

pub struct Pareto {
    dist: statrs::distribution::Pareto,
}
impl Pareto {
    #[inline(always)]
    pub fn new(shape: f64, rate: f64) -> Self {
        Self {
            dist: statrs::distribution::Pareto::new(shape, rate).unwrap(),
        }
    }
}

impl Pdf for Pareto {
    #[inline(always)]
    fn prob(&self, x: f64) -> f64 {
        self.dist.pdf(x)
    }
}