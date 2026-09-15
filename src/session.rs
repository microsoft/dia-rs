//! The DIA session: the entry point for querying symbols.

use crate::bindings::*;
use crate::constants::{NameSearchOptions, SymTag};
use crate::enums::{
    DebugStreams, InjectedSources, InputAssemblyFiles, LineNumbers, SourceFiles, SourceLinks,
    Symbols, SymbolsByAddress, Tables,
};
use crate::records::{InputAssemblyFile, SourceFile};
use crate::symbol::Symbol;
use windows_core::*;

/// A symbol session, opened against loaded debug data.
///
/// A `Session` wraps `IDiaSession` and also exposes the `IDiaSessionEx`
/// extension methods.
#[derive(Clone, Debug)]
pub struct Session(pub(crate) IDiaSession);

impl Session {
    fn ex(&self) -> Result<IDiaSessionEx> {
        self.0.cast()
    }

    /// Retrieves a reference to the global scope.
    pub fn global_scope(&self) -> Result<Symbol> {
        unsafe { Ok(Symbol(self.0.globalScope()?)) }
    }

    /// Retrieves the address a VA maps to in the image.
    pub fn address_for_va(&self, va: u64) -> Result<(u32, u32)> {
        unsafe {
            let mut isect: u32 = 0;
            let mut offset: u32 = 0;
            self.0.addressForVA(va, &mut isect, &mut offset).ok()?;
            Ok((isect, offset))
        }
    }

    /// Retrieves the address an RVA maps to in the image.
    pub fn address_for_rva(&self, rva: u32) -> Result<(u32, u32)> {
        unsafe {
            let mut isect: u32 = 0;
            let mut offset: u32 = 0;
            self.0.addressForRVA(rva, &mut isect, &mut offset).ok()?;
            Ok((isect, offset))
        }
    }

    /// Retrieves an enumerator for all tables contained in the symbol store.
    pub fn tables(&self) -> Result<Tables> {
        unsafe { Ok(Tables(self.0.getEnumTables()?)) }
    }

    /// Retrieves an enumerator for all named symbols at static locations.
    pub fn symbols_by_addr(&self) -> Result<SymbolsByAddress> {
        unsafe { Ok(SymbolsByAddress(self.0.getSymbolsByAddr()?)) }
    }

    /// Retrieves a symbol by its unique ID.
    pub fn symbol_by_id(&self, unique_id: u32) -> Result<Symbol> {
        unsafe { Ok(Symbol(self.0.symbolById(unique_id)?)) }
    }

    /// Determines whether two symbols are equivalent.
    ///
    /// The verdict is carried in the HRESULT: `S_OK` = equivalent,
    /// `S_FALSE` = not equivalent (the SDK documentation lists `E_FAIL` for
    /// the negative case, but the DIA SDK in practice returns `S_FALSE`; the
    /// pinned test `syms_are_equiv_polarity` exercises both branches). Any
    /// other code — including `E_INVALIDARG` for a null argument — is a real
    /// error.
    ///
    /// Equivalence is session-scoped, so this lives on [`Session`] rather than
    /// as an `Eq` impl on [`Symbol`].
    pub fn syms_are_equiv(&self, a: &Symbol, b: &Symbol) -> Result<bool> {
        let hr = unsafe { self.0.symsAreEquiv(&a.0, &b.0) };
        match hr.0 {
            0x0000_0000 => Ok(true),  // S_OK: equivalent
            0x0000_0001 => Ok(false), // S_FALSE: not equivalent
            code => Err(HRESULT(code).into()),
        }
    }

    /// Retrieves all children of a specified parent identifier that match the
    /// name and symbol type.
    pub fn find_children(
        &self,
        parent: &Symbol,
        sym_tag: SymTag,
        name: &str,
        compare_flags: NameSearchOptions,
    ) -> Result<Symbols> {
        unsafe {
            Ok(Symbols(self.0.findChildren(
                &parent.0,
                sym_tag.abi() as i32,
                &HSTRING::from(name),
                compare_flags.abi(),
            )?))
        }
    }

    /// Retrieves a source file by compiland and name.
    pub fn find_file(
        &self,
        compiland: &Symbol,
        name: &str,
        compare_flags: NameSearchOptions,
    ) -> Result<SourceFiles> {
        unsafe {
            Ok(SourceFiles(self.0.findFile(
                &compiland.0,
                &HSTRING::from(name),
                compare_flags.abi(),
            )?))
        }
    }

