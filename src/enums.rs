//! Typed enumerators over DIA collections, each implementing Rust's
//! `Iterator` (one item per step) alongside fallible COM access methods.

use crate::bindings::*;
use crate::records::{
    FrameData, InjectedSource, InputAssemblyFile, LineNumber, SectionContrib, Segment, SourceFile,
};
use crate::symbol::Symbol;
use windows_core::*;

/// Declares a typed enumerator newtype wrapping a raw DIA `IDiaEnum*`
/// interface. Generates `Clone`, `Debug`, `Iterator`, `len()`, `item()`,
/// `next_item()`, `skip()` and `reset()`.
///
/// `$raw` is the enumerator interface (e.g. `IDiaEnumSymbols`); `$item_raw`
/// is the raw interface of a single item (e.g. `IDiaSymbol`); `$item` is the
/// Rust wrapper type (e.g. `Symbol`).
macro_rules! item_enum {
    ($(#[$meta:meta])* $name:ident, $raw:ty, $item_raw:ty, $item:ty) => {
        $(#[$meta])*
        #[derive(Debug)]
        pub struct $name(pub(crate) $raw);

        impl Clone for $name {
            /// A fresh enumerator at the same position, via COM `Clone()`.
            fn clone(&self) -> Self {
                unsafe { Self(self.0.Clone().expect("DIA enumerator COM Clone")) }
            }
        }

        impl $name {
            /// Retrieves the number of items in the enumeration.
            pub fn len(&self) -> Result<u32> {
                unsafe { Ok(self.0.Count()? as u32) }
            }

            /// Returns whether the enumeration is empty.
            pub fn is_empty(&self) -> Result<bool> {
                Ok(self.len()? == 0)
            }

            /// Retrieves an item by index.
            pub fn item(&self, index: u32) -> Result<$item> {
                unsafe { Ok(Self::wrap(self.0.Item(index)?)) }
            }

            /// Advances the enumerator by one item, returning a `Result`.
            ///
            /// `None` is a clean end of enumeration; `Err` is a real COM failure.
            /// Prefer this over the infallible [`Iterator::next`] when you need to
            /// distinguish a corrupt-PDB error from a clean stop.
            pub fn next_item(&mut self) -> Result<Option<$item>> {
                unsafe {
                    let mut slot: Option<$item_raw> = None;
                    let mut fetched: u32 = 0;
                    let hr = self.0.Next(1, &mut slot, &mut fetched);
                    if fetched == 0 {
                        hr.ok()?;
                        Ok(None)
                    } else {
                        hr.ok()?;
                        match slot {
                            Some(t) => Ok(Some(Self::wrap(t))),
                            None => Err(Error::from(HRESULT(0x80004005u32 as i32))),
                        }
                    }
                }
            }

            /// Skips a specified number of items in the enumeration.
            pub fn skip(&mut self, celt: u32) -> Result<()> {
                unsafe { self.0.Skip(celt).ok() }
            }

            /// Resets the enumeration to the beginning.
            pub fn reset(&mut self) -> Result<()> {
                unsafe { self.0.Reset().ok() }
            }
        }

        impl Iterator for $name {
            type Item = $item;

            fn next(&mut self) -> Option<Self::Item> {
                self.next_item().ok().flatten()
            }
        }
    };
}

item_enum!(
    /// An enumerator of symbols, as returned by the DIA finders.
    ///
    /// Implements [`Iterator`] with [`Symbol`] items.
    Symbols,
    IDiaEnumSymbols,
    IDiaSymbol,
    Symbol
);

impl Symbols {
    fn wrap(raw: IDiaSymbol) -> Symbol {
        Symbol(raw)
    }
}

item_enum!(
    /// An enumerator of line-number records.
    ///
    /// Implements [`Iterator`] with [`LineNumber`] items.
    LineNumbers,
    IDiaEnumLineNumbers,
    IDiaLineNumber,
    LineNumber
);

impl LineNumbers {
    fn wrap(raw: IDiaLineNumber) -> LineNumber {
        LineNumber(raw)
    }
}

item_enum!(
    /// An enumerator of source files.
    ///
    /// Implements [`Iterator`] with [`SourceFile`] items.
    SourceFiles,
    IDiaEnumSourceFiles,
    IDiaSourceFile,
    SourceFile
);

impl SourceFiles {
    fn wrap(raw: IDiaSourceFile) -> SourceFile {
        SourceFile(raw)
    }
}

/// An enumerator of symbols addressable by location.
///
/// Implements [`Iterator`] with [`Symbol`] items, and also exposes the
/// fallible `prev_item()` and the `symbol_by_*` random-access methods.
///
/// Unlike the other enumerators, `IDiaEnumSymbolsByAddr` does not expose
/// `Count`, `Item`, `Skip` or `Reset` in the DIA interface, so those are
/// not available here.
#[derive(Debug)]
pub struct SymbolsByAddress(pub(crate) IDiaEnumSymbolsByAddr);

impl Clone for SymbolsByAddress {
    /// A fresh enumerator at the same position, via COM `Clone()`.
    fn clone(&self) -> Self {
        unsafe { Self(self.0.Clone().expect("DIA symbols-by-addr COM Clone")) }
    }
}

impl SymbolsByAddress {
    /// Advances the enumerator by one symbol, returning a `Result`.
    ///
    /// `None` is a clean end of enumeration; `Err` is a real COM failure.
    pub fn next_item(&mut self) -> Result<Option<Symbol>> {
        unsafe {
            let mut slot: Option<IDiaSymbol> = None;
            let mut fetched: u32 = 0;
            let hr = self.0.Next(1, &mut slot, &mut fetched);
            if fetched == 0 {
                hr.ok()?;
                Ok(None)
            } else {
                hr.ok()?;
                match slot {
                    Some(t) => Ok(Some(Symbol(t))),
                    None => Err(Error::from(HRESULT(0x80004005u32 as i32))),
                }
            }
        }
    }

    /// Moves the enumerator back one symbol, returning a `Result`.
    pub fn prev_item(&mut self) -> Result<Option<Symbol>> {
        unsafe {
            let mut slot: Option<IDiaSymbol> = None;
            let mut fetched: u32 = 0;
            let hr = self.0.Prev(1, &mut slot, &mut fetched);
            if fetched == 0 {
                hr.ok()?;
                Ok(None)
            } else {
                hr.ok()?;
                match slot {
                    Some(t) => Ok(Some(Symbol(t))),
                    None => Err(Error::from(HRESULT(0x80004005u32 as i32))),
                }
            }
        }
    }

    /// Retrieves the symbol at the given address (section + offset).
    pub fn symbol_by_addr(&self, isect: u32, offset: u32) -> Result<Symbol> {
        unsafe { Ok(Symbol(self.0.symbolByAddr(isect, offset)?)) }
    }

    /// Retrieves the symbol at the given relative virtual address.
    pub fn symbol_by_rva(&self, rva: u32) -> Result<Symbol> {
        unsafe { Ok(Symbol(self.0.symbolByRVA(rva)?)) }
    }

    /// Retrieves the symbol at the given virtual address.
    pub fn symbol_by_va(&self, va: u64) -> Result<Symbol> {
        unsafe { Ok(Symbol(self.0.symbolByVA(va)?)) }
    }
}

impl Iterator for SymbolsByAddress {
    type Item = Symbol;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_item().ok().flatten()
    }
}

/// An enumerator of the tables in a symbol store.
///
/// Implements [`Iterator`] with [`crate::tables::Table`] items.
#[derive(Debug)]
pub struct Tables(pub(crate) IDiaEnumTables);

impl Clone for Tables {
    /// A fresh enumerator at the same position, via COM `Clone()`.
    fn clone(&self) -> Self {
        unsafe { Self(self.0.Clone().expect("DIA tables COM Clone")) }
    }
}

impl Tables {
    /// Retrieves the number of tables.
    pub fn len(&self) -> Result<u32> {
        unsafe { Ok(self.0.Count()? as u32) }
    }

    /// Returns whether the store has no tables.
    pub fn is_empty(&self) -> Result<bool> {
        Ok(self.len()? == 0)
    }

    /// Advances the enumerator by one table, returning a `Result`.
    ///
    /// `None` is a clean end; `Err` is a real COM failure.
    pub fn next_item(&mut self) -> Result<Option<crate::tables::Table>> {
        unsafe {
            let mut slot: Option<IDiaTable> = None;
            let mut fetched: u32 = 0;
            let hr = self.0.Next(1, &mut slot, &mut fetched);
            if fetched == 0 {
                hr.ok()?;
                Ok(None)
            } else {
                hr.ok()?;
                match slot {
                    Some(t) => Ok(Some(crate::tables::Table(t))),
                    None => Err(Error::from(HRESULT(0x80004005u32 as i32))),
                }
            }
        }
    }

    /// Skips a specified number of tables in the enumeration.
    pub fn skip(&mut self, celt: u32) -> Result<()> {
        unsafe { self.0.Skip(celt).ok() }
    }

    /// Resets the enumeration to the beginning.
    pub fn reset(&mut self) -> Result<()> {
        unsafe { self.0.Reset().ok() }
    }
}

impl Iterator for Tables {
    type Item = crate::tables::Table;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_item().ok().flatten()
    }
}

