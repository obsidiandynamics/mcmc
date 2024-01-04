use std::ops::RangeInclusive;

use tinyrand::Wyrand;
use metromc::beta::Beta;

use metromc::sampler::{Config, Sampler};

fn main() {
    const ALPHA: f64 = 0.5;
    const BETA: f64 = 0.5;
    const RANGE: RangeInclusive<f64> = 0.0..=1.0;
    const NUM_BUCKETS: usize = 1_000;

    let sampler = Sampler::new(Config {
        rand: Wyrand::default(),
        dist: Beta::new(ALPHA, BETA),
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