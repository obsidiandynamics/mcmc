use std::ops::RangeInclusive;

use tinyrand::Wyrand;

use metromc::pareto::Pareto;
use metromc::sampler::{Config, Sampler};

fn main() {
    const SHAPE: f64 = 2.0;
    const RATE: f64 = 1.0;
    const RANGE: RangeInclusive<f64> = 0.0..=16.0;
    const NUM_BUCKETS: usize = 1_000;

    let sampler = Sampler::new(Config {
        rand: Wyrand::default(),
        dist: Pareto::new(SHAPE, RATE),
        range: RANGE,
    });

    let mut buckets = [0_usize; NUM_BUCKETS];
    let span = RANGE.end() - RANGE.start();
    for sample in sampler.skip(100).take(10_000_000) {
        // println!("{rand:.6}");
        let bucket = ((sample - RANGE.start()) / span * NUM_BUCKETS as f64) as usize;
        buckets[bucket] += 1;
    }

    println!("bucket start, count");
    println!("------------, -----");

    for (bucket, count) in buckets.iter().enumerate() {
        let bucket_start = RANGE.start() + bucket as f64 / NUM_BUCKETS as f64 * span;
        println!("{bucket_start:.3}, {}", count);
    }
}