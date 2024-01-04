use std::ops::RangeInclusive;

use criterion::{Criterion, criterion_group, criterion_main};
use tinyrand::Wyrand;

use metromc::gaussian::Gaussian;
use metromc::sampler::{Config, Sampler};

fn criterion_benchmark(c: &mut Criterion) {
    const MEAN: f64 = 0.0;
    const STD_DEV: f64 = 1.0;
    const RANGE: RangeInclusive<f64> = -5.0..=5.0;

    let mut sampler = Sampler::new(Config {
        rand: Wyrand::default(),
        dist: Gaussian::new(MEAN, STD_DEV),
        range: RANGE,
    });

    // sanity check
    assert!(RANGE.contains(&sampler.next()));

    c.bench_function("cri_gaussian", |b| {
        b.iter(|| sampler.next());
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
