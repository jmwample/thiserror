#![allow(
    clippy::blocks_in_conditions,
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::manual_find,
    clippy::manual_let_else,
    clippy::manual_map,
    clippy::map_unwrap_or,
    clippy::module_name_repetitions,
    clippy::needless_pass_by_value,
    clippy::range_plus_one,
    clippy::single_match_else,
    clippy::struct_field_names,
    clippy::too_many_lines,
    clippy::wrong_self_convention
)]
#![cfg_attr(feature = "no-std", no_std)]

#[rustversion::before(1.81)]
#[cfg(feature = "no-std")]
::core::compile_error!("no-std feature requires rustversion 1.81 or newer");

extern crate proc_macro;

mod ast;
mod attr;
mod expand;
mod fmt;
mod generics;
mod prop;
mod span;
mod valid;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Error, attributes(backtrace, error, from, source))]
pub fn derive_error(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand::derive(&input).into()
}
