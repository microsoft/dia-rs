//! DIA data-record types: `LineNumber`, `SourceFile`, and their companions.

use crate::bindings::*;
use crate::enums::Symbols;
use crate::symbol::Symbol;
use windows_core::*;

/// A line-number record, relating a source position to an image location.
///
/// Returned by the line-number finders and the
/// [`LineNumbers`](crate::enums::LineNumbers) enumerator.
#[derive(Clone, Debug)]
pub struct LineNumber(pub(crate) IDiaLineNumber);

impl LineNumber {
    /// Retrieves the line number in the source file.
    pub fn line_number(&self) -> Result<u32> {
        unsafe { self.0.lineNumber() }
    }

    /// Retrieves the one-based source line number where the statement or
    /// expression ends.
    pub fn line_number_end(&self) -> Result<u32> {
        unsafe { self.0.lineNumberEnd() }
    }

    /// Retrieves the column number where the expression or statement begins.
    pub fn column_number(&self) -> Result<u32> {
        unsafe { self.0.columnNumber() }
    }

    /// Retrieves the column number where the expression or statement ends.
    pub fn column_number_end(&self) -> Result<u32> {
        unsafe { self.0.columnNumberEnd() }
    }

    /// Retrieves the section part of the memory address where a block begins.
    pub fn address_section(&self) -> Result<u32> {
        unsafe { self.0.addressSection() }
    }

    /// Retrieves the offset part of the memory address where a block begins.
    pub fn address_offset(&self) -> Result<u32> {
        unsafe { self.0.addressOffset() }
    }

    /// Retrieves the relative virtual address (RVA) of the block.
    pub fn rva(&self) -> Result<u32> {
        unsafe { self.0.relativeVirtualAddress() }
    }

    /// Retrieves the virtual address (VA) of the block.
    pub fn va(&self) -> Result<u64> {
        unsafe { self.0.virtualAddress() }
    }

    /// Retrieves the number of bytes in the block.
    pub fn length(&self) -> Result<u32> {
        unsafe { self.0.length() }
    }

    /// Retrieves a unique source file identifier for the source file that
    /// contributed this line.
    pub fn source_file_id(&self) -> Result<u32> {
        unsafe { self.0.sourceFileId() }
    }

    /// Retrieves a flag indicating that this line information describes the
    /// beginning of a statement in the program source.
    pub fn statement(&self) -> Result<bool> {
        unsafe { Ok(self.0.statement()?.as_bool()) }
    }

    /// Retrieves the unique identifier for the compiland that contributed this
    /// line.
    pub fn compiland_id(&self) -> Result<u32> {
        unsafe { self.0.compilandId() }
    }

    /// Retrieves the source file that contributed this line.
    pub fn source_file(&self) -> Result<SourceFile> {
        unsafe { Ok(SourceFile(self.0.sourceFile()?)) }
    }

    /// Retrieves the symbol for the compiland that contributed the bytes of
    /// image text for this line.
    pub fn compiland(&self) -> Result<Symbol> {
        unsafe { Ok(Symbol(self.0.compiland()?)) }
    }
}

/// A source file referenced by the debug data.
///
/// Returned by the source-file finders and the `SourceFiles` enumerator.
#[derive(Clone, Debug)]
pub struct SourceFile(pub(crate) IDiaSourceFile);

impl SourceFile {
    /// Retrieves a simple integer key value that is unique for this image.
    pub fn unique_id(&self) -> Result<u32> {
        unsafe { self.0.uniqueId() }
    }

    /// Retrieves the source file name.
    pub fn file_name(&self) -> Result<String> {
        unsafe { Ok(self.0.fileName()?.display().to_string()) }
    }

    /// Retrieves the checksum type.
    pub fn checksum_type(&self) -> Result<u32> {
        unsafe { self.0.checksumType() }
    }

    /// Retrieves an enumerator of the compilands with line numbers referencing
    /// this file.
    pub fn compilands(&self) -> Result<Symbols> {
        unsafe { Ok(Symbols(self.0.compilands()?)) }
    }

    /// Retrieves the checksum data for the source file.
    pub fn checksum(&self) -> Result<Vec<u8>> {
        unsafe {
            let mut size: u32 = 0;
            self.0
                .get_checksum(0, &mut size, core::ptr::null_mut())
                .ok()?;
            let mut buf = vec![0u8; size as usize];
            let mut fetched: u32 = 0;
            self.0
                .get_checksum(size, &mut fetched, buf.as_mut_ptr())
                .ok()?;
            buf.truncate(fetched as usize);
            Ok(buf)
        }
    }
}

