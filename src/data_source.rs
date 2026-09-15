//! The DIA data source: opens the DIA SDK against an image or PDB.

use crate::bindings::*;
use crate::callbacks::LoadCallbackAdapter;
use crate::com::no_reg_co_create;
use crate::constants::NameSearchOptions;
use crate::enums::NamedStreams;
use crate::session::Session;
use windows_core::*;

/// A DIA data source, opened against `msdia140.dll`.
///
/// Wraps `IDiaDataSource` and also exposes the `IDiaDataSourceEx`
/// extension methods.
#[derive(Clone, Debug)]
pub struct DataSource(pub(crate) IDiaDataSource);

impl DataSource {
    /// Opens the DIA SDK and returns a data source, searching the
    /// well-known `msdia140.dll` locations.
    ///
    /// Tries Visual Studio install paths (x86_64 / aarch64 / x86), WinDbg,
    /// and the process directory, in that order. The first path that loads
    /// and creates a data source wins.
    pub fn open() -> Result<Self> {
        let paths = msdia140_paths();
        let mut last = Error::empty();
        for path in paths {
            match no_reg_co_create::<IDiaDataSource>(&path, &DiaSource) {
                Ok(src) => return Ok(Self(src)),
                Err(e) => last = e,
            }
        }
        Err(last)
    }

    /// Opens the DIA SDK from an explicit `msdia140.dll` path.
    pub fn open_at(path: &str) -> Result<Self> {
        let src = no_reg_co_create::<IDiaDataSource>(path, &DiaSource)?;
        Ok(Self(src))
    }

    fn ex(&self) -> Result<IDiaDataSourceEx> {
        self.0.cast()
    }

    /// Loads the specified executable's debug data.
    ///
    /// `search_path` is an optional directory to look in for the matching
    /// PDB.
    pub fn load_exe(&self, executable: &str, search_path: Option<&str>) -> Result<()> {
        unsafe {
            let path = match search_path {
                Some(s) => PCWSTR::from_raw(HSTRING::from(s).as_ptr()),
                None => PCWSTR::null(),
            };
            self.0
                .loadDataForExe(&HSTRING::from(executable), path, None)
                .ok()
        }
    }

    /// Loads the specified executable's debug data, reporting progress and
    /// restricting debug-information sources through `callback`.
    ///
    /// `search_path` is an optional directory to look in for the matching
    /// PDB; `callback` is a [`LoadCallback`](crate::callbacks::LoadCallback)
    /// adapter (it keeps the callback's `Arc` payload alive for the load).
    /// `IDiaLoadCallback2` parameter).
    pub fn load_exe_with_callback(
        &self,
        executable: &str,
        search_path: Option<&str>,
        callback: LoadCallbackAdapter,
    ) -> Result<()> {
        let callback: IUnknown = callback.into();
        unsafe {
            let path = match search_path {
                Some(s) => PCWSTR::from_raw(HSTRING::from(s).as_ptr()),
                None => PCWSTR::null(),
            };
            self.0
                .loadDataForExe(&HSTRING::from(executable), path, &callback)
                .ok()
        }
    }

    /// Loads the specified PDB's debug data.
    pub fn load_pdb(&self, pdb_path: &str) -> Result<()> {
        unsafe { self.0.loadDataFromPdb(&HSTRING::from(pdb_path)).ok() }
    }

    /// Opens a new symbol session.
    pub fn open_session(&self) -> Result<Session> {
        unsafe { Ok(Session(self.0.openSession()?)) }
    }

    /// The `HRESULT` of the last operation that failed.
    pub fn last_error(&self) -> Result<String> {
        unsafe { Ok(self.0.lastError()?.display().to_string()) }
    }

    /// Gets the size of a raw debug stream, in bytes.
    pub fn get_stream_size(&self, stream: &str) -> Result<u64> {
        unsafe { self.ex()?.getStreamSize(&HSTRING::from(stream)) }
    }

    /// Gets a portion of a raw debug stream.
    ///
    /// Returns the number of bytes actually read.
    pub fn get_stream_raw_data(&self, stream: &str, offset: u64, buf: &mut [u8]) -> Result<usize> {
        unsafe {
            let mut fetched: u64 = 0;
            self.ex()?
                .getStreamRawData(
                    &HSTRING::from(stream),
                    offset,
                    buf.len() as u64,
                    &mut fetched,
                    buf.as_mut_ptr(),
                )
                .ok()?;
            Ok(fetched as usize)
        }
    }

    /// Validates the specified PDB file.
    pub fn validate_pdb(&self, pdb_path: &str, pcsig70: &GUID, sig: u32, age: u32) -> Result<bool> {
        unsafe {
            Ok(self
                .ex()?
                .ValidatePdb(&HSTRING::from(pdb_path), pcsig70, sig, age)?
                .as_bool())
        }
    }

    fn ex2(&self) -> Result<IDiaDataSourceEx2> {
        self.0.cast()
    }

    /// Loads the specified PDB's debug data, optionally prefetching.
    pub fn load_pdb_ex(&self, pdb_path: &str, prefetch: bool) -> Result<()> {
        unsafe {
            self.ex()?
                .loadDataFromPdbEx(&HSTRING::from(pdb_path), prefetch)
                .ok()
        }
    }

