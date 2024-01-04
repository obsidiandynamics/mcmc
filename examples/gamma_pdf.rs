use mcmc::gamma::Gamma;
use mcmc::Pdf;

fn main() {
    const SHAPE: f64 = 2.0;
    const RATE: f64 = 0.5;

    let dist = Gamma::new(SHAPE, RATE);
    for x in 0..=16 {
        let fi = dist.prob(x as f64);
        println!("{x:2.3}, {fi:.6}");
    }
}