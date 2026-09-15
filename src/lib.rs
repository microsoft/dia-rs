//! Rust bindings for the Windows **DIA** (Debug Interface Access) SDK.
//!
//! `microsoft-dia` wraps the DIA COM interfaces behind an idiomatic,
//! safe Rust API. The raw `windows-bindgen` bindings are kept in a
//! private module; the public surface is only the wrapper objects
//! ([`DataSource`], [`Session`], [`Symbol`], etc.) and the value
//! constants the finders take (`SymTag*`, `ns*`).
//!
//! # Quick start
//!
//! ```no_run
//! use microsoft_dia::{DataSource, NameSearchOptions, SymTag};
//!
//! fn main() -> Result<(), windows_core::Error> {
//!     let source = DataSource::open()?;
//!     source.load_exe("path\\to\\app.exe", None)?;
//!     let session = source.open_session()?;
//!     let global = session.global_scope()?;
//!     for sym in global.find_children(SymTag::Function, "*", NameSearchOptions::NONE)? {
//!         println!("{}", sym.name()?);
//!     }
//!     Ok(())
//! }
//! ```
//!
//! The SDK must be loadable: either the DIA COM server is registered, or
//! `msdia140.dll` is present in one of the well-known locations
//! (see [`DataSource::open`]).

#![doc(html_no_source)]

#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
#[allow(clippy::missing_safety_doc)]
#[allow(clippy::missing_transmute_annotations)]
#[allow(clippy::too_many_arguments)]
#[allow(clippy::upper_case_acronyms)]
mod bindings;

mod com;

pub mod constants;

pub mod callbacks;
pub mod data_source;
pub mod enums;
pub mod records;
pub mod session;
pub mod stack;
pub mod symbol;
pub mod tables;

pub use callbacks::{LoadCallback, LoadCallbackAdapter};
pub use constants::{NameSearchOptions, SymTag};

pub use data_source::DataSource;
pub use enums::{
    DebugStream, DebugStreams, FrameDatas, InjectedSources, InputAssemblyFiles, LineNumbers,
    NamedStreams, SectionContribs, Segments, SourceFiles, SourceLinks, Symbols, SymbolsByAddress,
    SymbolsByAddress2, Tables, Unknowns,
};
pub use records::{
    AddressMap, AddressMapEntry, FrameData, ImageData, InjectedSource, InputAssemblyFile,
    LineNumber, SectionContrib, Segment, SourceFile, TagValue,
};
pub use session::Session;
pub use stack::{
    StackFrame, StackFrames, StackWalkHelper, StackWalkHelper2, StackWalkHelper3,
    StackWalkHelperAdapter, StackWalker,
};
pub use symbol::Symbol;
pub use tables::Table;