use crate::constants::StackFrameType;
/// A segment of memory in the image.
#[derive(Clone, Debug)]
pub struct Segment(pub(crate) IDiaSegment);

impl Segment {
    /// Retrieves the segment number.
    pub fn frame(&self) -> Result<u32> {
        unsafe { self.0.frame() }
    }

    /// Retrieves the offset in segments where the section begins.
    pub fn offset(&self) -> Result<u32> {
        unsafe { self.0.offset() }
    }

    /// Retrieves the number of bytes in the segment.
    pub fn length(&self) -> Result<u32> {
        unsafe { self.0.length() }
    }

    /// Retrieves a flag that indicates whether the segment can be read.
    pub fn read(&self) -> Result<bool> {
        unsafe { Ok(self.0.read()?.as_bool()) }
    }

    /// Retrieves a flag that indicates whether the segment can be modified.
    pub fn write(&self) -> Result<bool> {
        unsafe { Ok(self.0.write()?.as_bool()) }
    }

    /// Retrieves a flag that indicates whether the segment is executable.
    pub fn execute(&self) -> Result<bool> {
        unsafe { Ok(self.0.execute()?.as_bool()) }
    }

    /// Retrieves the section number that maps to this segment.
    pub fn address_section(&self) -> Result<u32> {
        unsafe { self.0.addressSection() }
    }

    /// Retrieves the relative virtual address (RVA) of the beginning of the section.
    pub fn rva(&self) -> Result<u32> {
        unsafe { self.0.relativeVirtualAddress() }
    }

    /// Retrieves the virtual address (VA) of the beginning of the section.
    pub fn va(&self) -> Result<u64> {
        unsafe { self.0.virtualAddress() }
    }
}

/// A section contribution.
#[derive(Clone, Debug)]
pub struct SectionContrib(pub(crate) IDiaSectionContrib);

impl SectionContrib {
    /// Retrieves a reference to the compiland symbol that contributed this section.
    pub fn compiland(&self) -> Result<Symbol> {
        unsafe { Ok(Symbol(self.0.compiland()?)) }
    }

    /// Retrieves the section part of the contribution's address.
    pub fn address_section(&self) -> Result<u32> {
        unsafe { self.0.addressSection() }
    }

    /// Retrieves the offset part of the contribution's address.
    pub fn address_offset(&self) -> Result<u32> {
        unsafe { self.0.addressOffset() }
    }

    /// Retrieves the image relative virtual address (RVA) of the contribution.
    pub fn rva(&self) -> Result<u32> {
        unsafe { self.0.relativeVirtualAddress() }
    }

    /// Retrieves the virtual address (VA) of the contribution.
    pub fn va(&self) -> Result<u64> {
        unsafe { self.0.virtualAddress() }
    }

    /// Retrieves the number of bytes in a section.
    pub fn length(&self) -> Result<u32> {
        unsafe { self.0.length() }
    }

    /// Retrieves a flag that indicates whether the section cannot be paged out of memory.
    pub fn not_paged(&self) -> Result<bool> {
        unsafe { Ok(self.0.notPaged()?.as_bool()) }
    }

    /// Retrieves a flag that indicates whether the section contains executable code.
    pub fn code(&self) -> Result<bool> {
        unsafe { Ok(self.0.code()?.as_bool()) }
    }

    /// Retrieves a flag that indicates whether the section contains initialized data.
    pub fn initialized_data(&self) -> Result<bool> {
        unsafe { Ok(self.0.initializedData()?.as_bool()) }
    }

    /// Retrieves a flag that indicates whether the section contains uninitialized data.
    pub fn uninitialized_data(&self) -> Result<bool> {
        unsafe { Ok(self.0.uninitializedData()?.as_bool()) }
    }

    /// Retrieves a flag that indicates whether the section is removed before it is made part of the in-memory image.
    pub fn remove(&self) -> Result<bool> {
        unsafe { Ok(self.0.remove()?.as_bool()) }
    }

    /// Retrieves a flag that indicates whether the section is a COMDAT record.
    pub fn comdat(&self) -> Result<bool> {
        unsafe { Ok(self.0.comdat()?.as_bool()) }
    }

