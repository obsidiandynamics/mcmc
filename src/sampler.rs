use std::ops::RangeInclusive;
use tinyrand::Rand;
use crate::gaussian;

pub struct Config<R: Rand> {
    pub rand: R,
    pub mean: f64,
    pub stdev: f64,
    pub range: RangeInclusive<f64>,
}

pub struct Sampler<R: Rand> {
    config: Config<R>,
    current: f64,
    prob_current: f64,
    span: f64,
}
impl<R: Rand> Sampler<R> {
    pub fn new(config: Config<R>) -> Self {
        let span  = config.range.end() - config.range.start();
        let current = (config.range.start() + config.range.end()) / 2.0;
        let prob_current = gaussian::pdf(config.mean, config.stdev, current);
        Self {
            config,
            current,
            prob_current,
            span,
        }
    }

    #[inline(always)]
    pub fn next(&mut self) -> f64 {
        loop {
            let next = random_f64(&mut self.config.rand) * self.span + self.config.range.start();
            let prob_next = gaussian::pdf(self.config.mean, self.config.stdev, next);
            //println!("current: {:.3}, prob_current: {:.3}, next: {next:.3}, prob_next: {prob_next:.3}", self.current, self.prob_current);
            if prob_next >= self.prob_current {
                self.current = next;
                self.prob_current = prob_next;
                return self.current;
            } else {
                let threshold = prob_next / self.prob_current;
                let rand = random_f64(&mut self.config.rand);
                if rand < threshold {
                    self.current = next;
                    self.prob_current = prob_next;
                    return self.current;
                }
            }
        }
    }
}

#[inline(always)]
fn random_f64(rand: &mut impl Rand) -> f64 {
    rand.next_u64() as f64 / u64::MAX as f64
}