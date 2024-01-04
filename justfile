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
