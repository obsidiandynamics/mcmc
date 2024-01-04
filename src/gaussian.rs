use statrs::distribution::Continuous;
use crate::Pdf;

pub struct Gaussian {
    dist: statrs::distribution::Normal
}
impl Gaussian {
    #[inline(always)]
    pub fn new(mean: f64, std_dev: f64) -> Self {
        Self {
            dist: statrs::distribution::Normal::new(mean, std_dev).unwrap()
        }
    }
}

impl Pdf for Gaussian {
    #[inline(always)]
    fn prob(&self, x: f64) -> f64 {
        self.dist.pdf(x)
    }
}

#[cfg(test)]
mod tests {
    use assert_float_eq::*;
    use crate::Pdf;

    #[test]
    fn unit_distribution() {
        const MEAN: f64 = 0.0;
        const STD_DEV: f64 = 1.0;
        fn pdf(x: f64) -> f64 {
            super::Gaussian::new(MEAN, STD_DEV).prob(x)
        }

        assert_float_absolute_eq!(0.000001, pdf(-5.0));
        assert_float_absolute_eq!(0.000134, pdf(-4.0));
        assert_float_absolute_eq!(0.004432, pdf(-3.0));
        assert_float_absolute_eq!(0.053991, pdf(-2.0));
        assert_float_absolute_eq!(0.241971, pdf(-1.0));
        assert_float_absolute_eq!(0.398942, pdf(-0.0));
        assert_float_absolute_eq!(0.241971, pdf(1.0));
        assert_float_absolute_eq!(0.053991, pdf(2.0));
        assert_float_absolute_eq!(0.004432, pdf(3.0));
        assert_float_absolute_eq!(0.000134, pdf(4.0));
        assert_float_absolute_eq!(0.000001, pdf(5.0));
    }

    #[test]
    fn custom_distribution() {
        const MEAN: f64 = 3.0;
        const STD_DEV: f64 = 2.0;
        fn pdf(x: f64) -> f64 {
            super::Gaussian::new(MEAN, STD_DEV).prob(x)
        }

        assert_float_absolute_eq!(0.000436, pdf(-4.0));
        assert_float_absolute_eq!(0.002216, pdf(-3.0));
        assert_float_absolute_eq!(0.008764, pdf(-2.0));
        assert_float_absolute_eq!(0.026995, pdf(-1.0));
        assert_float_absolute_eq!(0.064759, pdf(0.0));
        assert_float_absolute_eq!(0.120985, pdf(1.0));
        assert_float_absolute_eq!(0.176033, pdf(2.0));
        assert_float_absolute_eq!(0.199471, pdf(3.0));
        assert_float_absolute_eq!(0.176033, pdf(4.0));
        assert_float_absolute_eq!(0.120985, pdf(5.0));
        assert_float_absolute_eq!(0.064759, pdf(6.0));
        assert_float_absolute_eq!(0.026995, pdf(7.0));
        assert_float_absolute_eq!(0.008764, pdf(8.0));
        assert_float_absolute_eq!(0.002216, pdf(9.0));
        assert_float_absolute_eq!(0.000436, pdf(10.0));
    }
}