/// An enumerator of raw `IUnknown` items, as returned by
/// [`Table::enumerate`](crate::tables::Table::enumerate).
///
/// DIA table items are heterogeneous, so the enumerator yields opaque
/// `IUnknown` values that the caller must cast to the expected DIA interface.
#[derive(Debug)]
pub struct Unknowns(pub(crate) IEnumUnknown);

impl Clone for Unknowns {
    /// A fresh enumerator at the same position, via COM `Clone()`.
    fn clone(&self) -> Self {
        unsafe { Self(self.0.Clone().expect("DIA unknowns COM Clone")) }
    }
}

impl Unknowns {
    /// Advances the enumerator by one item, returning a `Result`.
    pub fn next_item(&mut self) -> Result<Option<IUnknown>> {
        unsafe {
            let mut slot: Option<IUnknown> = None;
            let mut fetched: u32 = 0;
            let hr = self.0.Next(1, &mut slot, Some(&mut fetched));
            if fetched == 0 {
                hr.ok()?;
                Ok(None)
            } else {
                hr.ok()?;
                match slot {
                    Some(t) => Ok(Some(t)),
                    None => Err(Error::from(HRESULT(0x80004005u32 as i32))),
                }
            }
        }
    }

    /// Skips a specified number of items in the enumeration.
    pub fn skip(&mut self, celt: u32) -> Result<()> {
        unsafe { self.0.Skip(celt).ok() }
    }

