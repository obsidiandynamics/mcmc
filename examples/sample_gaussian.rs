use std::ops::RangeInclusive;
use tinyrand::Wyrand;
use mcmc::sampler::{Config, Sampler};

fn main() {
    const MEAN: f64 = 0.0;
    const STDEV: f64 = 1.0;
    const RANGE: RangeInclusive<f64> = -5.0..=5.0;
    const NUM_BUCKETS: usize = 100;
    const NUM_SAMPLES: usize = 10_000_000;

    let mut sampler = Sampler::new(Config {
        rand: Wyrand::default(),
        mean: MEAN,
        stdev: STDEV,
        range: RANGE,
    });

    let mut buckets = [0_usize; NUM_BUCKETS];
    let span = RANGE.end() - RANGE.start();
    for _ in 0..NUM_SAMPLES {
        let rand = sampler.next();
        // println!("{rand:.6}");
        let bucket = ((rand - RANGE.start()) / span * NUM_BUCKETS as f64) as usize;
        buckets[bucket] += 1;
    }

    println!("bucket start, count");
    println!("------------, -----");

    for (bucket, count) in buckets.iter().enumerate() {
        let bucket_start = RANGE.start() + bucket as f64 / NUM_BUCKETS as f64 * span;
        println!("{bucket_start:.3}, {}", count);
    }
}