use statrs::distribution::Continuous;
use crate::Pdf;

pub struct Beta {
    dist: statrs::distribution::Beta,
}
impl Beta {
    #[inline(always)]
    pub fn new(alpha: f64, beta: f64) -> Self {
        Self {
            dist: statrs::distribution::Beta::new(alpha, beta).unwrap(),
        }
    }
}

impl Pdf for Beta {
    #[inline(always)]
    fn prob(&self, x: f64) -> f64 {
        self.dist.pdf(x)
    }
}