    /// Resets the enumeration to the beginning.
    pub fn reset(&mut self) -> Result<()> {
        unsafe { self.0.Reset().ok() }
    }
}

impl Iterator for Unknowns {
    type Item = IUnknown;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_item().ok().flatten()
    }
}

/// An enumerator of source-link records.
///
/// Returned by [`Session::get_source_link_info`](crate::session::Session::get_source_link_info).
///
/// Unlike the other DIA enumerators this one has no typed item interface in
/// this SDK snapshot (`IDiaSourceLink` is not part of the DIA bindings), so
/// the enumerator yields raw bytes instead. Call [`next_item`](Self::next_item)
/// with a fetch count and a destination buffer; the records are packed
/// source-link structures as defined by the compiler.
#[derive(Debug)]
pub struct SourceLinks(pub(crate) IDiaEnumSourceLink);

impl Clone for SourceLinks {
    /// A fresh enumerator at the same position, via COM `Clone()`.
    fn clone(&self) -> Self {
        unsafe { Self(self.0.Clone().expect("DIA source-links COM Clone")) }
    }
}

impl SourceLinks {
    /// Retrieves the number of source-link records.
    pub fn len(&self) -> Result<u32> {
        unsafe { self.0.Count() }
    }

    /// Returns whether there are no source-link records.
    pub fn is_empty(&self) -> Result<bool> {
        Ok(self.len()? == 0)
    }

