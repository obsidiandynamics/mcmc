use mcmc::Pdf;
use mcmc::gaussian::Gaussian;

fn main() {
    const MEAN: f64 = 3.0;
    const STD_DEV: f64 = 2.0;

    let dist = Gaussian::new(MEAN, STD_DEV);
    for x in -4..=10 {
        let fi = dist.prob(x as f64);
        println!("{x:2.3}, {fi:.6}");
    }
}