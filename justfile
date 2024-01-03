_help:
    @just --list

# list values of a Gaussian pdf
gaussian_pdf:
    cargo run --release --example gaussian_pdf

# sample from a Gaussian distribution
sample_gaussian:
    cargo run --release --example sample_gaussian
