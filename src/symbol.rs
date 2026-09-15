//! The DIA symbol type: `IDiaSymbol` plus the `IDiaSymbol2`-`12` extensions.
//!
//! One public [`Symbol`] type covers the whole symbol surface: the base
//! `IDiaSymbol` getters and finders plus every numbered extension method,
//! which are cast to the numbered interface internally (the `IDiaSymbol2`-`12`
//! types are not part of the public API).

use crate::bindings::*;
use crate::constants::{
    Access, AssociationKind, BasicType, BuiltinKind, CallingConvention, CoroutineKind, DataKind,
    HlMemorySpace, HlRegister, Language, LocationType, MachineType, NameSearchOptions,
    ScalableVectorType, SymTag, ThunkOrdinal, UdtKind,
};
use crate::enums::{LineNumbers, Symbols};
use crate::records::{InputAssemblyFile, TagValue};
use windows_core::*;

/// A debug symbol, as returned by the DIA finders and enumerators.
///
/// A `Symbol` wraps `IDiaSymbol` and also exposes the `IDiaSymbol2`-`12`
/// extension methods, so a single type covers the whole symbol surface.
#[derive(Clone, Debug)]
pub struct Symbol(pub(crate) IDiaSymbol);

impl Symbol {
    /// Retrieves the symbol table tag.
    ///
    /// Unknown discriminants (tags a newer compiler may add) map to
    /// [`SymTag::Null`].
    pub fn sym_tag(&self) -> Result<SymTag> {
        unsafe { Ok(SymTag::from_abi(self.0.symTag()?)) }
    }

    /// Retrieves the name of the symbol.
    pub fn name(&self) -> Result<String> {
        unsafe { Ok(self.0.name()?.display().to_string()) }
    }

    /// Retrieves the undecorated name of the symbol.
    pub fn undecorated_name(&self) -> Result<String> {
        unsafe { Ok(self.0.undecoratedName()?.display().to_string()) }
    }