    /// Opens and verifies that the program database (.pdb) file matches the
    /// signature information provided; prepares the .pdb file as a debug
    /// data source.
    pub fn load_and_validate_pdb(
        &self,
        pdb_path: &str,
        sig70: &GUID,
        sig: u32,
        age: u32,
    ) -> Result<()> {
        unsafe {
            self.0
                .loadAndValidateDataFromPdb(&HSTRING::from(pdb_path), sig70, sig, age)
                .ok()
        }
    }

    /// Opens and verifies that the program database (.pdb) file matches the
    /// signature information provided; prepares the .pdb file as a debug
    /// data source, optionally prefetching.
    pub fn load_and_validate_pdb_ex(
        &self,
        pdb_path: &str,
        sig70: &GUID,
        sig: u32,
        age: u32,
        prefetch: bool,
    ) -> Result<()> {
        unsafe {
            self.ex()?
                .loadAndValidateDataFromPdbEx(&HSTRING::from(pdb_path), sig70, sig, age, prefetch)
                .ok()
        }
    }

    /// Opens and prepares the debug data associated with the .exe/.dll
    /// file, reporting progress and restricting debug-information sources
    /// through `callback`, optionally prefetching.
    ///
    /// `search_path` is an optional directory to look in for the matching
    /// PDB.
    pub fn load_exe_ex(
        &self,
        executable: &str,
        search_path: Option<&str>,
        callback: LoadCallbackAdapter,
        prefetch: bool,
    ) -> Result<()> {
        let callback: IUnknown = callback.into();
        unsafe {
            let path = match search_path {
                Some(s) => PCWSTR::from_raw(HSTRING::from(s).as_ptr()),
                None => PCWSTR::null(),
            };
            self.ex()?
                .loadDataForExeEx(&HSTRING::from(executable), path, &callback, prefetch)
                .ok()
        }
    }

    /// Opens and prepares the debug data associated with the provided
    /// CodeView information, reporting progress and restricting
    /// debug-information sources through `callback`.
    pub fn load_from_code_view_info(
        &self,
        executable: &str,
        search_path: Option<&str>,
        cv_info: &[u8],
        callback: LoadCallbackAdapter,
    ) -> Result<()> {
        let callback: IUnknown = callback.into();
        unsafe {
            let path = match search_path {
                Some(s) => PCWSTR::from_raw(HSTRING::from(s).as_ptr()),
                None => PCWSTR::null(),
            };
            self.0
                .loadDataFromCodeViewInfo(
                    &HSTRING::from(executable),
                    path,
                    cv_info.len() as u32,
                    cv_info.as_ptr(),
                    &callback,
                )
                .ok()
        }
    }

    /// Opens and prepares the debug data associated with the provided
    /// debug information, reporting progress and restricting
    /// debug-information sources through `callback`.
    #[allow(clippy::too_many_arguments)]
    pub fn load_from_misc_info(
        &self,
        executable: &str,
        search_path: Option<&str>,
        timestamp_exe: u32,
        timestamp_dbg: u32,
        sizeof_exe: u32,
        misc_info: &[u8],
        callback: LoadCallbackAdapter,
    ) -> Result<()> {
        let callback: IUnknown = callback.into();
        unsafe {
            let path = match search_path {
                Some(s) => PCWSTR::from_raw(HSTRING::from(s).as_ptr()),
                None => PCWSTR::null(),
            };
            self.0
                .loadDataFromMiscInfo(
                    &HSTRING::from(executable),
                    path,
                    timestamp_exe,
                    timestamp_dbg,
                    sizeof_exe,
                    misc_info.len() as u32,
                    misc_info.as_ptr(),
                    &callback,
                )
                .ok()
        }
    }

    /// Retrieves all names of named streams within the data source that
    /// match the optionally given pattern.
    pub fn find_named_streams(
        &self,
        name: &str,
        compare_flags: NameSearchOptions,
    ) -> Result<NamedStreams> {
        unsafe {
            Ok(NamedStreams(self.ex2()?.findNamedStreams(
                &HSTRING::from(name),
                compare_flags.abi(),
            )?))
        }
    }
}

fn msdia140_paths() -> Vec<String> {
    let arch = if cfg!(target_arch = "x86_64") {
        "amd64"
    } else if cfg!(target_arch = "aarch64") {
        "arm64"
    } else if cfg!(target_arch = "x86") {
        ""
    } else {
        return Vec::new();
    };

    let mut paths = vec![];
    let roots = [
        r"C:\Program Files\Microsoft Visual Studio\18\Insiders\DIA SDK\bin",
        r"C:\Program Files\Microsoft Visual Studio\18\Professional\DIA SDK\bin",
        r"C:\Program Files\Microsoft Visual Studio\18\Enterprise\DIA SDK\bin",
        r"C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\DIA SDK\bin",
    ];
    for root in roots {
        let path = if arch.is_empty() {
            format!("{root}\\msdia140.dll")
        } else {
            format!("{root}\\{arch}\\msdia140.dll")
        };
        paths.push(path);
    }
    let kit = if arch.is_empty() {
        r"C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\msdia140.dll"
    } else if arch == "arm64" {
        r"C:\Program Files (x86)\Windows Kits\10\Debuggers\arm64\msdia140.dll"
    } else {
        r"C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\msdia140.dll"
    };
    paths.push(kit.to_string());
    paths.push("msdia140.dll".to_string());
    paths
}