    /// Fetches up to `count` source-link records into `buf`.
    ///
    /// Returns the number of records actually fetched, or `None` when the
    /// enumeration is exhausted.
    pub fn next_item(&mut self, count: u32, buf: &mut [u8]) -> Result<Option<u32>> {
        unsafe {
            let mut fetched: u32 = 0;
            let hr = self.0.Next(count, &mut fetched, buf.as_mut_ptr());
            hr.ok()?;
            if fetched == 0 {
                Ok(None)
            } else {
                Ok(Some(fetched))
            }
        }
    }

    /// Skips a specified number of records in the enumeration.
    pub fn skip(&mut self, count: u32) -> Result<()> {
        unsafe { self.0.Skip(count).ok() }
    }

    /// Resets the enumeration to the beginning.
    pub fn reset(&mut self) -> Result<()> {
        unsafe { self.0.Reset().ok() }
    }
}

item_enum!(
    /// An enumerator of segments, as returned by the segments table.
    ///
    /// Implements [`Iterator`] with [`Segment`] items.
    Segments,
    IDiaEnumSegments,
    IDiaSegment,
    Segment
);

impl Segments {
    fn wrap(raw: IDiaSegment) -> Segment {
        Segment(raw)
    }
}

item_enum!(
    /// An enumerator of section contributions, as returned by the section-contribs table.
    ///
    /// Implements [`Iterator`] with [`SectionContrib`] items.
    SectionContribs,
    IDiaEnumSectionContribs,
    IDiaSectionContrib,
    SectionContrib
);

impl SectionContribs {
    fn wrap(raw: IDiaSectionContrib) -> SectionContrib {
        SectionContrib(raw)
    }
}

item_enum!(
    /// An enumerator of injected sources, as returned by the injected-sources table.
    ///
    /// Implements [`Iterator`] with [`InjectedSource`] items.
    InjectedSources,
    IDiaEnumInjectedSources,
    IDiaInjectedSource,
    InjectedSource
);

impl InjectedSources {
    fn wrap(raw: IDiaInjectedSource) -> InjectedSource {
        InjectedSource(raw)
    }
}

item_enum!(
    /// An enumerator of input assembly files, as returned by the input-assembly-files table.
    ///
    /// Implements [`Iterator`] with [`InputAssemblyFile`] items.
    InputAssemblyFiles,
    IDiaEnumInputAssemblyFiles,
    IDiaInputAssemblyFile,
    InputAssemblyFile
);

impl InputAssemblyFiles {
    fn wrap(raw: IDiaInputAssemblyFile) -> InputAssemblyFile {
        InputAssemblyFile(raw)
    }
}

item_enum!(
    /// An enumerator of frame-data records, as returned by the frame-data table.
    ///
    /// Implements [`Iterator`] with [`FrameData`] items, and also exposes the
    /// `frame_by_rva()` and `frame_by_va()` random-access methods.
    FrameDatas,
    IDiaEnumFrameData,
    IDiaFrameData,
    FrameData
);

impl FrameDatas {
    fn wrap(raw: IDiaFrameData) -> FrameData {
        FrameData(raw)
    }

    /// Retrieves a frame-data record by relative virtual address (RVA).
    pub fn frame_by_rva(&self, rva: u32) -> Result<FrameData> {
        unsafe { Ok(FrameData(self.0.frameByRVA(rva)?)) }
    }

    /// Retrieves a frame-data record by virtual address (VA).
    pub fn frame_by_va(&self, va: u64) -> Result<FrameData> {
        unsafe { Ok(FrameData(self.0.frameByVA(va)?)) }
    }
}

/// A named debug stream and its raw data.
///
/// Obtained by iterating the [`DebugStreams`](Self) enumerator.
#[derive(Clone, Debug)]
pub struct DebugStream(pub(crate) IDiaEnumDebugStreamData);

impl DebugStream {
    /// Retrieves the stream name.
    pub fn name(&self) -> Result<String> {
        unsafe { Ok(self.0.name()?.display().to_string()) }
    }