    /// Retrieves a flag that indicates whether the section can be discarded.
    pub fn discardable(&self) -> Result<bool> {
        unsafe { Ok(self.0.discardable()?.as_bool()) }
    }

    /// Retrieves a flag that indicates whether the section cannot be cached.
    pub fn not_cached(&self) -> Result<bool> {
        unsafe { Ok(self.0.notCached()?.as_bool()) }
    }

    /// Retrieves a flag that indicates whether the section can be shared in memory.
    pub fn share(&self) -> Result<bool> {
        unsafe { Ok(self.0.share()?.as_bool()) }
    }

    /// Retrieves a flag that indicates whether the section is executable as code.
    pub fn execute(&self) -> Result<bool> {
        unsafe { Ok(self.0.execute()?.as_bool()) }
    }

    /// Retrieves a flag that indicates whether the section can be read.
    pub fn read(&self) -> Result<bool> {
        unsafe { Ok(self.0.read()?.as_bool()) }
    }

    /// Retrieves a flag that indicates whether the section can be written.
    pub fn write(&self) -> Result<bool> {
        unsafe { Ok(self.0.write()?.as_bool()) }
    }

    /// Retrieves the cyclic redundancy check (CRC) of the data in the section.
    pub fn data_crc(&self) -> Result<u32> {
        unsafe { self.0.dataCrc() }
    }

    /// Retrieves the CRC of the relocation information for the section.
    pub fn relocations_crc(&self) -> Result<u32> {
        unsafe { self.0.relocationsCrc() }
    }

    /// Retrieves the compiland identifier for the section.
    pub fn compiland_id(&self) -> Result<u32> {
        unsafe { self.0.compilandId() }
    }

    /// Retrieves a flag that indicates whether the section contains 16-bit code.
    pub fn code_16bit(&self) -> Result<bool> {
        unsafe { Ok(self.0.code16bit()?.as_bool()) }
    }
}

/// Frame-data for a function.
#[derive(Clone, Debug)]
pub struct FrameData(pub(crate) IDiaFrameData);

impl FrameData {
    /// Retrieves the section part of the code address for the frame.
    pub fn address_section(&self) -> Result<u32> {
        unsafe { self.0.addressSection() }
    }

    /// Retrieves the offset part of the code address for the frame.
    pub fn address_offset(&self) -> Result<u32> {
        unsafe { self.0.addressOffset() }
    }

    /// Retrieves the image relative virtual address (RVA) of the code for the frame.
    pub fn rva(&self) -> Result<u32> {
        unsafe { self.0.relativeVirtualAddress() }
    }

    /// Retrieves the virtual address (VA) of the code for the frame.
    pub fn va(&self) -> Result<u64> {
        unsafe { self.0.virtualAddress() }
    }

    /// Retrieves the length, in bytes, of the block of code described by the frame.
    pub fn length_block(&self) -> Result<u32> {
        unsafe { self.0.lengthBlock() }
    }

    /// Retrieves the number of bytes of local variables pushed on the stack.
    pub fn length_locals(&self) -> Result<u32> {
        unsafe { self.0.lengthLocals() }
    }

    /// Retrieves the number of bytes of parameters pushed on the stack.
    pub fn length_params(&self) -> Result<u32> {
        unsafe { self.0.lengthParams() }
    }

    /// Retrieves the maximum number of bytes pushed on the stack in the frame.
    pub fn max_stack(&self) -> Result<u32> {
        unsafe { self.0.maxStack() }
    }

    /// Retrieves the number of bytes of prologue code in the block.
    pub fn length_prolog(&self) -> Result<u32> {
        unsafe { self.0.lengthProlog() }
    }

    /// Retrieves the number of bytes of saved registers pushed on the stack.
    pub fn length_saved_registers(&self) -> Result<u32> {
        unsafe { self.0.lengthSavedRegisters() }
    }

    /// Retrieves the program string that is used to compute the register set before the call to the current function.
    pub fn program(&self) -> Result<String> {
        unsafe { Ok(self.0.program()?.display().to_string()) }
    }

    /// Retrieves a flag that indicates that system exception handling is in effect.
    pub fn system_exception_handling(&self) -> Result<bool> {
        unsafe { Ok(self.0.systemExceptionHandling()?.as_bool()) }
    }

