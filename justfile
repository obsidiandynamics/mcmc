_help:
    @just --list

# list values of the Gaussian pdf
gaussian_pdf:
    cargo run --release --example gaussian_pdf

# list values of the Gamma pdf
gamma_pdf:
    cargo run --release --example gamma_pdf

# sample from the Gaussian distribution
sample_gaussian:
    cargo run --release --example sample_gaussian

# sample from the Gamma distribution
sample_gamma:
    cargo run --release --example sample_gamma

# sample from the Pareto distribution
sample_pareto:
    cargo run --release --example sample_pareto

# sample from the Beta distribution
sample_beta:
    cargo run --release --example sample_beta

# run Criterion bechmarks
bench:
    bash -c 'type cargo-criterion >/dev/null 2>&1 || cargo install cargo-criterion'
    cargo criterion

# run the tests
test:
    cargo test -- --include-ignored
    cargo test --examples
    cargo doc --no-deps
    cargo bench --no-run --profile dev
