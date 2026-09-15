//! Stack-walking types.
//!
//! DIA can enumerate the call frames of a running executable. The client
//! supplies a [`StackWalkHelper`] that answers DIA's questions about register
//! values and memory; DIA walks the stack and hands back a [`StackFrames`]
//! enumerator of [`StackFrame`] records.
//!
//! Typical flow:
//!
//! ```no_run
//! use std::sync::Arc;
//! use microsoft_dia::{
//!     constants::MemoryType,
//!     stack::{
//!         StackWalkHelper, StackWalkHelper2, StackWalkHelper3, StackWalkHelperAdapter,
//!         StackWalker,
//!     },
//! };
//! use windows_core::{Error, Result};
//!
//! /// A minimal helper (only the mandatory methods).
//! struct MyHelper;
//!
//! impl StackWalkHelper for MyHelper {
//!     fn register_value(&self, _index: u32) -> Result<u64> { Ok(0) }
//!     fn read_memory(&self, _kind: MemoryType, _va: u64, _buffer: &mut [u8]) -> Result<usize> {
//!         Err(Error::from(windows_core::HRESULT(0x80004001u32 as i32))) // E_NOTIMPL
//!     }
//!     fn search_for_return_address(&self, _frame: &microsoft_dia::records::FrameData) -> Result<u64> {
//!         Err(Error::from(windows_core::HRESULT(0x80004001u32 as i32))) // E_NOTIMPL
//!     }
//! }
//! // Extension tiers: empty impls adopt the `E_NOTIMPL` defaults.
//! impl microsoft_dia::stack::StackWalkHelper2 for MyHelper {}
//! impl StackWalkHelper3 for MyHelper {}
//!
//! # fn main() -> Result<()> {
//! // 1. Create the walker (loads msdia140.dll without COM registration).
//! let walker = StackWalker::new(r"C:\path\to\msdia140.dll")?;
//!
//! // 2. Wrap the helper in a COM object.
//! let adapter = StackWalkHelperAdapter::new(Arc::new(MyHelper));
//!
//! // 3. Walk; the enumerator keeps the helper alive for the walk.
//! for frame in walker.get_enum_frames(adapter)? {
//!     println!("{:?}", frame.return_address());
//! }
//! # Ok(()) }
//! ```

use crate::bindings::*;
use crate::com::no_reg_co_create;
use crate::constants::{CpuType, MemoryType, StackFrameType};
use windows_core::*;

/// A single call frame in the stack being walked.
///
/// Obtained from the [`StackFrames`] enumerator returned by
/// [`StackWalker::get_enum_frames`].
#[derive(Debug)]
pub struct StackFrame(pub(crate) IDiaStackFrame);