    /// Retrieves a source file by source file identifier.
    pub fn find_file_by_id(&self, unique_id: u32) -> Result<SourceFile> {
        unsafe { Ok(SourceFile(self.0.findFileById(unique_id)?)) }
    }

    /// Retrieves line numbers within a specified compiland and source file.
    pub fn find_lines(&self, compiland: &Symbol, file: &SourceFile) -> Result<LineNumbers> {
        unsafe { Ok(LineNumbers(self.0.findLines(&compiland.0, &file.0)?)) }
    }

    /// Retrieves the line numbers for a given line number in the source file.
    pub fn find_lines_by_linenum(
        &self,
        compiland: &Symbol,
        file: &SourceFile,
        linenum: u32,
        column: u32,
    ) -> Result<LineNumbers> {
        unsafe {
            Ok(LineNumbers(self.0.findLinesByLinenum(
                &compiland.0,
                &file.0,
                linenum,
                column,
            )?))
        }
    }

    /// Retrieves the line numbers for a given address range.
    pub fn find_lines_by_addr(&self, seg: u32, offset: u32, length: u32) -> Result<LineNumbers> {
        unsafe { Ok(LineNumbers(self.0.findLinesByAddr(seg, offset, length)?)) }
    }

    /// Retrieves the line numbers for a given RVA range.
    pub fn find_lines_by_rva(&self, rva: u32, length: u32) -> Result<LineNumbers> {
        unsafe { Ok(LineNumbers(self.0.findLinesByRVA(rva, length)?)) }
    }

    /// Retrieves the line numbers for a given VA range.
    pub fn find_lines_by_va(&self, va: u64, length: u32) -> Result<LineNumbers> {
        unsafe { Ok(LineNumbers(self.0.findLinesByVA(va, length)?)) }
    }

    /// Sets the base address used to map relative virtual addresses.
    pub fn set_load_address(&self, va: u64) -> Result<()> {
        unsafe { self.0.SetloadAddress(va).ok() }
    }

    /// Retrieves the base address used to map relative virtual addresses.
    pub fn load_address(&self) -> Result<u64> {
        unsafe { self.0.loadAddress() }
    }

    /// Retrieves a flag indicating whether the PDB was linked with
    /// incremental / fast-link.
    pub fn is_fast_link_pdb(&self) -> Result<bool> {
        unsafe { Ok(self.ex()?.isFastLinkPDB()?.as_bool()) }
    }

    /// Retrieves a flag indicating whether the PDB is a portable PDB.
    pub fn is_portable_pdb(&self) -> Result<bool> {
        unsafe { Ok(self.ex()?.isPortablePDB()?.as_bool()) }
    }

    /// Retrieves an enumerator of the source-link records for the
    /// specified symbol.
    pub fn get_source_link_info(&self, parent: &Symbol) -> Result<SourceLinks> {
        unsafe { Ok(SourceLinks(self.ex()?.getSourceLinkInfo(&parent.0)?)) }
    }

    /// Retrieves a specified symbol type that contains, or is closest to,
    /// a specified address.
    pub fn find_symbol_by_addr(
        &self,
        section: u32,
        offset: u32,
        sym_tag: SymTag,
    ) -> Result<Symbol> {
        unsafe {
            Ok(Symbol(self.0.findSymbolByAddr(
                section,
                offset,
                sym_tag.abi() as i32,
            )?))
        }
    }

    /// Retrieves a specified symbol type that contains, or is closest to,
    /// a specified relative virtual address (RVA).
    pub fn find_symbol_by_rva(&self, rva: u32, sym_tag: SymTag) -> Result<Symbol> {
        unsafe { Ok(Symbol(self.0.findSymbolByRVA(rva, sym_tag.abi() as i32)?)) }
    }

    /// Retrieves a specified symbol type that contains, or is closest to,
    /// a specified virtual address (VA).
    pub fn find_symbol_by_va(&self, va: u64, sym_tag: SymTag) -> Result<Symbol> {
        unsafe { Ok(Symbol(self.0.findSymbolByVA(va, sym_tag.abi() as i32)?)) }
    }

    /// Retrieves the symbol that contains a specified metadata token.
    pub fn find_symbol_by_token(&self, token: u32, sym_tag: SymTag) -> Result<Symbol> {
        unsafe {
            Ok(Symbol(
                self.0.findSymbolByToken(token, sym_tag.abi() as i32)?,
            ))
        }
    }