    /// Retrieves a flag that indicates that exception handling is in effect for the frame.
    pub fn cplusplus_exception_handling(&self) -> Result<bool> {
        unsafe { Ok(self.0.cplusplusExceptionHandling()?.as_bool()) }
    }

    /// Retrieves a flag that indicates that the block contains the entry point of a function.
    pub fn function_start(&self) -> Result<bool> {
        unsafe { Ok(self.0.functionStart()?.as_bool()) }
    }

    /// Retrieves a flag that indicates that the base pointer is allocated for code in this address range. This method is deprecated.
    pub fn allocates_base_pointer(&self) -> Result<bool> {
        unsafe { Ok(self.0.allocatesBasePointer()?.as_bool()) }
    }

    /// The kind of stack frame data, describing how the frame was laid out.
    pub fn frame_data_type(&self) -> Result<StackFrameType> {
        unsafe { Ok(StackFrameType::from_abi(self.0.r#type()? as i32)) }
    }

    /// Retrieves frame data interface for enclosing function.
    pub fn function_parent(&self) -> Result<FrameData> {
        unsafe { Ok(FrameData(self.0.functionParent()?)) }
    }
}

/// An injected source.
#[derive(Clone, Debug)]
pub struct InjectedSource(pub(crate) IDiaInjectedSource);

impl InjectedSource {
    /// Retrieves a cyclic redundancy check (CRC) calculated from the bytes of the source code.
    pub fn crc(&self) -> Result<u32> {
        unsafe { self.0.crc() }
    }

    /// Retrieves the number of bytes of code.
    pub fn length(&self) -> Result<u64> {
        unsafe { self.0.length() }
    }

    /// Retrieves the file name for the source.
    pub fn filename(&self) -> Result<String> {
        unsafe { Ok(self.0.filename()?.display().to_string()) }
    }

    /// Retrieves the object file name to which the source was compiled.
    pub fn object_filename(&self) -> Result<String> {
        unsafe { Ok(self.0.objectFilename()?.display().to_string()) }
    }

    /// Retrieves the name given to non-file source code; that is, code that was injected.
    pub fn virtual_filename(&self) -> Result<String> {
        unsafe { Ok(self.0.virtualFilename()?.display().to_string()) }
    }

    /// Retrieves the indicator of the source compression used.
    pub fn source_compression(&self) -> Result<u32> {
        unsafe { self.0.sourceCompression() }
    }

    /// The raw bytes of the injected source.
    pub fn source(&self) -> Result<Vec<u8>> {
        unsafe {
            let mut size: u32 = 0;
            self.0
                .get_source(0, &mut size, core::ptr::null_mut())
                .ok()?;
            let mut buf = vec![0u8; size as usize];
            let mut fetched: u32 = 0;
            self.0
                .get_source(size, &mut fetched, buf.as_mut_ptr())
                .ok()?;
            buf.truncate(fetched as usize);
            Ok(buf)
        }
    }
}

/// An input assembly file.
#[derive(Clone, Debug)]
pub struct InputAssemblyFile(pub(crate) IDiaInputAssemblyFile);

impl InputAssemblyFile {
    /// Retrieves a unique identifier for the file.
    pub fn unique_id(&self) -> Result<u32> {
        unsafe { self.0.uniqueId() }
    }

    /// Retrieves the file index.
    pub fn index(&self) -> Result<u32> {
        unsafe { self.0.index() }
    }

    /// Retrieves the time stamp.
    pub fn timestamp(&self) -> Result<u32> {
        unsafe { self.0.timestamp() }
    }

    /// Retrieves a flag that indicates whether the PDB was available at the creation of the .NET Native binary.
    pub fn pdb_available_at_i_l_merge(&self) -> Result<bool> {
        unsafe { Ok(self.0.pdbAvailableAtILMerge()?.as_bool()) }
    }

    /// Retrieves the original assembly file name.
    pub fn file_name(&self) -> Result<String> {
        unsafe { Ok(self.0.fileName()?.display().to_string()) }
    }

    /// The raw bytes of the input assembly version information.
    pub fn version(&self) -> Result<Vec<u8>> {
        unsafe {
            let mut size: u32 = 0;
            self.0
                .get_version(0, &mut size, core::ptr::null_mut())
                .ok()?;
            let mut buf = vec![0u8; size as usize];
            let mut fetched: u32 = 0;
            self.0
                .get_version(size, &mut fetched, buf.as_mut_ptr())
                .ok()?;
            buf.truncate(fetched as usize);
            Ok(buf)
        }
    }
}

/// The address map for an image.
#[derive(Clone, Debug)]
pub struct AddressMap(pub(crate) IDiaAddressMap);

impl AddressMap {
    /// Indicates whether an address map has been established for a particular session.
    pub fn address_map_enabled(&self) -> Result<bool> {
        unsafe { Ok(self.0.addressMapEnabled()?.as_bool()) }
    }

    /// Specifies whether the address map should be used to translate symbol addresses.
    pub fn set_address_map_enabled(&self, newval: bool) -> Result<()> {
        unsafe { self.0.SetaddressMapEnabled(newval).ok() }
    }

    /// Indicates whether the calculation and use of relative virtual addresses is enabled.
    pub fn relative_virtual_address_enabled(&self) -> Result<bool> {
        unsafe { Ok(self.0.relativeVirtualAddressEnabled()?.as_bool()) }
    }

    /// Allows the client to enable or disable the calculation of relative virtual addresses.
    pub fn set_relative_virtual_address_enabled(&self, newval: bool) -> Result<()> {
        unsafe { self.0.SetrelativeVirtualAddressEnabled(newval).ok() }
    }

    /// Retrieves the current image alignment.
    pub fn image_align(&self) -> Result<u32> {
        unsafe { self.0.imageAlign() }
    }

    /// Sets the image alignment.
    pub fn set_image_align(&self, newval: u32) -> Result<()> {
        unsafe { self.0.SetimageAlign(newval).ok() }
    }

    /// Sets image headers to enable the translation of relative virtual addresses.
    pub fn set_image_headers(&self, data: &[u8], original_headers: bool) -> Result<()> {
        unsafe {
            self.0
                .set_imageHeaders(data.len() as u32, data.as_ptr(), original_headers)
                .ok()
        }
    }

    /// Provides an address map to support image layout translations.
    pub fn set_address_map(&self, data: &[AddressMapEntry], image_to_symbols: bool) -> Result<()> {
        unsafe {
            let abi: Vec<DiaAddressMapEntry> = data.iter().map(|e| e.abi()).collect();
            self.0
                .set_addressMap(abi.len() as u32, abi.as_ptr(), image_to_symbols)
                .ok()
        }
    }
}

/// Image-data for a module.
#[derive(Clone, Debug)]
pub struct ImageData(pub(crate) IDiaImageData);

impl ImageData {
    /// Retrieves the location in virtual memory of the module relative to the application.
    pub fn rva(&self) -> Result<u32> {
        unsafe { self.0.relativeVirtualAddress() }
    }

    /// Retrieves the location in virtual memory of the image.
    pub fn va(&self) -> Result<u64> {
        unsafe { self.0.virtualAddress() }
    }

    /// Retrieves the memory location where the image should be based.
    pub fn image_base(&self) -> Result<u64> {
        unsafe { self.0.imageBase() }
    }
}

/// A 16-byte tag value read from an enum.
///
/// `value` is the raw tag data, left-aligned; `value_size_bytes`
/// is the number of significant bytes (at most 16).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TagValue {
    /// The raw tag bytes, left-aligned in a 16-byte buffer.
    pub value: [u8; 16],
    /// Number of significant bytes in [`value`](Self::value).
    pub value_size_bytes: u8,
}

/// An image-to-symbols address-map entry.
///
/// Maps an image RVA to the corresponding symbol-table RVA.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AddressMapEntry {
    /// The relative virtual address in the image.
    pub rva: u32,
    /// The relative virtual address in the symbol table.
    pub rva_to: u32,
}

impl TagValue {
    #[allow(dead_code)]
    pub(crate) fn abi(&self) -> DiaTagValue {
        DiaTagValue {
            value: self.value,
            valueSizeBytes: self.value_size_bytes,
        }
    }

    pub(crate) fn from_abi(v: DiaTagValue) -> Self {
        Self {
            value: v.value,
            value_size_bytes: v.valueSizeBytes,
        }
    }
}

impl AddressMapEntry {
    #[allow(dead_code)]
    pub(crate) fn abi(&self) -> DiaAddressMapEntry {
        DiaAddressMapEntry {
            rva: self.rva,
            rvaTo: self.rva_to,
        }
    }
}