    /// Retrieves the number of bytes in the stream.
    pub fn length(&self) -> Result<u32> {
        unsafe { Ok(self.0.Count()? as u32) }
    }

    /// Retrieves the raw bytes of the stream.
    pub fn data(&self) -> Result<Vec<u8>> {
        unsafe {
            let mut size: u32 = 0;
            self.0.Item(0, 0, &mut size, core::ptr::null_mut()).ok()?;
            let mut buf = vec![0u8; size as usize];
            let mut fetched: u32 = 0;
            self.0.Item(0, size, &mut fetched, buf.as_mut_ptr()).ok()?;
            buf.truncate(fetched as usize);
            Ok(buf)
        }
    }
}

/// An enumerator of debug streams, as returned by
/// [`crate::Session::get_enum_debug_streams`].
///
/// Implements [`Iterator`] with [`DebugStream`] items. Unlike the standard
/// enumerators, `IDiaEnumDebugStreams` exposes no `Item`, `Skip` or
/// `Reset` on the enumerator itself (the stream data lives on the
/// `DebugStream` items).
#[derive(Debug)]
pub struct DebugStreams(pub(crate) IDiaEnumDebugStreams);

impl Clone for DebugStreams {
    /// A fresh enumerator at the same position, via COM `Clone()`.
    fn clone(&self) -> Self {
        unsafe { Self(self.0.Clone().expect("DIA debug-streams COM Clone")) }
    }
}

impl DebugStreams {
    /// Retrieves the number of debug streams.
    pub fn len(&self) -> Result<u32> {
        unsafe { Ok(self.0.Count()? as u32) }
    }

    /// Returns whether there are no debug streams.
    pub fn is_empty(&self) -> Result<bool> {
        Ok(self.len()? == 0)
    }

    /// Advances the enumerator by one stream, returning a `Result`.
    ///
    /// `None` is a clean end of enumeration; `Err` is a real COM failure.
    pub fn next_item(&mut self) -> Result<Option<DebugStream>> {
        unsafe {
            let mut slot: Option<IDiaEnumDebugStreamData> = None;
            let mut fetched: u32 = 0;
            let hr = self.0.Next(1, &mut slot, &mut fetched);
            if fetched == 0 {
                hr.ok()?;
                Ok(None)
            } else {
                hr.ok()?;
                match slot {
                    Some(t) => Ok(Some(DebugStream(t))),
                    None => Err(Error::from(HRESULT(0x80004005u32 as i32))),
                }
            }
        }
    }
}

impl Iterator for DebugStreams {
    type Item = DebugStream;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_item().ok().flatten()
    }
}

/// An enumerator of named streams, as returned by
/// [`crate::DataSource::find_named_streams`].
///
/// Implements [`Iterator`] with `String` items (the stream names).
#[derive(Debug)]
pub struct NamedStreams(pub(crate) IDiaEnumNamedStreams);

impl Clone for NamedStreams {
    /// A fresh enumerator at the same position, via COM `Clone()`.
    fn clone(&self) -> Self {
        unsafe { Self(self.0.Clone().expect("DIA named-streams COM Clone")) }
    }
}

impl NamedStreams {
    /// Retrieves the number of named streams.
    pub fn len(&self) -> Result<u32> {
        unsafe { Ok(self.0.Count()? as u32) }
    }

    /// Returns whether there are no named streams.
    pub fn is_empty(&self) -> Result<bool> {
        Ok(self.len()? == 0)
    }

    /// Advances the enumerator by one stream name, returning a `Result`.
    ///
    /// `None` is a clean end of enumeration; `Err` is a real COM failure.
    ///
    /// End of enumeration is signalled by an empty stream name, which
    /// this method reports as `None`.
    pub fn next_item(&mut self) -> Result<Option<String>> {
        unsafe {
            let b = self.0.Next()?;
            if b.is_empty() {
                Ok(None)
            } else {
                Ok(Some(b.display().to_string()))
            }
        }
    }

    /// Skips a specified number of stream names in the enumeration.
    pub fn skip(&mut self, celt: u32) -> Result<()> {
        unsafe { self.0.Skip(celt).ok() }
    }

