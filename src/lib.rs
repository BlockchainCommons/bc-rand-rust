#![doc(html_root_url = "https://docs.rs/bc-rand/0.2.0")]
#![warn(rust_2018_idioms)]

//! # Blockchain Commons Random Number Utilities
//!
//!
//! `bc-rand` exposes a uniform API for the random number primitives used
//! in higher-level [Blockchain Commons](https://blockchaincommons.com)
//! projects, including a cryptographically strong random number generator
//! [`SecureRandomNumberGenerator`] and a deterministic random number generator
//! [`SeededRandomNumberGenerator`].
//!
//! These primitive random number generators implement the
//! [`RandomNumberGenerator`] trait to produce random numbers compatible with
//! the `RandomNumberGenerator` Swift protocol used in MacOS and iOS, which is
//! important when using the deterministic random number generator for
//! cross-platform testing.
//!
//! The crate also includes several convenience functions for generating secure
//! and deterministic random numbers.

mod magnitude;
mod widening;

mod random_number_generator;
pub use random_number_generator::{
    RandomNumberGenerator,
    rng_next_with_upper_bound,
    rng_next_in_range,
    rng_next_in_closed_range,
    rng_random_data,
    rng_fill_random_data,
};

mod secure_random;
pub use secure_random::{
    SecureRandomNumberGenerator,
    random_data,
    fill_random_data
};

mod seeded_random;
pub use seeded_random::{
    SeededRandomNumberGenerator,
    fake_random_data,
    make_fake_random_number_generator
};

#[cfg(test)]
mod tests {
    #[test]
    fn test_readme_deps() {
        version_sync::assert_markdown_deps_updated!("README.md");
    }

    #[test]
    fn test_html_root_url() {
        version_sync::assert_html_root_url_updated!("src/lib.rs");
    }
}