    /// Retrieves a specified symbol type that contains, or is closest to,
    /// a specified relative virtual address, plus the signed offset of that
    /// address from the start of the symbol.
    pub fn find_symbol_by_rva_ex(&self, rva: u32, sym_tag: SymTag) -> Result<(Symbol, i32)> {
        unsafe {
            let mut symbol: Option<IDiaSymbol> = None;
            let mut displacement: i32 = 0;
            self.0
                .findSymbolByRVAEx(rva, sym_tag.abi() as i32, &mut symbol, &mut displacement)
                .ok()?;
            Ok((
                Symbol(symbol.ok_or_else(|| Error::from(HRESULT(0x80004003u32 as i32)))?),
                displacement,
            ))
        }
    }

    /// Retrieves a specified symbol type that contains, or is closest to,
    /// a specified virtual address, plus the signed offset of that address
    /// from the start of the symbol.
    pub fn find_symbol_by_va_ex(&self, va: u64, sym_tag: SymTag) -> Result<(Symbol, i32)> {
        unsafe {
            let mut symbol: Option<IDiaSymbol> = None;
            let mut displacement: i32 = 0;
            self.0
                .findSymbolByVAEx(va, sym_tag.abi() as i32, &mut symbol, &mut displacement)
                .ok()?;
            Ok((
                Symbol(symbol.ok_or_else(|| Error::from(HRESULT(0x80004003u32 as i32)))?),
                displacement,
            ))
        }
    }

    /// Retrieves all children of a specified parent identifier that match
    /// the name and symbol type, including optimized locals.
    pub fn find_children_ex(
        &self,
        parent: &Symbol,
        sym_tag: SymTag,
        name: &str,
        compare_flags: NameSearchOptions,
    ) -> Result<Symbols> {
        unsafe {
            Ok(Symbols(self.0.findChildrenEx(
                &parent.0,
                sym_tag.abi() as i32,
                &HSTRING::from(name),
                compare_flags.abi(),
            )?))
        }
    }

    /// Retrieves all children of a specified parent identifier that match
    /// the name and symbol type, including optimized locals, that contain,
    /// or are closest to, a specified address.
    pub fn find_children_ex_by_addr(
        &self,
        parent: &Symbol,
        sym_tag: SymTag,
        name: &str,
        compare_flags: NameSearchOptions,
        section: u32,
        offset: u32,
    ) -> Result<Symbols> {
        unsafe {
            Ok(Symbols(self.0.findChildrenExByAddr(
                &parent.0,
                sym_tag.abi() as i32,
                &HSTRING::from(name),
                compare_flags.abi(),
                section,
                offset,
            )?))
        }
    }

    /// Retrieves all children of a specified parent identifier that match
    /// the name and symbol type, including optimized locals, that contain,
    /// or are closest to, a specified virtual address (VA).
    pub fn find_children_ex_by_va(
        &self,
        parent: &Symbol,
        sym_tag: SymTag,
        name: &str,
        compare_flags: NameSearchOptions,
        va: u64,
    ) -> Result<Symbols> {
        unsafe {
            Ok(Symbols(self.0.findChildrenExByVA(
                &parent.0,
                sym_tag.abi() as i32,
                &HSTRING::from(name),
                compare_flags.abi(),
                va,
            )?))
        }
    }

    /// Retrieves all children of a specified parent identifier that match
    /// the name and symbol type, including optimized locals, that contain,
    /// or are closest to, a specified relative virtual address (RVA).
    pub fn find_children_ex_by_rva(
        &self,
        parent: &Symbol,
        sym_tag: SymTag,
        name: &str,
        compare_flags: NameSearchOptions,
        rva: u32,
    ) -> Result<Symbols> {
        unsafe {
            Ok(Symbols(self.0.findChildrenExByRVA(
                &parent.0,
                sym_tag.abi() as i32,
                &HSTRING::from(name),
                compare_flags.abi(),
                rva,
            )?))
        }
    }

    /// Retrieves an enumeration that allows a client to iterate through
    /// all of the inline frames on a given address.
    pub fn find_inline_frames_by_addr(
        &self,
        parent: &Symbol,
        section: u32,
        offset: u32,
    ) -> Result<Symbols> {
        unsafe {
            Ok(Symbols(
                self.0.findInlineFramesByAddr(&parent.0, section, offset)?,
            ))
        }
    }

