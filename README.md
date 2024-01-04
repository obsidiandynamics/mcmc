`metromc`
===
Markov chain Monte Carlo sampling using the _Independence Metropolis-Hastings_ algorithm with uniform transition kernel.

[![Crates.io](https://img.shields.io/crates/v/mcmc?style=flat-square&logo=rust)](https://crates.io/crates/mcmc)
[![docs.rs](https://img.shields.io/badge/docs.rs-mcmc-blue?style=flat-square&logo=docs.rs)](https://docs.rs/mcmc)
[![Build Status](https://img.shields.io/github/actions/workflow/status/obsidiandynamics/mcmc/master.yml?branch=master&style=flat-square&logo=github)](https://github.com/obsidiandynamics/mcmc/actions/workflows/master.yml)

Uses the [tinyrand](https://github.com/obsidiandynamics/tinyrand) RNG to sample at rate of ~50M samples/sec.

# Example
Draw samples from the Gaussian distribution.

```rust
use std::ops::RangeInclusive;
use tinyrand::Wyrand;
use metromc::gaussian::Gaussian;
use metromc::sampler::{Config, Sampler};

// sample from Gaussian with µ=0.0 and σ=1.0, in the interval [-5.0, 5.0]
let sampler = Sampler::new(Config {
    rand: Wyrand::default(),
    dist: Gaussian::new(0.0, 1.0),
    range: -5.0..=5.0,
});

// take 1,000 samples after dropping the first 10
for sample in sampler.skip(10).take(1_000) {
    println!("{sample:.6}");
}
```
