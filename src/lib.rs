#![doc = include_str!("../.github/readme.md")]
#![doc(html_no_source)]

#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
#[allow(clippy::missing_safety_doc)]
#[allow(clippy::missing_transmute_annotations)]
#[allow(clippy::too_many_arguments)]
mod bindings;

pub mod helpers;
pub use bindings::*;