    /// Retrieves an enumeration that allows a client to iterate through
    /// all of the inline frames on a specified relative virtual address
    /// (RVA).
    pub fn find_inline_frames_by_rva(&self, parent: &Symbol, rva: u32) -> Result<Symbols> {
        unsafe { Ok(Symbols(self.0.findInlineFramesByRVA(&parent.0, rva)?)) }
    }

    /// Retrieves an enumeration that allows a client to iterate through
    /// all of the inline frames on a specified virtual address (VA).
    pub fn find_inline_frames_by_va(&self, parent: &Symbol, va: u64) -> Result<Symbols> {
        unsafe { Ok(Symbols(self.0.findInlineFramesByVA(&parent.0, va)?)) }
    }

    /// Retrieves an enumeration that allows a client to iterate through
    /// the line number information of all functions that are inlined,
    /// directly or indirectly, by the specified parent symbol.
    pub fn find_inlinee_lines(&self, parent: &Symbol) -> Result<LineNumbers> {
        unsafe { Ok(LineNumbers(self.0.findInlineeLines(&parent.0)?)) }
    }

    /// Retrieves an enumeration that allows a client to iterate through
    /// the line number information of all functions that are inlined,
    /// directly or indirectly, by the specified parent symbol and are
    /// contained within the specified address range.
    pub fn find_inlinee_lines_by_addr(
        &self,
        parent: &Symbol,
        section: u32,
        offset: u32,
        length: u32,
    ) -> Result<LineNumbers> {
        unsafe {
            Ok(LineNumbers(self.0.findInlineeLinesByAddr(
                &parent.0, section, offset, length,
            )?))
        }
    }

    /// Retrieves an enumeration that allows a client to iterate through
    /// the line number information of all functions that are inlined,
    /// directly or indirectly, by the specified parent symbol and are
    /// contained within the specified relative virtual address (RVA).
    pub fn find_inlinee_lines_by_rva(
        &self,
        parent: &Symbol,
        rva: u32,
        length: u32,
    ) -> Result<LineNumbers> {
        unsafe {
            Ok(LineNumbers(
                self.0.findInlineeLinesByRVA(&parent.0, rva, length)?,
            ))
        }
    }

    /// Retrieves an enumeration that allows a client to iterate through
    /// the line number information of all functions that are inlined,
    /// directly or indirectly, by the specified parent symbol and are
    /// contained within the specified virtual address (VA).
    pub fn find_inlinee_lines_by_va(
        &self,
        parent: &Symbol,
        va: u64,
        length: u32,
    ) -> Result<LineNumbers> {
        unsafe {
            Ok(LineNumbers(
                self.0.findInlineeLinesByVA(&parent.0, va, length)?,
            ))
        }
    }

    /// Retrieves an enumeration that allows a client to iterate through
    /// the line number information of all functions that are inlined,
    /// directly or indirectly, in the specified source file and line
    /// number.
    pub fn find_inlinee_lines_by_linenum(
        &self,
        compiland: &Symbol,
        file: &SourceFile,
        linenum: u32,
        column: u32,
    ) -> Result<LineNumbers> {
        unsafe {
            Ok(LineNumbers(self.0.findInlineeLinesByLinenum(
                &compiland.0,
                &file.0,
                linenum,
                column,
            )?))
        }
    }

    /// Retrieves an enumerator of inline frames that match a specified
    /// name.
    pub fn find_inlinees_by_name(&self, name: &str, options: NameSearchOptions) -> Result<Symbols> {
        unsafe {
            Ok(Symbols(
                self.0
                    .findInlineesByName(&HSTRING::from(name), options.abi())?,
            ))
        }
    }

    /// Retrieves an enumerator of the line numbers of the inline frames
    /// that are inlined, directly or indirectly, at the specified source
    /// file and line number within the specified parent symbol.
    pub fn find_accelerator_inlinee_lines_by_linenum(
        &self,
        parent: &Symbol,
        file: &SourceFile,
        linenum: u32,
        column: u32,
    ) -> Result<LineNumbers> {
        unsafe {
            Ok(LineNumbers(self.0.findAcceleratorInlineeLinesByLinenum(
                &parent.0, &file.0, linenum, column,
            )?))
        }
    }

