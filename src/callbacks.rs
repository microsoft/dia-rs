//! Load-time callbacks for [`DataSource`](crate::data_source::DataSource).
//!
//! When DIA loads an executable's debug data it may consult the registry, a
//! symbol server, or candidate files in several locations. A [`LoadCallback`]
//! lets the client observe each candidate it opens and veto (or keep) each
//! source of debug information.
//!
//! Implement [`LoadCallback`] on your own type, wrap it in an `Arc`, and
//! pass the resulting [`LoadCallbackAdapter`] to
//! [`DataSource::load_exe_with_callback`](crate::data_source::DataSource::load_exe_with_callback).
//!
//! ```no_run
//! use std::sync::Arc;
//! use microsoft_dia::{
//!     callbacks::LoadCallback,
//!     data_source::DataSource,
//! };
//! use windows_core::{Result, HRESULT};
//!
//! /// Restrict everything except the symbol server.
//! struct StrictLoader;
//!
//! impl LoadCallback for StrictLoader {
//!     fn restrict_registry_access(&self) -> Result<bool> {
//!         Ok(false) // S_FALSE: don't query the registry.
//!     }
//!     fn restrict_symbol_server_access(&self) -> Result<bool> {
//!         Ok(true) // S_OK: symbol server allowed.
//!     }
//! }
//!
//! # fn main() -> Result<()> {
//! let source = DataSource::open()?;
//! source.load_exe_with_callback(
//!     r"C:\foo\bar.exe",
//!     None,
//!     microsoft_dia::callbacks::LoadCallbackAdapter::new(Arc::new(StrictLoader)),
//! )?;
//! # Ok(()) }
//! ```
//!
//! **Contract:** each `Restrict*` method *allows* the access by returning
//! `Ok(true)` and *restricts* it by returning `Ok(false)`. (Under the hood
//! the SDK distinguishes allow from restrict by COM status code; the safe
//! API hides that.)

use crate::bindings::*;
use windows_core::*;

/// Load-time callbacks that the debug store calls while it loads an image.
///
/// All methods are optional; the defaults allow every access and swallow the
/// notifications. Implement only the ones you care about.
pub trait LoadCallback: Send + Sync {
    /// Called when a debug directory was found in the `.exe` file.
    ///
    /// `executable` is whether the image is an executable (vs. a DLL);
    /// `data` is the raw debug-directory payload.
    fn notify_debug_dir(&self, _executable: bool, _data: &[u8]) -> Result<()> {
        Ok(())
    }

    /// Called when a candidate `.dbg` file has been opened.
    ///
    /// `result` is the `HRESULT` of the open attempt (`S_OK` means the file
    /// was opened successfully; `S_FALSE` means it was rejected as a bad
    /// candidate; any error means the open failed).
    fn notify_open_dbg(&self, _path: &str, _result: HRESULT) -> Result<()> {
        Ok(())
    }

    /// Called when a candidate `.pdb` file has been opened.
    ///
    /// `result` is the `HRESULT` of the open attempt (see
    /// [`notify_open_dbg`](Self::notify_open_dbg)).
    fn notify_open_pdb(&self, _path: &str, _result: HRESULT) -> Result<()> {
        Ok(())
    }

    /// Determines if registry queries can be used to locate symbol search
    /// paths.
    ///
    /// Return `Ok(true)` to allow (`S_OK`) or `Ok(false)` to restrict
    /// (`S_FALSE`).
    fn restrict_registry_access(&self) -> Result<bool> {
        Ok(true)
    }

    /// Determines if access is allowed to a symbol server to resolve symbols.
    ///
    /// Return `Ok(true)` to allow (`S_OK`) or `Ok(false)` to restrict
    /// (`S_FALSE`).
    fn restrict_symbol_server_access(&self) -> Result<bool> {
        Ok(true)
    }

    /// Determines if looking for a `.pdb` file in the original debug
    /// directory is allowed.
    ///
    /// Return `Ok(true)` to allow (`S_OK`) or `Ok(false)` to restrict
    /// (`S_FALSE`).
    fn restrict_original_path_access(&self) -> Result<bool> {
        Ok(true)
    }

    /// Determines if looking for a `.pdb` file in the path where the `.exe`
    /// file is located is allowed.
    ///
    /// Return `Ok(true)` to allow (`S_OK`) or `Ok(false)` to restrict
    /// (`S_FALSE`).
    fn restrict_reference_path_access(&self) -> Result<bool> {
        Ok(true)
    }

