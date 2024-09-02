#![allow(clippy::mixed_attributes_style)]

use thiserror::Error;

#[rustversion::since(1.81)]
#[cfg(feature="no-std")]
pub use core::error::Error as _;
#[cfg(not(feature="no-std"))]
pub use std::error::Error as _;

#[test]
fn test_unused_qualifications() {
    #![deny(unused_qualifications)]

    // Expansion of derive(Error) macro can't know whether something like
    // core::error::Error is already imported in the caller's scope so it must
    // suppress unused_qualifications.

    #[derive(Debug, Error)]
    #[error("...")]
    pub struct MyError;

    let _: MyError;
}