    /// Returns an enumeration of symbols for the variable that the
    /// specified tag value corresponds to in the parent Accelerator stub
    /// function.
    pub fn find_symbols_for_accelerator_pointer_tag(
        &self,
        parent: &Symbol,
        tag_value: u32,
    ) -> Result<Symbols> {
        unsafe {
            Ok(Symbols(self.0.findSymbolsForAcceleratorPointerTag(
                &parent.0, tag_value,
            )?))
        }
    }

    /// Given a corresponding tag value, this method returns an enumeration
    /// of symbols that are contained in a specified parent Accelerator
    /// stub function at a specified relative virtual address.
    pub fn find_symbols_by_rva_for_accelerator_pointer_tag(
        &self,
        parent: &Symbol,
        tag_value: u32,
        rva: u32,
    ) -> Result<Symbols> {
        unsafe {
            Ok(Symbols(self.0.findSymbolsByRVAForAcceleratorPointerTag(
                &parent.0, tag_value, rva,
            )?))
        }
    }

    /// Returns an enumeration of symbols for inline frames corresponding
    /// to the specified inline function name.
    pub fn find_accelerator_inlinees_by_name(
        &self,
        name: &str,
        options: NameSearchOptions,
    ) -> Result<Symbols> {
        unsafe {
            Ok(Symbols(self.0.findAcceleratorInlineesByName(
                &HSTRING::from(name),
                options.abi(),
            )?))
        }
    }

    /// Retrieves an enumeration that allows a client to iterate through
    /// the MSIL offsets within a specified address range.
    pub fn find_i_offsets_by_addr(
        &self,
        section: u32,
        offset: u32,
        length: u32,
    ) -> Result<LineNumbers> {
        unsafe {
            Ok(LineNumbers(
                self.0.findILOffsetsByAddr(section, offset, length)?,
            ))
        }
    }

    /// Retrieves an enumeration that allows a client to iterate through
    /// the MSIL offsets within a specified relative virtual address (RVA)
    /// range.
    pub fn find_i_offsets_by_rva(&self, rva: u32, length: u32) -> Result<LineNumbers> {
        unsafe { Ok(LineNumbers(self.0.findILOffsetsByRVA(rva, length)?)) }
    }

    /// Retrieves an enumeration that allows a client to iterate through
    /// the MSIL offsets within a specified virtual address (VA) range.
    pub fn find_i_offsets_by_va(&self, va: u64, length: u32) -> Result<LineNumbers> {
        unsafe { Ok(LineNumbers(self.0.findILOffsetsByVA(va, length)?)) }
    }

    /// Retrieves a source that has been placed into the symbol store by
    /// attribute providers or other components of the compilation process.
    pub fn find_injected_source(&self, name: &str) -> Result<InjectedSources> {
        unsafe {
            Ok(InjectedSources(
                self.0.findInjectedSource(&HSTRING::from(name))?,
            ))
        }
    }

    /// Retrieves an enumeration that allows a client to iterate through
    /// the .NET Native input assembly files.
    pub fn find_input_assembly_files(&self) -> Result<InputAssemblyFiles> {
        unsafe { Ok(InputAssemblyFiles(self.0.findInputAssemblyFiles()?)) }
    }

    /// Retrieves a .NET Native input assembly file by index.
    pub fn find_input_assembly(&self, index: u32) -> Result<InputAssemblyFile> {
        unsafe { Ok(InputAssemblyFile(self.0.findInputAssembly(index)?)) }
    }

    /// Retrieves a .NET Native input assembly file by unique identifier.
    pub fn find_input_assembly_by_id(&self, unique_id: u32) -> Result<InputAssemblyFile> {
        unsafe { Ok(InputAssemblyFile(self.0.findInputAssemblyById(unique_id)?)) }
    }

    /// Retrieves the .NET Native input assembly file that is the parent of
    /// the specified symbol.
    pub fn find_input_assembly_file(&self, symbol: &Symbol) -> Result<InputAssemblyFile> {
        unsafe { Ok(InputAssemblyFile(self.0.findInputAssemblyFile(&symbol.0)?)) }
    }

    /// Retrieves the size, in bytes, of the .NET Native metadata function
    /// token map.
    pub fn func_md_token_map_size(&self) -> Result<u32> {
        unsafe { self.0.getFuncMDTokenMapSize() }
    }

