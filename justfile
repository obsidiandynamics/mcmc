_help:
    @just --list

# list values of a Gaussian pdf
gaussian_pdf:
    cargo run --release --example gaussian_pdf

# list values of a Gamma pdf
gamma_pdf:
    cargo run --release --example gamma_pdf

# sample from a Gaussian distribution
sample_gaussian:
    cargo run --release --example sample_gaussian

# sample from a Gamma distribution
sample_gamma:
    cargo run --release --example sample_gamma

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
