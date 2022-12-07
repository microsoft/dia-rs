#![doc(html_no_source)]

extern crate windows;

pub mod helpers;

#[allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    clippy::useless_transmute,
    clippy::extra_unused_lifetimes,
    clippy::derivable_impls,
    clippy::missing_safety_doc,
    clippy::too_many_arguments
)]
pub mod bindings;
pub use bindings::*;