    /// Resets the enumeration to the beginning.
    pub fn reset(&mut self) -> Result<()> {
        unsafe { self.0.Reset().ok() }
    }
}

impl Iterator for NamedStreams {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_item().ok().flatten()
    }
}

/// An enumerator of symbols addressable by location, with the
/// "promote block symbol" option.
///
/// Implements [`Iterator`] with [`Symbol`] items. Unlike
/// `SymbolsByAddress`, this interface exposes no `Count`, `Item`, `Skip`
/// or `Reset`; use the `symbol_by_*` lookups to position, and
/// `next_ex()`/`prev_ex()` to step.
#[derive(Debug)]
pub struct SymbolsByAddress2(pub(crate) IDiaEnumSymbolsByAddr2);

impl Clone for SymbolsByAddress2 {
    /// A fresh enumerator at the same position, via COM `Clone()`.
    ///
    /// The minimal binding's `Clone()` returns the base
    /// `IDiaEnumSymbolsByAddr`, so we cast up to the `2` interface.
    fn clone(&self) -> Self {
        unsafe {
            Self(
                self.0
                    .Clone()
                    .expect("DIA symbols-by-addr-2 COM Clone")
                    .cast::<IDiaEnumSymbolsByAddr2>()
                    .expect("DIA symbols-by-addr-2 Clone cast"),
            )
        }
    }
}

impl SymbolsByAddress2 {
    /// Positions the enumerator by section and offset.
    ///
    /// `promote_block_symbol` controls whether a block symbol is promoted
    /// to its parent function.
    pub fn symbol_by_addr(
        &self,
        promote_block_symbol: bool,
        isect: u32,
        offset: u32,
    ) -> Result<Symbol> {
        unsafe {
            Ok(Symbol(self.0.symbolByAddrEx(
                promote_block_symbol,
                isect,
                offset,
            )?))
        }
    }

    /// Positions the enumerator by relative virtual address.
    pub fn symbol_by_rva(&self, promote_block_symbol: bool, rva: u32) -> Result<Symbol> {
        unsafe { Ok(Symbol(self.0.symbolByRVAEx(promote_block_symbol, rva)?)) }
    }

    /// Positions the enumerator by virtual address.
    pub fn symbol_by_va(&self, promote_block_symbol: bool, va: u64) -> Result<Symbol> {
        unsafe { Ok(Symbol(self.0.symbolByVAEx(promote_block_symbol, va)?)) }
    }

    /// Advances the enumerator by one symbol, returning a `Result`.
    ///
    /// `None` is a clean end of enumeration; `Err` is a real COM failure.
    pub fn next_ex(&mut self, promote_block_symbol: bool) -> Result<Option<Symbol>> {
        unsafe {
            let mut slot: Option<IDiaSymbol> = None;
            let mut fetched: u32 = 0;
            let hr = self
                .0
                .NextEx(promote_block_symbol, 1, &mut slot, &mut fetched);
            if fetched == 0 {
                hr.ok()?;
                Ok(None)
            } else {
                hr.ok()?;
                match slot {
                    Some(t) => Ok(Some(Symbol(t))),
                    None => Err(Error::from(HRESULT(0x80004005u32 as i32))),
                }
            }
        }
    }

    /// Moves the enumerator back one symbol, returning a `Result`.
    pub fn prev_ex(&mut self, promote_block_symbol: bool) -> Result<Option<Symbol>> {
        unsafe {
            let mut slot: Option<IDiaSymbol> = None;
            let mut fetched: u32 = 0;
            let hr = self
                .0
                .PrevEx(promote_block_symbol, 1, &mut slot, &mut fetched);
            if fetched == 0 {
                hr.ok()?;
                Ok(None)
            } else {
                hr.ok()?;
                match slot {
                    Some(t) => Ok(Some(Symbol(t))),
                    None => Err(Error::from(HRESULT(0x80004005u32 as i32))),
                }
            }
        }
    }
}

impl Iterator for SymbolsByAddress2 {
    type Item = Symbol;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_ex(false).ok().flatten()
    }
}