    /// Retrieves the symbol for the type of this symbol.
    pub fn type_symbol(&self) -> Result<Symbol> {
        unsafe { Ok(Symbol(self.0.r#type()?)) }
    }

    /// Retrieves the machine type of the symbol.
    pub fn machine_type(&self) -> Result<MachineType> {
        unsafe { Ok(MachineType::from_abi(self.0.machineType()? as i32)) }
    }

    /// Retrieves the source language of the symbol.
    pub fn language(&self) -> Result<Language> {
        unsafe { Ok(Language::from_abi(self.0.language()? as i32)) }
    }

    /// Retrieves the calling convention of the symbol.
    pub fn calling_convention(&self) -> Result<CallingConvention> {
        unsafe {
            Ok(CallingConvention::from_abi(
                self.0.callingConvention()? as i32
            ))
        }
    }

    /// Retrieves the address section of the symbol.
    pub fn address_section(&self) -> Result<u32> {
        unsafe { self.0.addressSection() }
    }

    /// Retrieves the address offset of the symbol.
    pub fn address_offset(&self) -> Result<u32> {
        unsafe { self.0.addressOffset() }
    }

    /// Retrieves the relative virtual address (RVA) of the symbol.
    pub fn relative_virtual_address(&self) -> Result<u32> {
        unsafe { self.0.relativeVirtualAddress() }
    }

    /// Retrieves the virtual address of the symbol.
    pub fn virtual_address(&self) -> Result<u64> {
        unsafe { self.0.virtualAddress() }
    }

    /// Retrieves a flag indicating whether the user-defined data type is
    /// unaligned.
    pub fn unaligned_type(&self) -> Result<bool> {
        unsafe { Ok(self.0.unalignedType()?.as_bool()) }
    }

    /// Retrieves a flag indicating whether the user-defined data type is
    /// constant.
    pub fn const_type(&self) -> Result<bool> {
        unsafe { Ok(self.0.constType()?.as_bool()) }
    }

    /// Retrieves a flag indicating whether the user-defined data type is
    /// volatile.
    pub fn volatile_type(&self) -> Result<bool> {
        unsafe { Ok(self.0.volatileType()?.as_bool()) }
    }

    /// Retrieves the GUID for the symbol.
    pub fn guid(&self) -> Result<GUID> {
        unsafe { self.0.guid() }
    }

    /// Retrieves the source line record for the symbol's type definition.
    pub fn src_line_on_type_defn(&self) -> Result<crate::records::LineNumber> {
        unsafe { Ok(crate::records::LineNumber(self.0.getSrcLineOnTypeDefn()?)) }
    }

    /// Retrieves all children of the symbol matching the symbol type, name,
    /// and comparison flags.
    pub fn find_children(
        &self,
        sym_tag: SymTag,
        name: &str,
        compare_flags: NameSearchOptions,
    ) -> Result<Symbols> {
        unsafe {
            Ok(Symbols(self.0.findChildren(
                sym_tag.abi() as i32,
                &HSTRING::from(name),
                compare_flags.abi(),
            )?))
        }
    }

    /// Retrieves the unique symbol identifier.
    pub fn sym_index_id(&self) -> Result<u32> {
        unsafe { self.0.symIndexId() }
    }

    /// Retrieves a reference to the lexical parent of the symbol.
    pub fn lexical_parent(&self) -> Result<Symbol> {
        unsafe { Ok(Symbol(self.0.lexicalParent()?)) }
    }

    /// Retrieves a reference to the class parent of the symbol.
    pub fn class_parent(&self) -> Result<Symbol> {
        unsafe { Ok(Symbol(self.0.classParent()?)) }
    }

    /// Retrieves the variable classification of a data symbol.
    pub fn data_kind(&self) -> Result<DataKind> {
        unsafe { Ok(DataKind::from_abi(self.0.dataKind()? as i32)) }
    }

    /// Retrieves the location type of a data symbol.
    pub fn location_type(&self) -> Result<LocationType> {
        unsafe { Ok(LocationType::from_abi(self.0.locationType()? as i32)) }
    }

    /// Retrieves the register designator of the location.
    pub fn register_id(&self) -> Result<u32> {
        unsafe { self.0.registerId() }
    }

    /// Retrieves the offset of the symbol location.
    pub fn offset(&self) -> Result<i32> {
        unsafe { self.0.offset() }
    }

    /// Retrieves the number of bytes of memory used by the object represented by this symbol.
    pub fn length(&self) -> Result<u64> {
        unsafe { self.0.length() }
    }

    /// Retrieves the slot number of the location.
    pub fn slot(&self) -> Result<u32> {
        unsafe { self.0.slot() }
    }

    /// Retrieves the access modifier of a class member.
    pub fn access(&self) -> Result<Access> {
        unsafe { Ok(Access::from_abi(self.0.access()? as i32)) }
    }

    /// Retrieves the file name of the library or object file from which the object was loaded.
    pub fn library_name(&self) -> Result<String> {
        unsafe { Ok(self.0.libraryName()?.display().to_string()) }
    }

    /// Retrieves the platform type for which the program or compiland was compiled.
    pub fn platform(&self) -> Result<u32> {
        unsafe { self.0.platform() }
    }

    /// Retrieves the flag describing the Edit and Continue features of the compiled program or unit.
    pub fn edit_and_continue_enabled(&self) -> Result<bool> {
        unsafe { Ok(self.0.editAndContinueEnabled()?.as_bool()) }
    }

    /// Retrieves the front-end major version number.
    pub fn front_end_major(&self) -> Result<u32> {
        unsafe { self.0.frontEndMajor() }
    }

    /// Retrieves the front-end minor version number.
    pub fn front_end_minor(&self) -> Result<u32> {
        unsafe { self.0.frontEndMinor() }
    }

    /// Retrieves the front-end build number.
    pub fn front_end_build(&self) -> Result<u32> {
        unsafe { self.0.frontEndBuild() }
    }

    /// Retrieves the back-end major version number.
    pub fn back_end_major(&self) -> Result<u32> {
        unsafe { self.0.backEndMajor() }
    }

    /// Retrieves the back-end minor version number.
    pub fn back_end_minor(&self) -> Result<u32> {
        unsafe { self.0.backEndMinor() }
    }

    /// Retrieves the back-end build number.
    pub fn back_end_build(&self) -> Result<u32> {
        unsafe { self.0.backEndBuild() }
    }

    /// Retrieves the file name of the source file.
    pub fn source_file_name(&self) -> Result<String> {
        unsafe { Ok(self.0.sourceFileName()?.display().to_string()) }
    }

    #[doc(hidden)]
    /// Deprecated function.
    pub fn unused(&self) -> Result<String> {
        unsafe { Ok(self.0.unused()?.display().to_string()) }
    }

    /// Retrieves the thunk type of a function.
    pub fn thunk_ordinal(&self) -> Result<ThunkOrdinal> {
        unsafe { Ok(ThunkOrdinal::from_abi(self.0.thunkOrdinal()? as i32)) }
    }

    /// Retrieves the logical this adjustor for the method.
    pub fn this_adjust(&self) -> Result<i32> {
        unsafe { self.0.thisAdjust() }
    }

    /// Retrieves the offset in the virtual function table of a virtual function.
    pub fn virtual_base_offset(&self) -> Result<u32> {
        unsafe { self.0.virtualBaseOffset() }
    }

    /// Retrieves a flag indicating whether the function is virtual.
    pub fn is_virtual(&self) -> Result<bool> {
        unsafe { Ok(self.0.r#virtual()?.as_bool()) }
    }

    /// Retrieves a flag indicating whether the function is the base class virtual function.
    pub fn intro(&self) -> Result<bool> {
        unsafe { Ok(self.0.intro()?.as_bool()) }
    }

    /// Retrieves a flag that indicating whether the function is pure virtual.
    pub fn pure(&self) -> Result<bool> {
        unsafe { Ok(self.0.pure()?.as_bool()) }
    }

    /// Retrieves the type tag of a simple type.
    pub fn base_type(&self) -> Result<BasicType> {
        unsafe { Ok(BasicType::from_abi(self.0.baseType()? as i32)) }
    }

    /// Retrieves the metadata token of a managed function or variable.
    pub fn token(&self) -> Result<u32> {
        unsafe { self.0.token() }
    }

    /// Retrieves the timestamp of the underlying executable file.
    pub fn time_stamp(&self) -> Result<u32> {
        unsafe { self.0.timeStamp() }
    }

    /// Retrieves the name of the file from which the symbols were loaded.
    pub fn symbols_file_name(&self) -> Result<String> {
        unsafe { Ok(self.0.symbolsFileName()?.display().to_string()) }
    }

    /// Retrieves a flag indicating whether a pointer type is a reference.
    pub fn reference(&self) -> Result<bool> {
        unsafe { Ok(self.0.reference()?.as_bool()) }
    }

    /// Retrieves the number of items in a list or array.
    pub fn count(&self) -> Result<u32> {
        unsafe { self.0.count() }
    }

    /// Retrieves the bit position of a location.
    pub fn bit_position(&self) -> Result<u32> {
        unsafe { self.0.bitPosition() }
    }

    /// Retrieves the symbol identifier of the array index type.
    pub fn array_index_type(&self) -> Result<Symbol> {
        unsafe { Ok(Symbol(self.0.arrayIndexType()?)) }
    }

    /// Retrieves a flag indicating whether the user-defined data type is packed.
    pub fn packed(&self) -> Result<bool> {
        unsafe { Ok(self.0.packed()?.as_bool()) }
    }

    /// Retrieves a flag indicating whether the user-defined data type has a constructor.
    pub fn constructor(&self) -> Result<bool> {
        unsafe { Ok(self.0.constructor()?.as_bool()) }
    }

    /// Retrieves a flag indicating whether the user-defined data type has overloaded operators.
    pub fn overloaded_operator(&self) -> Result<bool> {
        unsafe { Ok(self.0.overloadedOperator()?.as_bool()) }
    }

    /// Retrieves a flag indicating whether the user-defined data type is nested.
    pub fn nested(&self) -> Result<bool> {
        unsafe { Ok(self.0.nested()?.as_bool()) }
    }

    /// Retrieves a flag indicating whether the user-defined data type has nested type definitions.
    pub fn has_nested_types(&self) -> Result<bool> {
        unsafe { Ok(self.0.hasNestedTypes()?.as_bool()) }
    }

    /// Retrieves a flag indicating whether the user-defined data type has any assignment operators defined.
    pub fn has_assignment_operator(&self) -> Result<bool> {
        unsafe { Ok(self.0.hasAssignmentOperator()?.as_bool()) }
    }

    /// Retrieves a flag indicating whether the user-defined data type has any cast operators defined.
    pub fn has_cast_operator(&self) -> Result<bool> {
        unsafe { Ok(self.0.hasCastOperator()?.as_bool()) }
    }

    /// Retrieves a flag indicating whether the user-defined data type appears in a nonglobal lexical scope.
    pub fn scoped(&self) -> Result<bool> {
        unsafe { Ok(self.0.scoped()?.as_bool()) }
    }

    /// Retrieves a flag indicating whether the user-defined data type is a virtual base class.
    pub fn virtual_base_class(&self) -> Result<bool> {
        unsafe { Ok(self.0.virtualBaseClass()?.as_bool()) }
    }

    /// Retrieves a flag indicating whether the user-defined data type is an indirect virtual base class.
    pub fn indirect_virtual_base_class(&self) -> Result<bool> {
        unsafe { Ok(self.0.indirectVirtualBaseClass()?.as_bool()) }
    }

    /// Retrieves the offset of the virtual base pointer.
    pub fn virtual_base_pointer_offset(&self) -> Result<i32> {
        unsafe { self.0.virtualBasePointerOffset() }
    }

    /// Retrieves the symbol interface of the type of the virtual table for a user-defined type.
    pub fn virtual_table_shape(&self) -> Result<Symbol> {
        unsafe { Ok(Symbol(self.0.virtualTableShape()?)) }
    }

    /// Retrieves the lexical parent identifier of the symbol.
    pub fn lexical_parent_id(&self) -> Result<u32> {
        unsafe { self.0.lexicalParentId() }
    }

    /// Retrieves the class parent identifier of the symbol.
    pub fn class_parent_id(&self) -> Result<u32> {
        unsafe { self.0.classParentId() }
    }

    /// Retrieves the type identifier of the symbol.
    pub fn type_id(&self) -> Result<u32> {
        unsafe { self.0.typeId() }
    }

    /// Retrieves the array index type identifier of the symbol.
    pub fn array_index_type_id(&self) -> Result<u32> {
        unsafe { self.0.arrayIndexTypeId() }
    }

    /// Retrieves the virtual table shape identifier of the symbol.
    pub fn virtual_table_shape_id(&self) -> Result<u32> {
        unsafe { self.0.virtualTableShapeId() }
    }

    /// Retrieves a flag indicating whether the symbol refers to a code address.
    pub fn code(&self) -> Result<bool> {
        unsafe { Ok(self.0.code()?.as_bool()) }
    }

    /// Retrieves a flag indicating whether the public symbol refers to a function.
    pub fn function(&self) -> Result<bool> {
        unsafe { Ok(self.0.function()?.as_bool()) }
    }

    /// Retrieves a flag that indicating whether the symbol refers to managed code.
    pub fn managed(&self) -> Result<bool> {
        unsafe { Ok(self.0.managed()?.as_bool()) }
    }

    /// Retrieves a flag indicating whether the symbol refers to Microsoft Intermediate Language (MSIL) code.
    pub fn msil(&self) -> Result<bool> {
        unsafe { Ok(self.0.msil()?.as_bool()) }
    }

    /// Retrieves the index to the virtual base displacement table.
    pub fn virtual_base_disp_index(&self) -> Result<u32> {
        unsafe { self.0.virtualBaseDispIndex() }
    }

    /// Retrieves the age value of a program database.
    pub fn age(&self) -> Result<u32> {
        unsafe { self.0.age() }
    }

    /// Retrieves the symbol's signature value.
    pub fn signature(&self) -> Result<u32> {
        unsafe { self.0.signature() }
    }

    /// Retrieves a flag indicating whether the symbol was compiler-generated.
    pub fn compiler_generated(&self) -> Result<bool> {
        unsafe { Ok(self.0.compilerGenerated()?.as_bool()) }
    }

    /// Retrieves a flag indicating whether another symbol references this address.
    pub fn address_taken(&self) -> Result<bool> {
        unsafe { Ok(self.0.addressTaken()?.as_bool()) }
    }

    /// Retrieves the rank of a FORTRAN multidimensional array.
    pub fn rank(&self) -> Result<u32> {
        unsafe { self.0.rank() }
    }

    /// Retrieves the lower bound of a FORTRAN array dimension.
    pub fn lower_bound(&self) -> Result<Symbol> {
        unsafe { Ok(Symbol(self.0.lowerBound()?)) }
    }

    /// Retrieves the upper bound of a FORTRAN array dimension.
    pub fn upper_bound(&self) -> Result<Symbol> {
        unsafe { Ok(Symbol(self.0.upperBound()?)) }
    }

    /// Retrieves the symbol identifier of the lower bound of a FORTRAN array dimension.
    pub fn lower_bound_id(&self) -> Result<u32> {
        unsafe { self.0.lowerBoundId() }
    }

    /// Retrieves the symbol identifier of the upper bound of a FORTRAN array dimension.
    pub fn upper_bound_id(&self) -> Result<u32> {
        unsafe { self.0.upperBoundId() }
    }

    /// Retrieves the data bytes of an OEM symbol.
    pub fn data_bytes(&self) -> Result<Vec<u8>> {
        unsafe {
            let mut count: u32 = 0;
            self.0
                .get_dataBytes(0, &mut count, core::ptr::null_mut())
                .ok()?;
            let mut buf = vec![0u8; count as usize];
            self.0
                .get_dataBytes(count, &mut count, buf.as_mut_ptr())
                .ok()?;
            buf.truncate(count as usize);
            Ok(buf)
        }
    }

    /// Retrieves the children of the symbol. This method is the extended version of IDiaSymbol::findChildren.
    pub fn find_children_ex(
        &self,
        sym_tag: SymTag,
        name: &str,
        compare_flags: NameSearchOptions,
    ) -> Result<Symbols> {
        unsafe {
            Ok(Symbols(self.0.findChildrenEx(
                sym_tag.abi() as i32,
                &HSTRING::from(name),
                compare_flags.abi(),
            )?))
        }
    }

    /// Retrieves the children of the symbol that are valid at a specified address.
    pub fn find_children_ex_by_addr(
        &self,
        sym_tag: SymTag,
        name: &str,
        compare_flags: NameSearchOptions,
        section: u32,
        offset: u32,
    ) -> Result<Symbols> {
        unsafe {
            Ok(Symbols(self.0.findChildrenExByAddr(
                sym_tag.abi() as i32,
                &HSTRING::from(name),
                compare_flags.abi(),
                section,
                offset,
            )?))
        }
    }

    /// Retrieves the children of the symbol that are valid at a specified virtual address.
    pub fn find_children_ex_by_va(
        &self,
        sym_tag: SymTag,
        name: &str,
        compare_flags: NameSearchOptions,
        va: u64,
    ) -> Result<Symbols> {
        unsafe {
            Ok(Symbols(self.0.findChildrenExByVA(
                sym_tag.abi() as i32,
                &HSTRING::from(name),
                compare_flags.abi(),
                va,
            )?))
        }
    }

    /// Retrieves the children of the symbol that are valid at a specified relative virtual address (RVA).
    pub fn find_children_ex_by_rva(
        &self,
        sym_tag: SymTag,
        name: &str,
        compare_flags: NameSearchOptions,
        rva: u32,
    ) -> Result<Symbols> {
        unsafe {
            Ok(Symbols(self.0.findChildrenExByRVA(
                sym_tag.abi() as i32,
                &HSTRING::from(name),
                compare_flags.abi(),
                rva,
            )?))
        }
    }

    /// Retrieves the address section of a thunk target.
    pub fn target_section(&self) -> Result<u32> {
        unsafe { self.0.targetSection() }
    }

    /// Retrieves the offset section of a thunk target.
    pub fn target_offset(&self) -> Result<u32> {
        unsafe { self.0.targetOffset() }
    }

    /// Retrieves the relative virtual address (RVA) of a thunk target.
    pub fn target_relative_virtual_address(&self) -> Result<u32> {
        unsafe { self.0.targetRelativeVirtualAddress() }
    }

    /// Retrieves the virtual address (VA) of a thunk target.
    pub fn target_virtual_address(&self) -> Result<u64> {
        unsafe { self.0.targetVirtualAddress() }
    }

    /// Retrieves the symbol's oemId value.
    pub fn oem_id(&self) -> Result<u32> {
        unsafe { self.0.oemId() }
    }

    /// Retrieves the symbol's oemSymbolId value.
    pub fn oem_symbol_id(&self) -> Result<u32> {
        unsafe { self.0.oemSymbolId() }
    }

    /// Retrieves an array of compiler-specific type values for this symbol.
    pub fn types(&self) -> Result<Vec<Option<Symbol>>> {
        unsafe {
            let mut count: u32 = 0;
            self.0
                .get_types(0, &mut count, core::ptr::null_mut())
                .ok()?;
            let mut buf: Vec<Option<IDiaSymbol>> = vec![None; count as usize];
            self.0.get_types(count, &mut count, buf.as_mut_ptr()).ok()?;
            buf.truncate(count as usize);
            Ok(buf.into_iter().map(|s| s.map(Symbol)).collect())
        }
    }

    /// Retrieves an array of compiler-specific type identifier values for this symbol.
    pub fn type_ids(&self) -> Result<Vec<u32>> {
        unsafe {
            let mut count: u32 = 0;
            self.0
                .get_typeIds(0, &mut count, core::ptr::null_mut())
                .ok()?;
            let mut buf = vec![0u32; count as usize];
            self.0
                .get_typeIds(count, &mut count, buf.as_mut_ptr())
                .ok()?;
            buf.truncate(count as usize);
            Ok(buf)
        }
    }

    /// Retrieves the type of the object pointer for a class method.
    pub fn object_pointer_type(&self) -> Result<Symbol> {
        unsafe { Ok(Symbol(self.0.objectPointerType()?)) }
    }

    /// Retrieves the variety of a user-defined type (UDT).
    pub fn udt_kind(&self) -> Result<UdtKind> {
        unsafe { Ok(UdtKind::from_abi(self.0.udtKind()? as i32)) }
    }

    /// Extension of the get_undecoratedName method that retrieves the undecorated name based on the value of an extension field.
    pub fn undecorated_name_ex(&self, undecorate_options: u32) -> Result<String> {
        unsafe {
            Ok(self
                .0
                .get_undecoratedNameEx(undecorate_options)?
                .display()
                .to_string())
        }
    }

    /// Retrieves a flag indicating whether the function has been declared with the noreturn attribute.
    pub fn no_return(&self) -> Result<bool> {
        unsafe { Ok(self.0.noReturn()?.as_bool()) }
    }

    /// Retrieves a flag indicating whether the function uses a custom calling convention.
    pub fn custom_calling_convention(&self) -> Result<bool> {
        unsafe { Ok(self.0.customCallingConvention()?.as_bool()) }
    }

    /// Retrieves a flag indicating whether the function is marked with the noinline attribute.
    pub fn no_inline(&self) -> Result<bool> {
        unsafe { Ok(self.0.noInline()?.as_bool()) }
    }

    /// Retrieves a flag indicating whether the function or label contains optimized code as well as debug information.
    pub fn optimized_code_debug_info(&self) -> Result<bool> {
        unsafe { Ok(self.0.optimizedCodeDebugInfo()?.as_bool()) }
    }

    /// Retrieves a flag indicating whether the function or label is never reached.
    pub fn not_reached(&self) -> Result<bool> {
        unsafe { Ok(self.0.notReached()?.as_bool()) }
    }

    /// Retrieves a flag indicating whether the function has a return from interrupt instruction.
    pub fn interrupt_return(&self) -> Result<bool> {
        unsafe { Ok(self.0.interruptReturn()?.as_bool()) }
    }

    /// Retrieves a flag indicating whether the function uses a far return.
    pub fn far_return(&self) -> Result<bool> {
        unsafe { Ok(self.0.farReturn()?.as_bool()) }
    }

    /// Retrieves a flag indicating whether a function or thunk layer is static.
    pub fn is_static(&self) -> Result<bool> {
        unsafe { Ok(self.0.isStatic()?.as_bool()) }
    }

    /// Retrieves a flag indicating whether the compiland contains any debugging information.
    pub fn has_debug_info(&self) -> Result<bool> {
        unsafe { Ok(self.0.hasDebugInfo()?.as_bool()) }
    }

    /// Retrieves a flag indicating whether the managed compiland was linked with the linker's LTCG.
    pub fn is_ltcg(&self) -> Result<bool> {
        unsafe { Ok(self.0.isLTCG()?.as_bool()) }
    }

    /// Retrieves a flag indicating whether the elements of a user-defined data type are aligned to a specific boundary.
    pub fn is_data_aligned(&self) -> Result<bool> {
        unsafe { Ok(self.0.isDataAligned()?.as_bool()) }
    }

    /// Retrieves a flag indicating whether the function or compiland has security checks compiled in (via the /GS (Buffer Security Check) compiler switch).
    pub fn has_security_checks(&self) -> Result<bool> {
        unsafe { Ok(self.0.hasSecurityChecks()?.as_bool()) }
    }

    /// Retrieves the name of the compiler used to create the Compiland.
    pub fn compiler_name(&self) -> Result<String> {
        unsafe { Ok(self.0.compilerName()?.display().to_string()) }
    }

    /// Retrieves a flag indicating whether the function contains a call to alloca.
    pub fn has_alloca(&self) -> Result<bool> {
        unsafe { Ok(self.0.hasAlloca()?.as_bool()) }
    }

    /// Retrieves a flag indicating whether the function contains a setjmp command.
    pub fn has_set_jump(&self) -> Result<bool> {
        unsafe { Ok(self.0.hasSetJump()?.as_bool()) }
    }

    /// Retrieves a flag indicating whether the function contains a longjmp command (part of C-style exception handling).
    pub fn has_long_jump(&self) -> Result<bool> {
        unsafe { Ok(self.0.hasLongJump()?.as_bool()) }
    }

    /// Retrieves a flag indicating whether the function has inline assembly.
    pub fn has_inl_asm(&self) -> Result<bool> {
        unsafe { Ok(self.0.hasInlAsm()?.as_bool()) }
    }

    /// Retrieves a flag indicating whether the function has an exception handler.
    pub fn has_e_h(&self) -> Result<bool> {
        unsafe { Ok(self.0.hasEH()?.as_bool()) }
    }

    /// Retrieves a flag indicating whether the function has Win32-style Structured Exception Handling.
    pub fn has_s_e_h(&self) -> Result<bool> {
        unsafe { Ok(self.0.hasSEH()?.as_bool()) }
    }

    /// Retrieves a flag indicating whether the function has an asynchronous exception handler.
    pub fn has_e_ha(&self) -> Result<bool> {
        unsafe { Ok(self.0.hasEHa()?.as_bool()) }
    }

    /// Retrieves a flag indicating whether the function has the naked attribute.
    pub fn is_naked(&self) -> Result<bool> {
        unsafe { Ok(self.0.isNaked()?.as_bool()) }
    }

    /// Retrieves a flag indicating whether the data is part of an aggregate of many symbols.
    pub fn is_aggregated(&self) -> Result<bool> {
        unsafe { Ok(self.0.isAggregated()?.as_bool()) }
    }

    /// Retrieves a flag indicating whether the data has been split into an aggregate of separate symbols.
    pub fn is_splitted(&self) -> Result<bool> {
        unsafe { Ok(self.0.isSplitted()?.as_bool()) }
    }

    /// Retrieves the containing symbol of this symbol.
    pub fn container(&self) -> Result<Symbol> {
        unsafe { Ok(Symbol(self.0.container()?)) }
    }

    /// Retrieves a flag indicating whether the function has been marked with the inline attribute.
    pub fn inl_spec(&self) -> Result<bool> {
        unsafe { Ok(self.0.inlSpec()?.as_bool()) }
    }

    /// Retrieves a flag indicating whether no stack ordering could be done as part of stack buffer checking.
    pub fn no_stack_ordering(&self) -> Result<bool> {
        unsafe { Ok(self.0.noStackOrdering()?.as_bool()) }
    }

    /// Retrieves the type of a virtual base table pointer.
    pub fn virtual_base_table_type(&self) -> Result<Symbol> {
        unsafe { Ok(Symbol(self.0.virtualBaseTableType()?)) }
    }

    /// Retrieves a flag indicating whether the module contains managed code.
    pub fn has_managed_code(&self) -> Result<bool> {
        unsafe { Ok(self.0.hasManagedCode()?.as_bool()) }
    }

    /// Retrieves a flag indicating whether the module was compiled with the /hotpatch (Create Hotpatchable Image) compiler switch.
    pub fn is_hotpatchable(&self) -> Result<bool> {
        unsafe { Ok(self.0.isHotpatchable()?.as_bool()) }
    }

    /// Retrieves a flag indicating whether the module was converted from Common Intermediate Language (CIL) to native code.
    pub fn is_cvt_cil(&self) -> Result<bool> {
        unsafe { Ok(self.0.isCVTCIL()?.as_bool()) }
    }

    /// Retrieves a flag indicating whether the managed compiland is a .netmodule (containing only metadata).
    pub fn is_msil_netmodule(&self) -> Result<bool> {
        unsafe { Ok(self.0.isMSILNetmodule()?.as_bool()) }
    }

    /// Retrieves a flag indicating whether the symbol file contains C types.
    pub fn is_c_types(&self) -> Result<bool> {
        unsafe { Ok(self.0.isCTypes()?.as_bool()) }
    }

    /// Retrieves a flag indicating whether private symbols have been stripped from the symbol file.
    pub fn is_stripped(&self) -> Result<bool> {
        unsafe { Ok(self.0.isStripped()?.as_bool()) }
    }

    /// Retrieves the front-end QFE version number.
    pub fn front_end_qfe(&self) -> Result<u32> {
        unsafe { self.0.frontEndQFE() }
    }

    /// Retrieves the back-end qfe number.
    pub fn back_end_qfe(&self) -> Result<u32> {
        unsafe { self.0.backEndQFE() }
    }

    /// Retrieves a flag indicating whether this function symbol was inlined into another function.
    pub fn was_inlined(&self) -> Result<bool> {
        unsafe { Ok(self.0.wasInlined()?.as_bool()) }
    }

    /// Retrieves a flag indicating whether this function was compiled with strict_gs_check pragma enabled.
    pub fn strict_gs_check(&self) -> Result<bool> {
        unsafe { Ok(self.0.strictGSCheck()?.as_bool()) }
    }

    /// Retrieves a flag indicating whether the function returns a user-defined type by value.
    pub fn is_cxx_return_udt(&self) -> Result<bool> {
        unsafe { Ok(self.0.isCxxReturnUdt()?.as_bool()) }
    }

    /// Retrieves a flag indicating whether this is an instance constructor of a class with virtual base.
    pub fn is_constructor_virtual_base(&self) -> Result<bool> {
        unsafe { Ok(self.0.isConstructorVirtualBase()?.as_bool()) }
    }

    /// Retrieves a flag that specifies whether a pointer type is an rvalue reference.
    pub fn r_value_reference(&self) -> Result<bool> {
        unsafe { Ok(self.0.RValueReference()?.as_bool()) }
    }

    /// Retrieves the original (unmodified) type of this symbol.
    pub fn unmodified_type(&self) -> Result<Symbol> {
        unsafe { Ok(Symbol(self.0.unmodifiedType()?)) }
    }

    /// Retrieves a flag indicating whether this function has a frame pointer.
    pub fn frame_pointer_present(&self) -> Result<bool> {
        unsafe { Ok(self.0.framePointerPresent()?.as_bool()) }
    }

    /// Retrieves a flag that specifies whether the preprocessor directive for a safe buffer is used.
    pub fn is_safe_buffers(&self) -> Result<bool> {
        unsafe { Ok(self.0.isSafeBuffers()?.as_bool()) }
    }

    /// Retrieves a flag that specifies whether a class is an intrinsic type.
    pub fn intrinsic(&self) -> Result<bool> {
        unsafe { Ok(self.0.intrinsic()?.as_bool()) }
    }

    /// Retrieves a flag that specifies whether the class or method is sealed.
    pub fn sealed(&self) -> Result<bool> {
        unsafe { Ok(self.0.sealed()?.as_bool()) }
    }

    /// Retrieves a flag that specifies whether a user-defined type (UDT) contains homogeneous floating-point aggregate (HFA) data of type float.
    pub fn hfa_float(&self) -> Result<bool> {
        unsafe { Ok(self.0.hfaFloat()?.as_bool()) }
    }

    /// Retrieves a flag that specifies whether a user-defined type (UDT) contains homogeneous floating-point aggregate (HFA) data of type double.
    pub fn hfa_double(&self) -> Result<bool> {
        unsafe { Ok(self.0.hfaDouble()?.as_bool()) }
    }

    /// Returns the section part of the starting address range in which the local symbol is valid.
    pub fn live_range_start_address_section(&self) -> Result<u32> {
        unsafe { self.0.liveRangeStartAddressSection() }
    }

    /// Returns the offset part of the starting address range in which the local symbol is valid.
    pub fn live_range_start_address_offset(&self) -> Result<u32> {
        unsafe { self.0.liveRangeStartAddressOffset() }
    }

    /// Returns the start of the address range in which the local symbol is valid.
    pub fn live_range_start_relative_virtual_address(&self) -> Result<u32> {
        unsafe { self.0.liveRangeStartRelativeVirtualAddress() }
    }

    /// Retrieves the number of valid address ranges associated with the local symbol.
    pub fn count_live_ranges(&self) -> Result<u32> {
        unsafe { self.0.countLiveRanges() }
    }

    /// Returns the length of the address range in which the local symbol is valid.
    pub fn live_range_length(&self) -> Result<u64> {
        unsafe { self.0.liveRangeLength() }
    }

    /// Retrieves the field offset of this symbol withinthe outer user-defined type (UDT).
    pub fn offset_in_udt(&self) -> Result<u32> {
        unsafe { self.0.offsetInUdt() }
    }

    /// Retrieves the register designator of the register holding the base pointer to parameters.
    pub fn param_base_pointer_register_id(&self) -> Result<u32> {
        unsafe { self.0.paramBasePointerRegisterId() }
    }

    /// Retrieves the register designator of the register holding base pointer to locals.
    pub fn local_base_pointer_register_id(&self) -> Result<u32> {
        unsafe { self.0.localBasePointerRegisterId() }
    }

    /// Retrieves a flag indicating whether a local symbol's location liveness is dependent upon the control flow of the function.
    pub fn is_location_control_flow_dependent(&self) -> Result<bool> {
        unsafe { Ok(self.0.isLocationControlFlowDependent()?.as_bool()) }
    }

    /// Retrieves the stride of the matrix or strided array.
    pub fn stride(&self) -> Result<u32> {
        unsafe { self.0.stride() }
    }

    /// Retrieves the number of rows in the matrix.
    pub fn number_of_rows(&self) -> Result<u32> {
        unsafe { self.0.numberOfRows() }
    }

    /// Retrieves the number of columns in the matrix.
    pub fn number_of_columns(&self) -> Result<u32> {
        unsafe { self.0.numberOfColumns() }
    }

    /// Specifies whether the matrix is row major.
    pub fn is_matrix_row_major(&self) -> Result<bool> {
        unsafe { Ok(self.0.isMatrixRowMajor()?.as_bool()) }
    }

    /// Retrieves the set of numeric properties for this symbol.
    pub fn numeric_properties(&self) -> Result<Vec<u32>> {
        unsafe {
            let mut count: u32 = 0;
            self.0
                .get_numericProperties(0, &mut count, core::ptr::null_mut())
                .ok()?;
            let mut buf = vec![0u32; count as usize];
            self.0
                .get_numericProperties(count, &mut count, buf.as_mut_ptr())
                .ok()?;
            buf.truncate(count as usize);
            Ok(buf)
        }
    }

    /// Retrieves the set of modifiers for this symbol.
    pub fn modifier_values(&self) -> Result<Vec<u16>> {
        unsafe {
            let mut count: u32 = 0;
            self.0
                .get_modifierValues(0, &mut count, core::ptr::null_mut())
                .ok()?;
            let mut buf = vec![0u16; count as usize];
            self.0
                .get_modifierValues(count, &mut count, buf.as_mut_ptr())
                .ok()?;
            buf.truncate(count as usize);
            Ok(buf)
        }
    }

    /// Specifies whether the variable carries a return value.
    pub fn is_return_value(&self) -> Result<bool> {
        unsafe { Ok(self.0.isReturnValue()?.as_bool()) }
    }

    /// Specifies whether the variable is optimized away.
    pub fn is_optimized_away(&self) -> Result<bool> {
        unsafe { Ok(self.0.isOptimizedAway()?.as_bool()) }
    }

    /// Retrieves a built-in kind of the HLSL type.
    pub fn built_in_kind(&self) -> Result<BuiltinKind> {
        unsafe { Ok(BuiltinKind::from_abi(self.0.builtInKind()? as i32)) }
    }

    /// Retrieves the register type.
    pub fn register_type(&self) -> Result<HlRegister> {
        unsafe { Ok(HlRegister::from_abi(self.0.registerType()? as i32)) }
    }

    /// Retrieves the base data slot.
    pub fn base_data_slot(&self) -> Result<u32> {
        unsafe { self.0.baseDataSlot() }
    }

    /// Retrieves the base data offset.
    pub fn base_data_offset(&self) -> Result<u32> {
        unsafe { self.0.baseDataOffset() }
    }

    /// Retrieves the texture slot.
    pub fn texture_slot(&self) -> Result<u32> {
        unsafe { self.0.textureSlot() }
    }

    /// Retrieves the sampler slot.
    pub fn sampler_slot(&self) -> Result<u32> {
        unsafe { self.0.samplerSlot() }
    }

    /// Retrieves the uav slot.
    pub fn uav_slot(&self) -> Result<u32> {
        unsafe { self.0.uavSlot() }
    }

    /// Retrieves the size of a member of a user-defined type.
    pub fn size_in_udt(&self) -> Result<u32> {
        unsafe { self.0.sizeInUdt() }
    }

    /// Retrieves the memory space kind.
    pub fn memory_space_kind(&self) -> Result<HlMemorySpace> {
        unsafe { Ok(HlMemorySpace::from_abi(self.0.memorySpaceKind()? as i32)) }
    }

    /// Retrieves the ID of the original (unmodified) type.
    pub fn unmodified_type_id(&self) -> Result<u32> {
        unsafe { self.0.unmodifiedTypeId() }
    }

    /// Retrieves the sub type ID.
    pub fn sub_type_id(&self) -> Result<u32> {
        unsafe { self.0.subTypeId() }
    }

    /// Retrieves the sub type.
    pub fn sub_type(&self) -> Result<Symbol> {
        unsafe { Ok(Symbol(self.0.subType()?)) }
    }

    /// Retrieves the number of modifiers that are applied to the original type.
    pub fn number_of_modifiers(&self) -> Result<u32> {
        unsafe { self.0.numberOfModifiers() }
    }

    /// Retrieves the number of register indices.
    pub fn number_of_register_indices(&self) -> Result<u32> {
        unsafe { self.0.numberOfRegisterIndices() }
    }

    /// Specifies whether this symbol represents High Level Shader Language (HLSL) data.
    pub fn is_h_l_s_l_data(&self) -> Result<bool> {
        unsafe { Ok(self.0.isHLSLData()?.as_bool()) }
    }

    /// Specifies whether this symbol is a pointer to a data member.
    pub fn is_pointer_to_data_member(&self) -> Result<bool> {
        unsafe { Ok(self.0.isPointerToDataMember()?.as_bool()) }
    }

    /// Specifies whether this symbol is a pointer to a member function.
    pub fn is_pointer_to_member_function(&self) -> Result<bool> {
        unsafe { Ok(self.0.isPointerToMemberFunction()?.as_bool()) }
    }

    /// Specifies whether the this pointer points to a data member with single inheritance.
    pub fn is_single_inheritance(&self) -> Result<bool> {
        unsafe { Ok(self.0.isSingleInheritance()?.as_bool()) }
    }

    /// Specifies whether the this pointer points to a data member with multiple inheritance.
    pub fn is_multiple_inheritance(&self) -> Result<bool> {
        unsafe { Ok(self.0.isMultipleInheritance()?.as_bool()) }
    }

    /// Specifies whether the this pointer points to a data member with virtual inheritance.
    pub fn is_virtual_inheritance(&self) -> Result<bool> {
        unsafe { Ok(self.0.isVirtualInheritance()?.as_bool()) }
    }

    /// Specifies whether the this pointer is flagged as restricted.
    pub fn restricted_type(&self) -> Result<bool> {
        unsafe { Ok(self.0.restrictedType()?.as_bool()) }
    }

    /// Specifies whether the this pointer is based on a symbol value.
    pub fn is_pointer_based_on_symbol_value(&self) -> Result<bool> {
        unsafe { Ok(self.0.isPointerBasedOnSymbolValue()?.as_bool()) }
    }

    /// Retrieves the symbol from which the pointer is based.
    pub fn base_symbol(&self) -> Result<Symbol> {
        unsafe { Ok(Symbol(self.0.baseSymbol()?)) }
    }

    /// Retrieves the symbol ID from which the pointer is based.
    pub fn base_symbol_id(&self) -> Result<u32> {
        unsafe { self.0.baseSymbolId() }
    }

    /// Retrieves the object file name.
    pub fn object_file_name(&self) -> Result<String> {
        unsafe { Ok(self.0.objectFileName()?.display().to_string()) }
    }

    /// Retrieves a flag that indicates whether the symbol corresponds to a group shared local variable in code compiled for an AMP accelerator.
    pub fn is_accelerator_group_shared_local(&self) -> Result<bool> {
        unsafe { Ok(self.0.isAcceleratorGroupSharedLocal()?.as_bool()) }
    }

    /// Retrieves a flag that indicates whether the symbol corresponds to the definition range symbol for the tag component of a pointer variable in code compiled for an AMP accelerator. The definition range symbol is the location of a variable for a span of addresses.
    pub fn is_accelerator_pointer_tag_live_range(&self) -> Result<bool> {
        unsafe { Ok(self.0.isAcceleratorPointerTagLiveRange()?.as_bool()) }
    }

    /// Indicates whether the symbol corresponds to a top-level function symbol for a shader compiled for an accelerator that corresponds to a parallel_for_each call.
    pub fn is_accelerator_stub_function(&self) -> Result<bool> {
        unsafe { Ok(self.0.isAcceleratorStubFunction()?.as_bool()) }
    }

    /// Returns the number of accelerator pointer tags in an AMP stub function.
    pub fn number_of_accelerator_pointer_tags(&self) -> Result<u32> {
        unsafe { self.0.numberOfAcceleratorPointerTags() }
    }

    /// Specifies whether the module is compiled with the /SDL option.
    pub fn is_sdl(&self) -> Result<bool> {
        unsafe { Ok(self.0.isSdl()?.as_bool()) }
    }

    /// Retrieves a flag indicating whether this is a WinRT pointer type.
    pub fn is_win_r_t_pointer(&self) -> Result<bool> {
        unsafe { Ok(self.0.isWinRTPointer()?.as_bool()) }
    }

    /// Retrieves a flag indicating whether this is a WinRT reference user-defined type (UDT).
    pub fn is_ref_udt(&self) -> Result<bool> {
        unsafe { Ok(self.0.isRefUdt()?.as_bool()) }
    }

    /// Retrieves a flag indicating whether this is a WinRT value user-defined type (UDT).
    pub fn is_value_udt(&self) -> Result<bool> {
        unsafe { Ok(self.0.isValueUdt()?.as_bool()) }
    }

    /// Retrieves a flag indicating whether the type is a WinRT interface user-defined type (UDT).
    pub fn is_interface_udt(&self) -> Result<bool> {
        unsafe { Ok(self.0.isInterfaceUdt()?.as_bool()) }
    }

    /// Retrieves an enumeration that allows a client to iterate through all of the inline frames on a given address.
    pub fn find_inline_frames_by_addr(&self, section: u32, offset: u32) -> Result<Symbols> {
        unsafe { Ok(Symbols(self.0.findInlineFramesByAddr(section, offset)?)) }
    }

    /// Retrieves an enumeration that allows a client to iterate through all of the inline frames on a specified relative virtual address (RVA).
    pub fn find_inline_frames_by_rva(&self, rva: u32) -> Result<Symbols> {
        unsafe { Ok(Symbols(self.0.findInlineFramesByRVA(rva)?)) }
    }

    /// Retrieves an enumeration that allows a client to iterate through all of the inline frames on a specified virtual address (VA).
    pub fn find_inline_frames_by_va(&self, va: u64) -> Result<Symbols> {
        unsafe { Ok(Symbols(self.0.findInlineFramesByVA(va)?)) }
    }

    /// Retrieves an enumeration that allows a client to iterate through the line number information of all functions that are inlined, directly or indirectly, in this symbol.
    pub fn find_inlinee_lines(&self) -> Result<LineNumbers> {
        unsafe { Ok(LineNumbers(self.0.findInlineeLines()?)) }
    }

    /// Retrieves an enumeration that allows a client to iterate through the line number information of all functions that are inlined, directly or indirectly, in this symbol within the specified address range.
    pub fn find_inlinee_lines_by_addr(
        &self,
        section: u32,
        offset: u32,
        length: u32,
    ) -> Result<LineNumbers> {
        unsafe {
            Ok(LineNumbers(
                self.0.findInlineeLinesByAddr(section, offset, length)?,
            ))
        }
    }

    /// Retrieves an enumeration that allows a client to iterate through the line number information of all functions that are inlined, directly or indirectly, in this symbol within the specified relative virtual address (RVA).
    pub fn find_inlinee_lines_by_rva(&self, rva: u32, length: u32) -> Result<LineNumbers> {
        unsafe { Ok(LineNumbers(self.0.findInlineeLinesByRVA(rva, length)?)) }
    }

    /// Retrieves an enumeration that allows a client to iterate through the line number information of all functions that are inlined, directly or indirectly, in this symbol within the specified virtual address (VA).
    pub fn find_inlinee_lines_by_va(&self, va: u64, length: u32) -> Result<LineNumbers> {
        unsafe { Ok(LineNumbers(self.0.findInlineeLinesByVA(va, length)?)) }
    }

    /// Returns the number of accelerator pointer tags in an AMP stub function.
    pub fn find_symbols_for_accelerator_pointer_tag(&self, tag_value: u32) -> Result<Symbols> {
        unsafe {
            Ok(Symbols(
                self.0.findSymbolsForAcceleratorPointerTag(tag_value)?,
            ))
        }
    }

    /// Given a corresponding tag value, this method returns an enumeration of symbols that are contained in this stub function at a specified relative virtual address.
    pub fn find_symbols_by_rva_for_accelerator_pointer_tag(
        &self,
        tag_value: u32,
        rva: u32,
    ) -> Result<Symbols> {
        unsafe {
            Ok(Symbols(self.0.findSymbolsByRVAForAcceleratorPointerTag(
                tag_value, rva,
            )?))
        }
    }

    /// Returns all accelerator pointer tag values that correspond to an AMP accelerator stub function.
    pub fn accelerator_pointer_tags(&self) -> Result<Vec<u32>> {
        unsafe {
            let mut count: u32 = 0;
            self.0
                .get_acceleratorPointerTags(0, &mut count, core::ptr::null_mut())
                .ok()?;
            let mut buf = vec![0u32; count as usize];
            self.0
                .get_acceleratorPointerTags(count, &mut count, buf.as_mut_ptr())
                .ok()?;
            buf.truncate(count as usize);
            Ok(buf)
        }
    }

    /// Retrieves a flag indicating whether PGO was enabled.
    pub fn is_pgo(&self) -> Result<bool> {
        unsafe { Ok(self.0.isPGO()?.as_bool()) }
    }

    /// Retrieves a flag indicating whether this symbols has valid PGO counts.
    pub fn has_valid_p_g_o_counts(&self) -> Result<bool> {
        unsafe { Ok(self.0.hasValidPGOCounts()?.as_bool()) }
    }

    /// Retrieves a flag indicating whether the function is optimized for speed.
    pub fn is_optimized_for_speed(&self) -> Result<bool> {
        unsafe { Ok(self.0.isOptimizedForSpeed()?.as_bool()) }
    }

    /// Retrieves the total invocation count in PGO training.
    pub fn pgo_entry_count(&self) -> Result<u32> {
        unsafe { self.0.PGOEntryCount() }
    }

    /// Retrieves the edge count between a caller/callee and it's parent.
    pub fn pgo_edge_count(&self) -> Result<u32> {
        unsafe { self.0.PGOEdgeCount() }
    }

    /// Retrieves the dynamic instruction count calculated by training.
    pub fn pgo_dynamic_instruction_count(&self) -> Result<u64> {
        unsafe { self.0.PGODynamicInstructionCount() }
    }

    /// Retrieves the static instruction count.
    pub fn static_size(&self) -> Result<u32> {
        unsafe { self.0.staticSize() }
    }

    /// Retrieves the final static size of live function, after inlining.
    pub fn final_live_static_size(&self) -> Result<u32> {
        unsafe { self.0.finalLiveStaticSize() }
    }

    /// Retrieves the phase this function is a member of for PGO multiphased builds.
    pub fn phase_name(&self) -> Result<String> {
        unsafe { Ok(self.0.phaseName()?.display().to_string()) }
    }

    /// Retrieves a flag indicating whether this function contains control flow checks.
    pub fn has_control_flow_check(&self) -> Result<bool> {
        unsafe { Ok(self.0.hasControlFlowCheck()?.as_bool()) }
    }

    /// Retrieves a flag indicating whether this export is CONSTANT.
    pub fn constant_export(&self) -> Result<bool> {
        unsafe { Ok(self.0.constantExport()?.as_bool()) }
    }

    /// Retrieves a flag indicating whether this export is DATA.
    pub fn data_export(&self) -> Result<bool> {
        unsafe { Ok(self.0.dataExport()?.as_bool()) }
    }

    /// Retrieves a flag indicating whether this export is PRIVATE.
    pub fn private_export(&self) -> Result<bool> {
        unsafe { Ok(self.0.privateExport()?.as_bool()) }
    }

    /// Retrieves a flag indicating whether this export is NONAME.
    pub fn no_name_export(&self) -> Result<bool> {
        unsafe { Ok(self.0.noNameExport()?.as_bool()) }
    }

    /// Retrieves a flag indicating whether this export has an explicitly assigned ordinal.
    pub fn export_has_explicitly_assigned_ordinal(&self) -> Result<bool> {
        unsafe { Ok(self.0.exportHasExplicitlyAssignedOrdinal()?.as_bool()) }
    }

    /// Retrieves a flag indicating whether this export is a forwarder.
    pub fn export_is_forwarder(&self) -> Result<bool> {
        unsafe { Ok(self.0.exportIsForwarder()?.as_bool()) }
    }

    /// Retrieves the ordinal of this export.
    pub fn ordinal(&self) -> Result<u32> {
        unsafe { self.0.ordinal() }
    }

    /// Retrieves the frame size.
    pub fn frame_size(&self) -> Result<u32> {
        unsafe { self.0.frameSize() }
    }

    /// Retrieves the section number of the exception handler.
    pub fn exception_handler_address_section(&self) -> Result<u32> {
        unsafe { self.0.exceptionHandlerAddressSection() }
    }

    /// Retrieves the section offset of the exception handler.
    pub fn exception_handler_address_offset(&self) -> Result<u32> {
        unsafe { self.0.exceptionHandlerAddressOffset() }
    }

    /// Retrieves the relative virtual address of the exception handler.
    pub fn exception_handler_relative_virtual_address(&self) -> Result<u32> {
        unsafe { self.0.exceptionHandlerRelativeVirtualAddress() }
    }

    /// Retrieves the virtual address of the exception handler.
    pub fn exception_handler_virtual_address(&self) -> Result<u64> {
        unsafe { self.0.exceptionHandlerVirtualAddress() }
    }

    /// Retrieves the .NET Native input assembly file that is the parent of the symbol.
    pub fn find_input_assembly_file(&self) -> Result<InputAssemblyFile> {
        unsafe { Ok(InputAssemblyFile(self.0.findInputAssemblyFile()?)) }
    }

    /// Retrieves the characteristics of this COFF section.
    pub fn characteristics(&self) -> Result<u32> {
        unsafe { self.0.characteristics() }
    }

    /// Retrieves the COFF group symbol this symbol comes from.
    pub fn coff_group(&self) -> Result<Symbol> {
        unsafe { Ok(Symbol(self.0.coffGroup()?)) }
    }

    /// Retrieves the binding register index.
    pub fn bind_i_d(&self) -> Result<u32> {
        unsafe { self.0.bindID() }
    }

    /// Retrieves the binding space.
    pub fn bind_space(&self) -> Result<u32> {
        unsafe { self.0.bindSpace() }
    }

    /// Retrieves the lower bound in binding space.
    pub fn bind_slot(&self) -> Result<u32> {
        unsafe { self.0.bindSlot() }
    }

    /// Retrieves a flag indicating whether this is an Objective-C class interface/implementation.
    pub fn is_obj_c_class(&self) -> Result<bool> {
        let i2: IDiaSymbol2 = self.0.cast()?;
        unsafe { Ok(i2.isObjCClass()?.as_bool()) }
    }

    /// Retrieves a flag indicating whether this is an Objective-C category.
    pub fn is_obj_c_category(&self) -> Result<bool> {
        let i2: IDiaSymbol2 = self.0.cast()?;
        unsafe { Ok(i2.isObjCCategory()?.as_bool()) }
    }

    /// Retrieves a flag indicating whether this is an Objective-C protocol.
    pub fn is_obj_c_protocol(&self) -> Result<bool> {
        let i2: IDiaSymbol2 = self.0.cast()?;
        unsafe { Ok(i2.isObjCProtocol()?.as_bool()) }
    }

    /// Retrieves the inlinee symbol from a SymTagInlineSite symbol.
    pub fn inlinee(&self) -> Result<Symbol> {
        let i3: IDiaSymbol3 = self.0.cast()?;
        unsafe { Ok(Symbol(i3.inlinee()?)) }
    }

    /// Retrieves the inlinee ID from a SymTagInlineSite symbol.
    pub fn inlinee_id(&self) -> Result<u32> {
        let i3: IDiaSymbol3 = self.0.cast()?;
        unsafe { i3.inlineeId() }
    }

    /// Retrieves a flag indicating whether the function is declared as noexcept.
    pub fn noexcept(&self) -> Result<bool> {
        let i4: IDiaSymbol4 = self.0.cast()?;
        unsafe { Ok(i4.noexcept()?.as_bool()) }
    }

    /// Retrieves a flag that indicates whether this symbol has an absolute address.
    pub fn has_absolute_address(&self) -> Result<bool> {
        let i5: IDiaSymbol5 = self.0.cast()?;
        unsafe { Ok(i5.hasAbsoluteAddress()?.as_bool()) }
    }

    /// Retrieves a flag that indicates whether this function is a static member function.
    pub fn is_static_member_func(&self) -> Result<bool> {
        let i6: IDiaSymbol6 = self.0.cast()?;
        unsafe { Ok(i6.isStaticMemberFunc()?.as_bool()) }
    }

    /// Retrieves a flag that indicates whether this function has signed return address protections.
    pub fn is_sign_ret(&self) -> Result<bool> {
        let i7: IDiaSymbol7 = self.0.cast()?;
        unsafe { Ok(i7.isSignRet()?.as_bool()) }
    }

    /// Retrieves the coroutine function kind.
    pub fn coroutine_kind(&self) -> Result<CoroutineKind> {
        let i8: IDiaSymbol8 = self.0.cast()?;
        unsafe { Ok(CoroutineKind::from_abi(i8.coroutineKind()? as i32)) }
    }

    /// Retrieves the associated symbol kind.
    pub fn associated_symbol_kind(&self) -> Result<AssociationKind> {
        let i8: IDiaSymbol8 = self.0.cast()?;
        unsafe { Ok(AssociationKind::from_abi(i8.associatedSymbolKind()? as i32)) }
    }

    /// Retrieves the section component of the address of the associated symbol.
    pub fn associated_symbol_section(&self) -> Result<u32> {
        let i8: IDiaSymbol8 = self.0.cast()?;
        unsafe { i8.associatedSymbolSection() }
    }

    /// Retrieves the offset component of the address of the associated symbol.
    pub fn associated_symbol_offset(&self) -> Result<u32> {
        let i8: IDiaSymbol8 = self.0.cast()?;
        unsafe { i8.associatedSymbolOffset() }
    }

    /// Retrieves the relative virtual address (RVA) of the associated symbol.
    pub fn associated_symbol_rva(&self) -> Result<u32> {
        let i8: IDiaSymbol8 = self.0.cast()?;
        unsafe { i8.associatedSymbolRva() }
    }

    /// Retrieves the virtual address (VA) of the associated symbol.
    pub fn associated_symbol_addr(&self) -> Result<u64> {
        let i8: IDiaSymbol8 = self.0.cast()?;
        unsafe { i8.associatedSymbolAddr() }
    }

    /// Retrieves the stack frame pad size used for Edit and Continue.
    pub fn frame_pad_size(&self) -> Result<u32> {
        let i9: IDiaSymbol9 = self.0.cast()?;
        unsafe { i9.framePadSize() }
    }

    /// Retrieves the stack frame pad offset used for Edit and Continue.
    pub fn frame_pad_offset(&self) -> Result<u32> {
        let i9: IDiaSymbol9 = self.0.cast()?;
        unsafe { i9.framePadOffset() }
    }

    /// Retrieves a flag that indicates whether the function was compiled with runtime stack checks.
    pub fn is_r_t_cs(&self) -> Result<bool> {
        let i9: IDiaSymbol9 = self.0.cast()?;
        unsafe { Ok(i9.isRTCs()?.as_bool()) }
    }

    /// The raw source-link bytes for this symbol.
    pub fn source_link(&self) -> Result<Vec<u8>> {
        let i10: IDiaSymbol10 = self.0.cast()?;
        unsafe {
            let mut count: u32 = 0;
            i10.get_sourceLink(0, &mut count, core::ptr::null_mut())
                .ok()?;
            let mut buf = vec![0u8; count as usize];
            i10.get_sourceLink(count, &mut count, buf.as_mut_ptr())
                .ok()?;
            buf.truncate(count as usize);
            Ok(buf)
        }
    }

    /// The discriminator symbol, offset, and tag value of the discriminated union, when this symbol is a union member.
    pub fn discriminated_union_tag(&self) -> Result<(Option<Symbol>, u32, TagValue)> {
        let i11: IDiaSymbol11 = self.0.cast()?;
        unsafe {
            let mut sym: Option<IDiaSymbol> = None;
            let mut offset: u32 = 0;
            let mut mask = core::mem::zeroed();
            i11.get_discriminatedUnionTag(&mut sym, &mut offset, &mut mask)
                .ok()?;
            Ok((sym.map(Symbol), offset, TagValue::from_abi(mask)))
        }
    }

    /// The tag values of the type ranges of a union symbol.
    pub fn tag_ranges(&self) -> Result<Vec<TagValue>> {
        let i11: IDiaSymbol11 = self.0.cast()?;
        unsafe {
            let mut count: u32 = 0;
            i11.get_tagRanges(0, &mut count, core::ptr::null_mut())
                .ok()?;
            let mut buf = vec![core::mem::zeroed(); count as usize];
            i11.get_tagRanges(count, &mut count, buf.as_mut_ptr())
                .ok()?;
            buf.truncate(count as usize);
            Ok(buf.into_iter().map(TagValue::from_abi).collect())
        }
    }

    /// Retrieves scalable vector register type for the symbol.
    pub fn scalable_register_type(&self) -> Result<ScalableVectorType> {
        let i12: IDiaSymbol12 = self.0.cast()?;
        unsafe {
            Ok(ScalableVectorType::from_abi(
                i12.scalableRegisterType()? as i32
            ))
        }
    }

    /// Whether the symbol is an atomic type.
    pub fn atomic_type(&self) -> Result<bool> {
        let i12: IDiaSymbol12 = self.0.cast()?;
        unsafe { Ok(i12.atomicType()?.as_bool()) }
    }
}