    /// Retrieves the contents of the .NET Native metadata function token
    /// map.
    pub fn func_md_token_map(&self) -> Result<Vec<u8>> {
        unsafe {
            let size = self.0.getFuncMDTokenMapSize()?;
            let mut buf = vec![0u8; size as usize];
            let mut written: u32 = 0;
            self.0
                .getFuncMDTokenMap(size, &mut written, buf.as_mut_ptr())
                .ok()?;
            buf.truncate(written as usize);
            Ok(buf)
        }
    }

    /// Retrieves the size, in bytes, of the .NET Native metadata type
    /// token map.
    pub fn type_md_token_map_size(&self) -> Result<u32> {
        unsafe { self.0.getTypeMDTokenMapSize() }
    }

    /// Retrieves the contents of the .NET Native metadata type token map.
    pub fn type_md_token_map(&self) -> Result<Vec<u8>> {
        unsafe {
            let size = self.0.getTypeMDTokenMapSize()?;
            let mut buf = vec![0u8; size as usize];
            let mut written: u32 = 0;
            self.0
                .getTypeMDTokenMap(size, &mut written, buf.as_mut_ptr())
                .ok()?;
            buf.truncate(written as usize);
            Ok(buf)
        }
    }

    /// Retrieves the number of fragments of a function that starts at the
    /// specified virtual address (VA).
    ///
    /// `cb_func` is the size of the function, in bytes.
    pub fn function_fragment_count_by_va(&self, va_func: u64, cb_func: u32) -> Result<u32> {
        unsafe { self.0.getNumberOfFunctionFragments_VA(va_func, cb_func) }
    }

    /// Retrieves the number of fragments of a function that starts at the
    /// specified relative virtual address (RVA).
    ///
    /// `cb_func` is the size of the function, in bytes.
    pub fn function_fragment_count_by_rva(&self, rva_func: u32, cb_func: u32) -> Result<u32> {
        unsafe { self.0.getNumberOfFunctionFragments_RVA(rva_func, cb_func) }
    }

    /// Retrieves the addresses and lengths of the fragments of a function
    /// that starts at the specified virtual address (VA).
    ///
    /// `cb_func` is the size of the function, in bytes. Each entry of the
    /// returned vector is a `(start_va, length)` pair.
    pub fn function_fragments_by_va(&self, va_func: u64, cb_func: u32) -> Result<Vec<(u64, u32)>> {
        unsafe {
            let count = self.0.getNumberOfFunctionFragments_VA(va_func, cb_func)?;
            let mut rvas = vec![0u64; count as usize];
            let mut lengths = vec![0u32; count as usize];
            self.0
                .getFunctionFragments_VA(
                    va_func,
                    cb_func,
                    count,
                    rvas.as_mut_ptr(),
                    lengths.as_mut_ptr(),
                )
                .ok()?;
            Ok(rvas.into_iter().zip(lengths).collect())
        }
    }

    /// Retrieves the addresses and lengths of the fragments of a function
    /// that starts at the specified relative virtual address (RVA).
    ///
    /// `cb_func` is the size of the function, in bytes. Each entry of the
    /// returned vector is a `(start_rva, length)` pair.
    pub fn function_fragments_by_rva(
        &self,
        rva_func: u32,
        cb_func: u32,
    ) -> Result<Vec<(u32, u32)>> {
        unsafe {
            let count = self.0.getNumberOfFunctionFragments_RVA(rva_func, cb_func)?;
            let mut rvas = vec![0u32; count as usize];
            let mut lengths = vec![0u32; count as usize];
            self.0
                .getFunctionFragments_RVA(
                    rva_func,
                    cb_func,
                    count,
                    rvas.as_mut_ptr(),
                    lengths.as_mut_ptr(),
                )
                .ok()?;
            Ok(rvas.into_iter().zip(lengths).collect())
        }
    }

    /// Retrieves an enumerator for all exported symbols.
    pub fn exports(&self) -> Result<Symbols> {
        unsafe { Ok(Symbols(self.0.getExports()?)) }
    }

    /// Retrieves an enumerator for all `SymTagHeapAllocationSite`
    /// symbols.
    pub fn heap_allocation_sites(&self) -> Result<Symbols> {
        unsafe { Ok(Symbols(self.0.getHeapAllocationSites()?)) }
    }

    /// Retrieves an enumerated sequence of debug data streams.
    pub fn get_enum_debug_streams(&self) -> Result<DebugStreams> {
        unsafe { Ok(DebugStreams(self.0.getEnumDebugStreams()?)) }
    }
}
