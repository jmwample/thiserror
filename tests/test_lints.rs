#![allow(clippy::mixed_attributes_style)]

use thiserror::Error;

#[rustversion::since(1.81)]
#[cfg(feature = "no-std")]
pub use core::error::Error;
#[cfg(not(feature = "no-std"))]
pub use std::error::Error;

#[test]
fn test_unused_qualifications() {
    #![deny(unused_qualifications)]

    // Expansion of derive(Error) macro can't know whether something like
    // std::error::Error is already imported in the caller's scope so it must
    // suppress unused_qualifications.

    #[derive(Debug, Error)]
    #[error("...")]
    pub struct MyError;

    let _: MyError;
}
