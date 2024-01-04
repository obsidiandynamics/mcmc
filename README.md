`mcmc`
===
Markov chain Monte Carlo sampling using the _Independence Metropolis-Hastings_ algorithm with uniform transition kernel.

Uses the [tinyrand](https://github.com/obsidiandynamics/tinyrand) RNG.

# Example
Draw samples from the Gaussian distribution.

```rust
use std::ops::RangeInclusive;
use tinyrand::Wyrand;
use mcmc::gaussian::Gaussian;
use mcmc::sampler::{Config, Sampler};

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