    /// Determines if looking for debug information from `.dbg` files is
    /// allowed.
    ///
    /// Return `Ok(true)` to allow (`S_OK`) or `Ok(false)` to restrict
    /// (`S_FALSE`).
    fn restrict_dbg_access(&self) -> Result<bool> {
        Ok(true)
    }

    /// Determines if searching for `.pdb` files in the system root directory
    /// is allowed.
    ///
    /// Return `Ok(true)` to allow (`S_OK`) or `Ok(false)` to restrict
    /// (`S_FALSE`).
    fn restrict_system_root_access(&self) -> Result<bool> {
        Ok(true)
    }
}

/// `S_FALSE` (`0x0000_0001`) — the "restricted" verdict for `Restrict*`.
///
/// `windows_core` does not define a `S_FALSE` constant, so the mapping from
/// the safe API's `Result<bool>` to an `HRESULT` is spelled out here.
const S_FALSE: i32 = 0x0000_0001;

fn verdict(allow: bool) -> Result<()> {
    if allow {
        Ok(())
    } else {
        Err(Error::from_hresult(HRESULT(S_FALSE)))
    }
}

/// The COM adapter for a [`LoadCallback`].
///
/// Answers the raw `IDiaLoadCallback` and `IDiaLoadCallback2` vtable slots
/// by forwarding to the wrapped callback.
///
/// Build one with [`LoadCallbackAdapter::new`]; the adapter (or an owned
/// `IDiaLoadCallback`/`IUnknown` derived from it) keeps the callback's `Arc`
/// payload alive for the duration of the load.
#[implement(IDiaLoadCallback, IDiaLoadCallback2)]
pub struct LoadCallbackAdapter {
    inner: std::sync::Arc<dyn LoadCallback>,
}

impl LoadCallbackAdapter {
    /// Wraps a load callback in a COM object.
    ///
    /// The same callback must outlive any load started from it — an
    /// `Arc` is the natural handle: the adapter holds one strong
    /// reference, and you keep the other.
    pub fn new(callback: std::sync::Arc<dyn LoadCallback>) -> Self {
        Self { inner: callback }
    }

    /// The wrapped callback.
    pub fn callback(&self) -> &dyn LoadCallback {
        self.inner.as_ref()
    }
}

impl IDiaLoadCallback_Impl for LoadCallbackAdapter_Impl {
    fn NotifyDebugDir(&self, fexecutable: BOOL, cbdata: u32, pbdata: *const u8) -> Result<()> {
        let data = if pbdata.is_null() {
            &[] as &[u8]
        } else {
            unsafe { core::slice::from_raw_parts(pbdata, cbdata as usize) }
        };
        self.inner.notify_debug_dir(fexecutable.as_bool(), data)
    }

    fn NotifyOpenDBG(&self, dbgpath: &PCWSTR, resultcode: HRESULT) -> Result<()> {
        let path = unsafe { dbgpath.display().to_string() };
        self.inner.notify_open_dbg(&path, resultcode)
    }

    fn NotifyOpenPDB(&self, pdbpath: &PCWSTR, resultcode: HRESULT) -> Result<()> {
        let path = unsafe { pdbpath.display().to_string() };
        self.inner.notify_open_pdb(&path, resultcode)
    }

    fn RestrictRegistryAccess(&self) -> Result<()> {
        verdict(self.inner.restrict_registry_access()?)
    }

    fn RestrictSymbolServerAccess(&self) -> Result<()> {
        verdict(self.inner.restrict_symbol_server_access()?)
    }
}

impl IDiaLoadCallback2_Impl for LoadCallbackAdapter_Impl {
    fn RestrictOriginalPathAccess(&self) -> Result<()> {
        verdict(self.inner.restrict_original_path_access()?)
    }

    fn RestrictReferencePathAccess(&self) -> Result<()> {
        verdict(self.inner.restrict_reference_path_access()?)
    }

    fn RestrictDBGAccess(&self) -> Result<()> {
        verdict(self.inner.restrict_dbg_access()?)
    }

    fn RestrictSystemRootAccess(&self) -> Result<()> {
        verdict(self.inner.restrict_system_root_access()?)
    }
}
