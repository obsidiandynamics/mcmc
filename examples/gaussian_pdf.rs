use mcmc::gaussian;

fn main() {
    const MEAN: f64 = 3.0;
    const STDEV: f64 = 2.0;

    for x in -4..=10 {
        let fi = gaussian::pdf(MEAN, STDEV, x as f64);
        println!("{x:2.3}, {fi:.6}");
    }
}