impl StackFrame {
    /// The type of the frame.
    pub fn frame_type(&self) -> Result<StackFrameType> {
        unsafe { Ok(StackFrameType::from_abi(self.0.r#type()? as i32)) }
    }

    /// The address base of the frame.
    pub fn base(&self) -> Result<u64> {
        unsafe { self.0.base() }
    }

    /// The size of the frame in bytes.
    pub fn size(&self) -> Result<u32> {
        unsafe { self.0.size() }
    }

    /// The return address of the frame.
    pub fn return_address(&self) -> Result<u64> {
        unsafe { self.0.returnAddress() }
    }

    /// The address base of the frame's local variables.
    pub fn locals_base(&self) -> Result<u64> {
        unsafe { self.0.localsBase() }
    }

    /// The number of bytes of local variables pushed on the stack.
    pub fn length_locals(&self) -> Result<u32> {
        unsafe { self.0.lengthLocals() }
    }

    /// The number of bytes of parameters pushed on the stack.
    pub fn length_params(&self) -> Result<u32> {
        unsafe { self.0.lengthParams() }
    }

    /// The number of bytes of prologue code in the frame.
    pub fn length_prolog(&self) -> Result<u32> {
        unsafe { self.0.lengthProlog() }
    }

    /// The number of bytes of saved registers pushed on the stack.
    pub fn length_saved_registers(&self) -> Result<u32> {
        unsafe { self.0.lengthSavedRegisters() }
    }

    /// Whether system exception handling (e.g. `__try`/`__finally` on MSVC)
    /// is in effect for this frame.
    pub fn system_exception_handling(&self) -> Result<bool> {
        unsafe { Ok(self.0.systemExceptionHandling()?.as_bool()) }
    }

    /// Whether exception handling (`try`/`catch`) is in effect for this frame.
    pub fn cplusplus_exception_handling(&self) -> Result<bool> {
        unsafe { Ok(self.0.cplusplusExceptionHandling()?.as_bool()) }
    }

    /// Whether the frame contains the entry point of a function.
    pub fn function_start(&self) -> Result<bool> {
        unsafe { Ok(self.0.functionStart()?.as_bool()) }
    }

    /// Whether the base pointer is allocated for code in this address range.
    pub fn allocates_base_pointer(&self) -> Result<bool> {
        unsafe { Ok(self.0.allocatesBasePointer()?.as_bool()) }
    }

    /// The maximum number of bytes pushed on the stack in the frame.
    pub fn max_stack(&self) -> Result<u32> {
        unsafe { self.0.maxStack() }
    }

    /// The value of a register at the frame's execution point.
    ///
    /// `index` uses the same register numbering as
    /// [`StackWalkHelper::register_value`].
    pub fn register_value(&self, index: u32) -> Result<u64> {
        unsafe { self.0.registerValue(index) }
    }
}

/// A typed enumerator of [`StackFrame`]s, as returned by
/// [`StackWalker::get_enum_frames`].
///
/// Implements [`Iterator`] with [`StackFrame`] items.
///
/// Note: `IDiaEnumStackFrames` has no `Count`, `Item`, or `Skip` methods, so
/// this enumerator supports only single-step `Next` and `Reset`.
///
/// **Lifetime:** the enumerator holds a reference to the [`StackWalkHelper`]
/// adapter that started the walk, so the client's helper (`Arc` payload)
/// stays alive for as long as the enumerator does.
#[derive(Debug)]
#[allow(dead_code)]
pub struct StackFrames {
    frames: IDiaEnumStackFrames,
    /// The stack-walk helper the walker was driven with. `IDiaEnumStackFrames`
    /// carries no reference to it, so we hold one here to keep the user's
    /// helper alive across the whole walk.
    helper: IDiaStackWalkHelper,
}

impl StackFrames {
    pub(crate) fn new(frames: IDiaEnumStackFrames, helper: IDiaStackWalkHelper) -> Self {
        Self { frames, helper }
    }

    /// Advances the enumerator by one frame.
    ///
    /// `None` is a clean end of enumeration; `Err` is a real COM failure.
    pub fn next_item(&mut self) -> Result<Option<StackFrame>> {
        unsafe {
            let mut slot: Option<IDiaStackFrame> = None;
            let mut fetched: u32 = 0;
            let hr = self.frames.Next(1, &mut slot, &mut fetched);
            if fetched == 0 {
                hr.ok()?;
                Ok(None)
            } else {
                hr.ok()?;
                match slot {
                    Some(t) => Ok(Some(StackFrame(t))),
                    None => Err(Error::from(HRESULT(0x80004005u32 as i32))),
                }
            }
        }
    }

    /// Resets the enumeration to the beginning.
    pub fn reset(&mut self) -> Result<()> {
        unsafe { self.frames.Reset().ok() }
    }
}

impl Iterator for StackFrames {
    type Item = StackFrame;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_item().ok().flatten()
    }
}

/// The client-provided callback that DIA calls while walking a stack
///
/// Only three methods are mandatory; everything else defaults to
/// `E_NOTIMPL`:
///
/// - [`read_memory`](Self::read_memory)
/// - [`register_value`](Self::register_value)
/// - [`search_for_return_address`](Self::search_for_return_address)
///
/// Implement it on your own type, wrap it in an `Arc`, and hand it to
/// [`StackWalkHelperAdapter::new`].
pub trait StackWalkHelper: Send + Sync {
    /// Reads memory from the executable's image into `buffer`, returning the
    /// number of bytes actually read.
    ///
    /// `kind` selects the memory region, `va` the virtual address. When
    /// `buffer` is empty this is a size query: return the number of bytes
    /// available at `va` without reading.
    fn read_memory(&self, kind: MemoryType, va: u64, buffer: &mut [u8]) -> Result<usize>;

    /// Retrieves the value of a register.
    ///
    /// `index` is a register number in the same numbering DIA uses for
    /// [`StackFrame::register_value`].
    fn register_value(&self, index: u32) -> Result<u64>;

    /// Searches the specified stack frame for the nearest function return
    /// address.
    ///
    /// `frame` is the frame data (from [`StackFrame`]-related lookups or
    /// the frame data returned by `frame_for_va`), which is read to locate
    /// the return address.
    fn search_for_return_address(&self, frame: &crate::records::FrameData) -> Result<u64>;

    /// Searches the specified stack frame for a return address at or near the
    /// specified address.
    fn search_for_return_address_start(
        &self,
        frame: &crate::records::FrameData,
        start_address: u64,
    ) -> Result<u64> {
        let _ = (frame, start_address);
        Err(Error::from(HRESULT(0x80004001u32 as i32))) // E_NOTIMPL
    }

    /// Retrieves the stack frame that contains the specified virtual address.
    fn frame_for_va(&self, va: u64) -> Result<crate::records::FrameData> {
        let _ = va;
        Err(Error::from(HRESULT(0x80004001u32 as i32)))
    }

    /// Retrieves the symbol that contains the specified virtual address.
    fn symbol_for_va(&self, va: u64) -> Result<crate::symbol::Symbol> {
        let _ = va;
        Err(Error::from(HRESULT(0x80004001u32 as i32)))
    }

    /// Returns the PDATA data block associated with the specified VA.
    fn pdata_for_va(&self, va: u64, buffer: &mut [u8]) -> Result<usize> {
        let _ = (va, buffer);
        Err(Error::from(HRESULT(0x80004001u32 as i32)))
    }

    /// Returns the starting VA of the executable that contains the specified
    /// VA.
    fn image_for_va(&self, va_context: u64) -> Result<u64> {
        let _ = va_context;
        Err(Error::from(HRESULT(0x80004001u32 as i32)))
    }

    /// Returns the equivalent address (section index and offset) for the
    /// specified VA.
    fn address_for_va(&self, va: u64) -> Result<(u32, u32)> {
        let _ = va;
        Err(Error::from(HRESULT(0x80004001u32 as i32)))
    }

    /// Retrieves the number of discontiguous fragments for the function at
    /// the specified VA.
    fn number_of_function_fragments_for_va(&self, va_func: u64, cb_func: u32) -> Result<u32> {
        let _ = (va_func, cb_func);
        Err(Error::from(HRESULT(0x80004001u32 as i32)))
    }

    /// Retrieves the addresses and lengths of the discontiguous fragments for
    /// the function at the specified VA.
    ///
    /// `c_fragments` is how many fragment slots the caller is providing.
    fn function_fragments_for_va(
        &self,
        va_func: u64,
        cb_func: u32,
        c_fragments: u32,
    ) -> Result<Vec<(u64, u32)>> {
        let _ = (va_func, cb_func, c_fragments);
        Err(Error::from(HRESULT(0x80004001u32 as i32)))
    }

    /// Sets the value of a register.
    fn set_register_value(&self, index: u32, value: u64) -> Result<()> {
        let _ = (index, value);
        Err(Error::from(HRESULT(0x80004001u32 as i32)))
    }
}

/// The `IDiaStackWalkHelper2` extension.
///
/// ARM64 pointer-authentication support. The default returns `E_NOTIMPL`, so
/// a helper that does not support it need not implement this method.
pub trait StackWalkHelper2: StackWalkHelper {
    /// Returns the pointer-authentication mask for the specified value.
    fn get_pointer_authentication_mask(&self, _ptrval: u64) -> Result<u64> {
        Err(Error::from(HRESULT(0x80004001u32 as i32)))
    }
}

/// The `IDiaStackWalkHelper3` extension.
///
/// Variable-size register access, for platforms with large vector registers
/// (e.g. ARM64 SVE). Defaults to `E_NOTIMPL`.
pub trait StackWalkHelper3: StackWalkHelper2 {
    /// Retrieves the value of a register into `buffer`, returning the number
    /// of bytes written.
    fn register_value_bytes(&self, _index: u32, _buffer: &mut [u8]) -> Result<usize> {
        Err(Error::from(HRESULT(0x80004001u32 as i32)))
    }

    /// Sets the value of a register from a byte buffer.
    fn set_register_value_bytes(&self, _index: u32, _value: &[u8]) -> Result<()> {
        Err(Error::from(HRESULT(0x80004001u32 as i32)))
    }
}

/// The COM adapter for a [`StackWalkHelper`].
///
/// Answers the raw `IDiaStackWalkHelper`, `IDiaStackWalkHelper2`, and
/// `IDiaStackWalkHelper3` vtable slots by forwarding to the wrapped helper.
///
/// Build one with [`StackWalkHelperAdapter::new`], convert it to a
/// `IDiaStackWalkHelper` (via `From`/`into`) and pass a reference to
/// [`StackWalker::get_enum_frames`].
#[implement(IDiaStackWalkHelper, IDiaStackWalkHelper2, IDiaStackWalkHelper3)]
pub struct StackWalkHelperAdapter {
    inner: std::sync::Arc<dyn StackWalkHelper3>,
}

impl StackWalkHelperAdapter {
    /// Wraps a stack-walk helper in a COM object.
    ///
    /// The same helper must outlive any [`StackFrames`] enumeration started
    /// from it — an `Arc` is the natural handle: the adapter holds one
    /// strong reference, and you keep the other.
    ///
    /// [`StackWalkHelper3`] is used as the object type because it subsumes
    /// [`StackWalkHelper2`] and [`StackWalkHelper`]; the extension methods
    /// default to `E_NOTIMPL`, so a helper that only implements the base
    /// trait just adds empty `impl StackWalkHelper2 for H {}` /
    /// `impl StackWalkHelper3 for H {}` blocks.
    pub fn new(helper: std::sync::Arc<dyn StackWalkHelper3>) -> Self {
        Self { inner: helper }
    }

    /// The wrapped helper.
    pub fn helper(&self) -> &dyn StackWalkHelper {
        self.inner.as_ref()
    }
}

impl IDiaStackWalkHelper_Impl for StackWalkHelperAdapter_Impl {
    fn registerValue(&self, index: u32) -> Result<u64> {
        self.inner.register_value(index)
    }

    fn SetregisterValue(&self, index: u32, newval: u64) -> Result<()> {
        self.inner.set_register_value(index, newval)
    }

    fn readMemory(
        &self,
        r#type: MemoryTypeEnum,
        va: u64,
        cbdata: u32,
        pcbdata: *mut u32,
        pbdata: *mut u8,
    ) -> Result<()> {
        let kind = MemoryType::from_abi(r#type);
        let n = if pbdata.is_null() {
            self.inner.read_memory(kind, va, &mut [])?
        } else {
            let slice = unsafe { core::slice::from_raw_parts_mut(pbdata, cbdata as usize) };
            self.inner.read_memory(kind, va, slice)?
        };
        if !pcbdata.is_null() {
            unsafe { core::ptr::write(pcbdata, n as u32) };
        }
        Ok(())
    }

    fn searchForReturnAddress(&self, frame: InRef<IDiaFrameData>) -> Result<u64> {
        let fd = frame
            .cloned()
            .ok_or_else(|| Error::from(HRESULT(0x80004003u32 as i32)))?; // E_POINTER
        self.inner
            .search_for_return_address(&crate::records::FrameData(fd))
    }

    fn searchForReturnAddressStart(
        &self,
        frame: InRef<IDiaFrameData>,
        startaddress: u64,
    ) -> Result<u64> {
        let fd = frame
            .cloned()
            .ok_or_else(|| Error::from(HRESULT(0x80004003u32 as i32)))?; // E_POINTER
        self.inner
            .search_for_return_address_start(&crate::records::FrameData(fd), startaddress)
    }

    fn frameForVA(&self, va: u64) -> Result<IDiaFrameData> {
        Ok(self.inner.frame_for_va(va)?.0)
    }

    fn symbolForVA(&self, va: u64) -> Result<IDiaSymbol> {
        Ok(self.inner.symbol_for_va(va)?.0)
    }

    fn pdataForVA(&self, va: u64, cbdata: u32, pcbdata: *mut u32, pbdata: *mut u8) -> Result<()> {
        let n = if pbdata.is_null() {
            self.inner.pdata_for_va(va, &mut [])?
        } else {
            let slice = unsafe { core::slice::from_raw_parts_mut(pbdata, cbdata as usize) };
            self.inner.pdata_for_va(va, slice)?
        };
        if !pcbdata.is_null() {
            unsafe { core::ptr::write(pcbdata, n as u32) };
        }
        Ok(())
    }

    fn imageForVA(&self, vacontext: u64) -> Result<u64> {
        self.inner.image_for_va(vacontext)
    }

    fn addressForVA(&self, va: u64, pisect: *mut u32, poffset: *mut u32) -> Result<()> {
        let (s, o) = self.inner.address_for_va(va)?;
        if !pisect.is_null() {
            unsafe { core::ptr::write(pisect, s) };
        }
        if !poffset.is_null() {
            unsafe { core::ptr::write(poffset, o) };
        }
        Ok(())
    }

    fn numberOfFunctionFragmentsForVA(&self, vafunc: u64, cbfunc: u32) -> Result<u32> {
        self.inner
            .number_of_function_fragments_for_va(vafunc, cbfunc)
    }

    fn functionFragmentsForVA(
        &self,
        vafunc: u64,
        cbfunc: u32,
        cfragments: u32,
        pvafragment: *mut u64,
        plenfragment: *mut u32,
    ) -> Result<()> {
        let frags = self
            .inner
            .function_fragments_for_va(vafunc, cbfunc, cfragments)?;
        for (i, (va, len)) in frags.into_iter().enumerate() {
            if i as u32 >= cfragments {
                break;
            }
            unsafe {
                core::ptr::write(pvafragment.add(i), va);
                core::ptr::write(plenfragment.add(i), len);
            }
        }
        Ok(())
    }
}

impl IDiaStackWalkHelper2_Impl for StackWalkHelperAdapter_Impl {
    fn GetPointerAuthenticationMask(&self, ptrval: u64) -> Result<u64> {
        self.inner.get_pointer_authentication_mask(ptrval)
    }
}

impl IDiaStackWalkHelper3_Impl for StackWalkHelperAdapter_Impl {
    fn get_registerValue(
        &self,
        index: u32,
        cbdata: u32,
        pcbdata: *mut u32,
        pbdata: *mut u8,
    ) -> Result<()> {
        let n = if pbdata.is_null() {
            self.inner.register_value_bytes(index, &mut [])?
        } else {
            let slice = unsafe { core::slice::from_raw_parts_mut(pbdata, cbdata as usize) };
            self.inner.register_value_bytes(index, slice)?
        };
        if !pcbdata.is_null() {
            unsafe { core::ptr::write(pcbdata, n as u32) };
        }
        Ok(())
    }

    fn put_registerValue(&self, index: u32, cbdata: u32, pbdata: *const u8) -> Result<()> {
        let data = unsafe { core::slice::from_raw_parts(pbdata, cbdata as usize) };
        self.inner.set_register_value_bytes(index, data)
    }
}

/// The DIA stack walker.
///
/// Created by [`StackWalker::new`]; then call
/// [`get_enum_frames`](Self::get_enum_frames) with a
/// `IDiaStackWalkHelper` (obtained from a
/// [`StackWalkHelperAdapter`]) to obtain a [`StackFrames`] enumerator.
#[derive(Debug)]
pub struct StackWalker(pub(crate) IDiaStackWalker);

impl StackWalker {
    /// Creates a new stack walker, loading `msdia140.dll` if not already
    /// loaded.
    ///
    /// `lib` is the path to the DIA DLL (typically
    /// `"C:\\Program Files (x86)\\Microsoft Visual Studio\\2022\\BuildTools\\DIA SDK\\lib\\amd64\\msdia140.dll"`
    /// or a path on `PATH`).
    pub fn new(lib: &str) -> Result<Self> {
        Ok(Self(no_reg_co_create::<IDiaStackWalker>(
            lib,
            &DiaStackWalker,
        )?))
    }

    /// Retrieves a stack frame enumerator for x86 platforms.
    ///
    /// `helper` answers DIA's register/memory queries while it walks the
    /// stack. The enumerator keeps it (and its `Arc` payload) alive for the
    /// duration of the walk.
    pub fn get_enum_frames(&self, helper: StackWalkHelperAdapter) -> Result<StackFrames> {
        unsafe {
            let helper: IDiaStackWalkHelper = helper.into();
            let frames = self.0.getEnumFrames(&helper)?;
            Ok(StackFrames::new(frames, helper))
        }
    }

    /// Retrieves a stack frame enumerator for a specific platform type.
    ///
    /// `helper` answers DIA's register/memory queries while it walks the
    /// stack. The enumerator keeps it (and its `Arc` payload) alive for the
    /// duration of the walk.
    pub fn get_enum_frames2(
        &self,
        cpu_type: CpuType,
        helper: StackWalkHelperAdapter,
    ) -> Result<StackFrames> {
        unsafe {
            let helper: IDiaStackWalkHelper = helper.into();
            let frames = self.0.getEnumFrames2(cpu_type.abi(), &helper)?;
            Ok(StackFrames::new(frames, helper))
        }
    }
}
