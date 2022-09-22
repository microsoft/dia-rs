pub trait IDiaAddressMap_Impl: Sized {
    fn addressMapEnabled(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn SetaddressMapEnabled(&self, newval: ::windows::Win32::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn relativeVirtualAddressEnabled(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn SetrelativeVirtualAddressEnabled(&self, newval: ::windows::Win32::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn imageAlign(&self) -> ::windows::core::Result<u32>;
    fn SetimageAlign(&self, newval: u32) -> ::windows::core::Result<()>;
    fn set_imageHeaders(&self, cbdata: u32, pbdata: *const u8, originalheaders: ::windows::Win32::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn set_addressMap(&self, cdata: u32, pdata: *const DiaAddressMapEntry, imagetosymbols: ::windows::Win32::Foundation::BOOL) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDiaAddressMap {}
impl IDiaAddressMap_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaAddressMap_Impl, const OFFSET: isize>() -> IDiaAddressMap_Vtbl {
        unsafe extern "system" fn addressMapEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaAddressMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.addressMapEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetaddressMapEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaAddressMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetaddressMapEnabled(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn relativeVirtualAddressEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaAddressMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.relativeVirtualAddressEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetrelativeVirtualAddressEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaAddressMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetrelativeVirtualAddressEnabled(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn imageAlign<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaAddressMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.imageAlign() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetimageAlign<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaAddressMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetimageAlign(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn set_imageHeaders<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaAddressMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbdata: u32, pbdata: *const u8, originalheaders: ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.set_imageHeaders(::core::mem::transmute_copy(&cbdata), ::core::mem::transmute_copy(&pbdata), ::core::mem::transmute_copy(&originalheaders)).into()
        }
        unsafe extern "system" fn set_addressMap<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaAddressMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cdata: u32, pdata: *const DiaAddressMapEntry, imagetosymbols: ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.set_addressMap(::core::mem::transmute_copy(&cdata), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&imagetosymbols)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            addressMapEnabled: addressMapEnabled::<Identity, Impl, OFFSET>,
            SetaddressMapEnabled: SetaddressMapEnabled::<Identity, Impl, OFFSET>,
            relativeVirtualAddressEnabled: relativeVirtualAddressEnabled::<Identity, Impl, OFFSET>,
            SetrelativeVirtualAddressEnabled: SetrelativeVirtualAddressEnabled::<Identity, Impl, OFFSET>,
            imageAlign: imageAlign::<Identity, Impl, OFFSET>,
            SetimageAlign: SetimageAlign::<Identity, Impl, OFFSET>,
            set_imageHeaders: set_imageHeaders::<Identity, Impl, OFFSET>,
            set_addressMap: set_addressMap::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiaAddressMap as ::windows::core::Interface>::IID
    }
}
pub trait IDiaDataSource_Impl: Sized {
    fn lastError(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn loadDataFromPdb(&self, pdbpath: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn loadAndValidateDataFromPdb(&self, pdbpath: &::windows::core::PCWSTR, pcsig70: *const ::windows::core::GUID, sig: u32, age: u32) -> ::windows::core::Result<()>;
    fn loadDataForExe(&self, executable: &::windows::core::PCWSTR, searchpath: &::windows::core::PCWSTR, pcallback: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn loadDataFromIStream(&self, pistream: &::core::option::Option<::windows::Win32::System::Com::IStream>) -> ::windows::core::Result<()>;
    fn openSession(&self) -> ::windows::core::Result<IDiaSession>;
    fn loadDataFromCodeViewInfo(&self, executable: &::windows::core::PCWSTR, searchpath: &::windows::core::PCWSTR, cbcvinfo: u32, pbcvinfo: *const u8, pcallback: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn loadDataFromMiscInfo(&self, executable: &::windows::core::PCWSTR, searchpath: &::windows::core::PCWSTR, timestampexe: u32, timestampdbg: u32, sizeofexe: u32, cbmiscinfo: u32, pbmiscinfo: *const u8, pcallback: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDiaDataSource {}
impl IDiaDataSource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaDataSource_Impl, const OFFSET: isize>() -> IDiaDataSource_Vtbl {
        unsafe extern "system" fn lastError<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaDataSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.lastError() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn loadDataFromPdb<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaDataSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdbpath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.loadDataFromPdb(::core::mem::transmute(&pdbpath)).into()
        }
        unsafe extern "system" fn loadAndValidateDataFromPdb<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaDataSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdbpath: ::windows::core::PCWSTR, pcsig70: *const ::windows::core::GUID, sig: u32, age: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.loadAndValidateDataFromPdb(::core::mem::transmute(&pdbpath), ::core::mem::transmute_copy(&pcsig70), ::core::mem::transmute_copy(&sig), ::core::mem::transmute_copy(&age)).into()
        }
        unsafe extern "system" fn loadDataForExe<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaDataSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, executable: ::windows::core::PCWSTR, searchpath: ::windows::core::PCWSTR, pcallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.loadDataForExe(::core::mem::transmute(&executable), ::core::mem::transmute(&searchpath), ::core::mem::transmute(&pcallback)).into()
        }
        unsafe extern "system" fn loadDataFromIStream<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaDataSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistream: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.loadDataFromIStream(::core::mem::transmute(&pistream)).into()
        }
        unsafe extern "system" fn openSession<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaDataSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.openSession() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsession, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn loadDataFromCodeViewInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaDataSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, executable: ::windows::core::PCWSTR, searchpath: ::windows::core::PCWSTR, cbcvinfo: u32, pbcvinfo: *const u8, pcallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.loadDataFromCodeViewInfo(::core::mem::transmute(&executable), ::core::mem::transmute(&searchpath), ::core::mem::transmute_copy(&cbcvinfo), ::core::mem::transmute_copy(&pbcvinfo), ::core::mem::transmute(&pcallback)).into()
        }
        unsafe extern "system" fn loadDataFromMiscInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaDataSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, executable: ::windows::core::PCWSTR, searchpath: ::windows::core::PCWSTR, timestampexe: u32, timestampdbg: u32, sizeofexe: u32, cbmiscinfo: u32, pbmiscinfo: *const u8, pcallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.loadDataFromMiscInfo(
                ::core::mem::transmute(&executable),
                ::core::mem::transmute(&searchpath),
                ::core::mem::transmute_copy(&timestampexe),
                ::core::mem::transmute_copy(&timestampdbg),
                ::core::mem::transmute_copy(&sizeofexe),
                ::core::mem::transmute_copy(&cbmiscinfo),
                ::core::mem::transmute_copy(&pbmiscinfo),
                ::core::mem::transmute(&pcallback),
            )
            .into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            lastError: lastError::<Identity, Impl, OFFSET>,
            loadDataFromPdb: loadDataFromPdb::<Identity, Impl, OFFSET>,
            loadAndValidateDataFromPdb: loadAndValidateDataFromPdb::<Identity, Impl, OFFSET>,
            loadDataForExe: loadDataForExe::<Identity, Impl, OFFSET>,
            loadDataFromIStream: loadDataFromIStream::<Identity, Impl, OFFSET>,
            openSession: openSession::<Identity, Impl, OFFSET>,
            loadDataFromCodeViewInfo: loadDataFromCodeViewInfo::<Identity, Impl, OFFSET>,
            loadDataFromMiscInfo: loadDataFromMiscInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiaDataSource as ::windows::core::Interface>::IID
    }
}
pub trait IDiaEnumDebugStreamData_Impl: Sized {
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn name(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn Item(&self, index: u32, cbdata: u32, pcbdata: *mut u32, pbdata: *mut u8) -> ::windows::core::Result<()>;
    fn Next(&self, celt: u32, cbdata: u32, pcbdata: *mut u32, pbdata: *mut u8, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IDiaEnumDebugStreamData>;
}
impl ::windows::core::RuntimeName for IDiaEnumDebugStreamData {}
impl IDiaEnumDebugStreamData_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumDebugStreamData_Impl, const OFFSET: isize>() -> IDiaEnumDebugStreamData_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumDebugStreamData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumDebugStreamData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumDebugStreamData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumDebugStreamData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, cbdata: u32, pcbdata: *mut u32, pbdata: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Item(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&cbdata), ::core::mem::transmute_copy(&pcbdata), ::core::mem::transmute_copy(&pbdata)).into()
        }
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumDebugStreamData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, cbdata: u32, pcbdata: *mut u32, pbdata: *mut u8, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&cbdata), ::core::mem::transmute_copy(&pcbdata), ::core::mem::transmute_copy(&pbdata), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumDebugStreamData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumDebugStreamData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumDebugStreamData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            name: name::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiaEnumDebugStreamData as ::windows::core::Interface>::IID
    }
}
pub trait IDiaEnumDebugStreams_Impl: Sized {
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn Item(&self, index: &::windows::Win32::System::Com::VARIANT) -> ::windows::core::Result<IDiaEnumDebugStreamData>;
    fn Next(&self, celt: u32, rgelt: *mut ::core::option::Option<IDiaEnumDebugStreamData>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IDiaEnumDebugStreams>;
}
impl ::windows::core::RuntimeName for IDiaEnumDebugStreams {}
impl IDiaEnumDebugStreams_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumDebugStreams_Impl, const OFFSET: isize>() -> IDiaEnumDebugStreams_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumDebugStreams_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumDebugStreams_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumDebugStreams_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<::windows::Win32::System::Com::VARIANT>, stream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Item(::core::mem::transmute(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(stream, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumDebugStreams_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumDebugStreams_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumDebugStreams_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumDebugStreams_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiaEnumDebugStreams as ::windows::core::Interface>::IID
    }
}
pub trait IDiaEnumFrameData_Impl: Sized {
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn Item(&self, index: u32) -> ::windows::core::Result<IDiaFrameData>;
    fn Next(&self, celt: u32, rgelt: *mut ::core::option::Option<IDiaFrameData>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IDiaEnumFrameData>;
    fn frameByRVA(&self, relativevirtualaddress: u32) -> ::windows::core::Result<IDiaFrameData>;
    fn frameByVA(&self, virtualaddress: u64) -> ::windows::core::Result<IDiaFrameData>;
}
impl ::windows::core::RuntimeName for IDiaEnumFrameData {}
impl IDiaEnumFrameData_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumFrameData_Impl, const OFFSET: isize>() -> IDiaEnumFrameData_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumFrameData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumFrameData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumFrameData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, frame: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(frame, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumFrameData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumFrameData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumFrameData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumFrameData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn frameByRVA<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumFrameData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relativevirtualaddress: u32, frame: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.frameByRVA(::core::mem::transmute_copy(&relativevirtualaddress)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(frame, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn frameByVA<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumFrameData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, virtualaddress: u64, frame: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.frameByVA(::core::mem::transmute_copy(&virtualaddress)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(frame, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            frameByRVA: frameByRVA::<Identity, Impl, OFFSET>,
            frameByVA: frameByVA::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiaEnumFrameData as ::windows::core::Interface>::IID
    }
}
pub trait IDiaEnumInjectedSources_Impl: Sized {
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn Item(&self, index: u32) -> ::windows::core::Result<IDiaInjectedSource>;
    fn Next(&self, celt: u32, rgelt: *mut ::core::option::Option<IDiaInjectedSource>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IDiaEnumInjectedSources>;
}
impl ::windows::core::RuntimeName for IDiaEnumInjectedSources {}
impl IDiaEnumInjectedSources_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumInjectedSources_Impl, const OFFSET: isize>() -> IDiaEnumInjectedSources_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumInjectedSources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumInjectedSources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumInjectedSources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, injectedsource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(injectedsource, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumInjectedSources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumInjectedSources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumInjectedSources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumInjectedSources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiaEnumInjectedSources as ::windows::core::Interface>::IID
    }
}
pub trait IDiaEnumInputAssemblyFiles_Impl: Sized {
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn Item(&self, index: u32) -> ::windows::core::Result<IDiaInputAssemblyFile>;
    fn Next(&self, celt: u32, rgelt: *mut ::core::option::Option<IDiaInputAssemblyFile>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IDiaEnumInputAssemblyFiles>;
}
impl ::windows::core::RuntimeName for IDiaEnumInputAssemblyFiles {}
impl IDiaEnumInputAssemblyFiles_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumInputAssemblyFiles_Impl, const OFFSET: isize>() -> IDiaEnumInputAssemblyFiles_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumInputAssemblyFiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumInputAssemblyFiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumInputAssemblyFiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, file: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(file, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumInputAssemblyFiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumInputAssemblyFiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumInputAssemblyFiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumInputAssemblyFiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiaEnumInputAssemblyFiles as ::windows::core::Interface>::IID
    }
}
pub trait IDiaEnumLineNumbers_Impl: Sized {
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn Item(&self, index: u32) -> ::windows::core::Result<IDiaLineNumber>;
    fn Next(&self, celt: u32, rgelt: *mut ::core::option::Option<IDiaLineNumber>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IDiaEnumLineNumbers>;
}
impl ::windows::core::RuntimeName for IDiaEnumLineNumbers {}
impl IDiaEnumLineNumbers_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumLineNumbers_Impl, const OFFSET: isize>() -> IDiaEnumLineNumbers_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumLineNumbers_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumLineNumbers_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumLineNumbers_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, linenumber: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(linenumber, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumLineNumbers_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumLineNumbers_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumLineNumbers_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumLineNumbers_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiaEnumLineNumbers as ::windows::core::Interface>::IID
    }
}
pub trait IDiaEnumSectionContribs_Impl: Sized {
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn Item(&self, index: u32) -> ::windows::core::Result<IDiaSectionContrib>;
    fn Next(&self, celt: u32, rgelt: *mut ::core::option::Option<IDiaSectionContrib>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IDiaEnumSectionContribs>;
}
impl ::windows::core::RuntimeName for IDiaEnumSectionContribs {}
impl IDiaEnumSectionContribs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumSectionContribs_Impl, const OFFSET: isize>() -> IDiaEnumSectionContribs_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumSectionContribs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumSectionContribs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumSectionContribs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, section: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(section, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumSectionContribs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumSectionContribs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumSectionContribs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumSectionContribs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiaEnumSectionContribs as ::windows::core::Interface>::IID
    }
}
pub trait IDiaEnumSegments_Impl: Sized {
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn Item(&self, index: u32) -> ::windows::core::Result<IDiaSegment>;
    fn Next(&self, celt: u32, rgelt: *mut ::core::option::Option<IDiaSegment>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IDiaEnumSegments>;
}
impl ::windows::core::RuntimeName for IDiaEnumSegments {}
impl IDiaEnumSegments_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumSegments_Impl, const OFFSET: isize>() -> IDiaEnumSegments_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumSegments_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumSegments_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumSegments_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, segment: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(segment, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumSegments_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumSegments_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumSegments_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumSegments_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiaEnumSegments as ::windows::core::Interface>::IID
    }
}
pub trait IDiaEnumSourceFiles_Impl: Sized {
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn Item(&self, index: u32) -> ::windows::core::Result<IDiaSourceFile>;
    fn Next(&self, celt: u32, rgelt: *mut ::core::option::Option<IDiaSourceFile>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IDiaEnumSourceFiles>;
}
impl ::windows::core::RuntimeName for IDiaEnumSourceFiles {}
impl IDiaEnumSourceFiles_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumSourceFiles_Impl, const OFFSET: isize>() -> IDiaEnumSourceFiles_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumSourceFiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumSourceFiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumSourceFiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, sourcefile: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(sourcefile, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumSourceFiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumSourceFiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumSourceFiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumSourceFiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiaEnumSourceFiles as ::windows::core::Interface>::IID
    }
}
pub trait IDiaEnumStackFrames_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut ::core::option::Option<IDiaStackFrame>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDiaEnumStackFrames {}
impl IDiaEnumStackFrames_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumStackFrames_Impl, const OFFSET: isize>() -> IDiaEnumStackFrames_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumStackFrames_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumStackFrames_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiaEnumStackFrames as ::windows::core::Interface>::IID
    }
}
pub trait IDiaEnumSymbols_Impl: Sized {
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn Item(&self, index: u32) -> ::windows::core::Result<IDiaSymbol>;
    fn Next(&self, celt: u32, rgelt: *mut ::core::option::Option<IDiaSymbol>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IDiaEnumSymbols>;
}
impl ::windows::core::RuntimeName for IDiaEnumSymbols {}
impl IDiaEnumSymbols_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumSymbols_Impl, const OFFSET: isize>() -> IDiaEnumSymbols_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumSymbols_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumSymbols_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumSymbols_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, symbol: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(symbol, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumSymbols_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumSymbols_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumSymbols_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumSymbols_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiaEnumSymbols as ::windows::core::Interface>::IID
    }
}
pub trait IDiaEnumSymbolsByAddr_Impl: Sized {
    fn symbolByAddr(&self, isect: u32, offset: u32) -> ::windows::core::Result<IDiaSymbol>;
    fn symbolByRVA(&self, relativevirtualaddress: u32) -> ::windows::core::Result<IDiaSymbol>;
    fn symbolByVA(&self, virtualaddress: u64) -> ::windows::core::Result<IDiaSymbol>;
    fn Next(&self, celt: u32, rgelt: *mut ::core::option::Option<IDiaSymbol>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Prev(&self, celt: u32, rgelt: *mut ::core::option::Option<IDiaSymbol>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IDiaEnumSymbolsByAddr>;
}
impl ::windows::core::RuntimeName for IDiaEnumSymbolsByAddr {}
impl IDiaEnumSymbolsByAddr_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumSymbolsByAddr_Impl, const OFFSET: isize>() -> IDiaEnumSymbolsByAddr_Vtbl {
        unsafe extern "system" fn symbolByAddr<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumSymbolsByAddr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isect: u32, offset: u32, ppsymbol: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.symbolByAddr(::core::mem::transmute_copy(&isect), ::core::mem::transmute_copy(&offset)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsymbol, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn symbolByRVA<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumSymbolsByAddr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relativevirtualaddress: u32, ppsymbol: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.symbolByRVA(::core::mem::transmute_copy(&relativevirtualaddress)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsymbol, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn symbolByVA<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumSymbolsByAddr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, virtualaddress: u64, ppsymbol: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.symbolByVA(::core::mem::transmute_copy(&virtualaddress)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsymbol, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumSymbolsByAddr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Prev<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumSymbolsByAddr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Prev(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumSymbolsByAddr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            symbolByAddr: symbolByAddr::<Identity, Impl, OFFSET>,
            symbolByRVA: symbolByRVA::<Identity, Impl, OFFSET>,
            symbolByVA: symbolByVA::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Prev: Prev::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiaEnumSymbolsByAddr as ::windows::core::Interface>::IID
    }
}
pub trait IDiaEnumSymbolsByAddr2_Impl: Sized + IDiaEnumSymbolsByAddr_Impl {
    fn symbolByAddrEx(&self, fpromoteblocksym: ::windows::Win32::Foundation::BOOL, isect: u32, offset: u32) -> ::windows::core::Result<IDiaSymbol>;
    fn symbolByRVAEx(&self, fpromoteblocksym: ::windows::Win32::Foundation::BOOL, relativevirtualaddress: u32) -> ::windows::core::Result<IDiaSymbol>;
    fn symbolByVAEx(&self, fpromoteblocksym: ::windows::Win32::Foundation::BOOL, virtualaddress: u64) -> ::windows::core::Result<IDiaSymbol>;
    fn NextEx(&self, fpromoteblocksym: ::windows::Win32::Foundation::BOOL, celt: u32, rgelt: *mut ::core::option::Option<IDiaSymbol>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn PrevEx(&self, fpromoteblocksym: ::windows::Win32::Foundation::BOOL, celt: u32, rgelt: *mut ::core::option::Option<IDiaSymbol>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDiaEnumSymbolsByAddr2 {}
impl IDiaEnumSymbolsByAddr2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumSymbolsByAddr2_Impl, const OFFSET: isize>() -> IDiaEnumSymbolsByAddr2_Vtbl {
        unsafe extern "system" fn symbolByAddrEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumSymbolsByAddr2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fpromoteblocksym: ::windows::Win32::Foundation::BOOL, isect: u32, offset: u32, ppsymbol: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.symbolByAddrEx(::core::mem::transmute_copy(&fpromoteblocksym), ::core::mem::transmute_copy(&isect), ::core::mem::transmute_copy(&offset)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsymbol, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn symbolByRVAEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumSymbolsByAddr2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fpromoteblocksym: ::windows::Win32::Foundation::BOOL, relativevirtualaddress: u32, ppsymbol: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.symbolByRVAEx(::core::mem::transmute_copy(&fpromoteblocksym), ::core::mem::transmute_copy(&relativevirtualaddress)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsymbol, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn symbolByVAEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumSymbolsByAddr2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fpromoteblocksym: ::windows::Win32::Foundation::BOOL, virtualaddress: u64, ppsymbol: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.symbolByVAEx(::core::mem::transmute_copy(&fpromoteblocksym), ::core::mem::transmute_copy(&virtualaddress)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsymbol, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NextEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumSymbolsByAddr2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fpromoteblocksym: ::windows::Win32::Foundation::BOOL, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NextEx(::core::mem::transmute_copy(&fpromoteblocksym), ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn PrevEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumSymbolsByAddr2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fpromoteblocksym: ::windows::Win32::Foundation::BOOL, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PrevEx(::core::mem::transmute_copy(&fpromoteblocksym), ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        Self {
            base__: IDiaEnumSymbolsByAddr_Vtbl::new::<Identity, Impl, OFFSET>(),
            symbolByAddrEx: symbolByAddrEx::<Identity, Impl, OFFSET>,
            symbolByRVAEx: symbolByRVAEx::<Identity, Impl, OFFSET>,
            symbolByVAEx: symbolByVAEx::<Identity, Impl, OFFSET>,
            NextEx: NextEx::<Identity, Impl, OFFSET>,
            PrevEx: PrevEx::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiaEnumSymbolsByAddr2 as ::windows::core::Interface>::IID || iid == &<IDiaEnumSymbolsByAddr as ::windows::core::Interface>::IID
    }
}
pub trait IDiaEnumTables_Impl: Sized {
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn Item(&self, index: &::windows::Win32::System::Com::VARIANT) -> ::windows::core::Result<IDiaTable>;
    fn Next(&self, celt: u32, rgelt: *mut ::core::option::Option<IDiaTable>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IDiaEnumTables>;
}
impl ::windows::core::RuntimeName for IDiaEnumTables {}
impl IDiaEnumTables_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumTables_Impl, const OFFSET: isize>() -> IDiaEnumTables_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumTables_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumTables_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumTables_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<::windows::Win32::System::Com::VARIANT>, table: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Item(::core::mem::transmute(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(table, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumTables_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumTables_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumTables_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaEnumTables_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiaEnumTables as ::windows::core::Interface>::IID
    }
}
pub trait IDiaFrameData_Impl: Sized {
    fn addressSection(&self) -> ::windows::core::Result<u32>;
    fn addressOffset(&self) -> ::windows::core::Result<u32>;
    fn relativeVirtualAddress(&self) -> ::windows::core::Result<u32>;
    fn virtualAddress(&self) -> ::windows::core::Result<u64>;
    fn lengthBlock(&self) -> ::windows::core::Result<u32>;
    fn lengthLocals(&self) -> ::windows::core::Result<u32>;
    fn lengthParams(&self) -> ::windows::core::Result<u32>;
    fn maxStack(&self) -> ::windows::core::Result<u32>;
    fn lengthProlog(&self) -> ::windows::core::Result<u32>;
    fn lengthSavedRegisters(&self) -> ::windows::core::Result<u32>;
    fn program(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn systemExceptionHandling(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn cplusplusExceptionHandling(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn functionStart(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn allocatesBasePointer(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn r#type(&self) -> ::windows::core::Result<u32>;
    fn functionParent(&self) -> ::windows::core::Result<IDiaFrameData>;
    fn execute(&self, frame: &::core::option::Option<IDiaStackWalkFrame>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDiaFrameData {}
impl IDiaFrameData_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaFrameData_Impl, const OFFSET: isize>() -> IDiaFrameData_Vtbl {
        unsafe extern "system" fn addressSection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaFrameData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.addressSection() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn addressOffset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaFrameData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.addressOffset() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn relativeVirtualAddress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaFrameData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.relativeVirtualAddress() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn virtualAddress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaFrameData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.virtualAddress() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn lengthBlock<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaFrameData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.lengthBlock() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn lengthLocals<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaFrameData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.lengthLocals() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn lengthParams<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaFrameData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.lengthParams() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn maxStack<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaFrameData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.maxStack() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn lengthProlog<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaFrameData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.lengthProlog() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn lengthSavedRegisters<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaFrameData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.lengthSavedRegisters() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn program<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaFrameData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.program() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn systemExceptionHandling<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaFrameData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.systemExceptionHandling() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn cplusplusExceptionHandling<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaFrameData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.cplusplusExceptionHandling() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn functionStart<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaFrameData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.functionStart() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn allocatesBasePointer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaFrameData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.allocatesBasePointer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn r#type<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaFrameData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.r#type() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn functionParent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaFrameData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.functionParent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn execute<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaFrameData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frame: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.execute(::core::mem::transmute(&frame)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            addressSection: addressSection::<Identity, Impl, OFFSET>,
            addressOffset: addressOffset::<Identity, Impl, OFFSET>,
            relativeVirtualAddress: relativeVirtualAddress::<Identity, Impl, OFFSET>,
            virtualAddress: virtualAddress::<Identity, Impl, OFFSET>,
            lengthBlock: lengthBlock::<Identity, Impl, OFFSET>,
            lengthLocals: lengthLocals::<Identity, Impl, OFFSET>,
            lengthParams: lengthParams::<Identity, Impl, OFFSET>,
            maxStack: maxStack::<Identity, Impl, OFFSET>,
            lengthProlog: lengthProlog::<Identity, Impl, OFFSET>,
            lengthSavedRegisters: lengthSavedRegisters::<Identity, Impl, OFFSET>,
            program: program::<Identity, Impl, OFFSET>,
            systemExceptionHandling: systemExceptionHandling::<Identity, Impl, OFFSET>,
            cplusplusExceptionHandling: cplusplusExceptionHandling::<Identity, Impl, OFFSET>,
            functionStart: functionStart::<Identity, Impl, OFFSET>,
            allocatesBasePointer: allocatesBasePointer::<Identity, Impl, OFFSET>,
            r#type: r#type::<Identity, Impl, OFFSET>,
            functionParent: functionParent::<Identity, Impl, OFFSET>,
            execute: execute::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiaFrameData as ::windows::core::Interface>::IID
    }
}
pub trait IDiaImageData_Impl: Sized {
    fn relativeVirtualAddress(&self) -> ::windows::core::Result<u32>;
    fn virtualAddress(&self) -> ::windows::core::Result<u64>;
    fn imageBase(&self) -> ::windows::core::Result<u64>;
}
impl ::windows::core::RuntimeName for IDiaImageData {}
impl IDiaImageData_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaImageData_Impl, const OFFSET: isize>() -> IDiaImageData_Vtbl {
        unsafe extern "system" fn relativeVirtualAddress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaImageData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.relativeVirtualAddress() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn virtualAddress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaImageData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.virtualAddress() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn imageBase<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaImageData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.imageBase() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            relativeVirtualAddress: relativeVirtualAddress::<Identity, Impl, OFFSET>,
            virtualAddress: virtualAddress::<Identity, Impl, OFFSET>,
            imageBase: imageBase::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiaImageData as ::windows::core::Interface>::IID
    }
}
pub trait IDiaInjectedSource_Impl: Sized {
    fn crc(&self) -> ::windows::core::Result<u32>;
    fn length(&self) -> ::windows::core::Result<u64>;
    fn filename(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn objectFilename(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn virtualFilename(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn sourceCompression(&self) -> ::windows::core::Result<u32>;
    fn get_source(&self, cbdata: u32, pcbdata: *mut u32, pbdata: *mut u8) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDiaInjectedSource {}
impl IDiaInjectedSource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaInjectedSource_Impl, const OFFSET: isize>() -> IDiaInjectedSource_Vtbl {
        unsafe extern "system" fn crc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaInjectedSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.crc() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn length<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaInjectedSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.length() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn filename<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaInjectedSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.filename() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn objectFilename<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaInjectedSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.objectFilename() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn virtualFilename<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaInjectedSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.virtualFilename() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn sourceCompression<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaInjectedSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.sourceCompression() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_source<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaInjectedSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbdata: u32, pcbdata: *mut u32, pbdata: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_source(::core::mem::transmute_copy(&cbdata), ::core::mem::transmute_copy(&pcbdata), ::core::mem::transmute_copy(&pbdata)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            crc: crc::<Identity, Impl, OFFSET>,
            length: length::<Identity, Impl, OFFSET>,
            filename: filename::<Identity, Impl, OFFSET>,
            objectFilename: objectFilename::<Identity, Impl, OFFSET>,
            virtualFilename: virtualFilename::<Identity, Impl, OFFSET>,
            sourceCompression: sourceCompression::<Identity, Impl, OFFSET>,
            get_source: get_source::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiaInjectedSource as ::windows::core::Interface>::IID
    }
}
pub trait IDiaInputAssemblyFile_Impl: Sized {
    fn uniqueId(&self) -> ::windows::core::Result<u32>;
    fn index(&self) -> ::windows::core::Result<u32>;
    fn timestamp(&self) -> ::windows::core::Result<u32>;
    fn pdbAvailableAtILMerge(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn fileName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn get_version(&self, cbdata: u32, pcbdata: *mut u32, pbdata: *mut u8) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDiaInputAssemblyFile {}
impl IDiaInputAssemblyFile_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaInputAssemblyFile_Impl, const OFFSET: isize>() -> IDiaInputAssemblyFile_Vtbl {
        unsafe extern "system" fn uniqueId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaInputAssemblyFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.uniqueId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn index<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaInputAssemblyFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.index() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn timestamp<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaInputAssemblyFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.timestamp() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn pdbAvailableAtILMerge<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaInputAssemblyFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.pdbAvailableAtILMerge() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fileName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaInputAssemblyFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.fileName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_version<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaInputAssemblyFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbdata: u32, pcbdata: *mut u32, pbdata: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_version(::core::mem::transmute_copy(&cbdata), ::core::mem::transmute_copy(&pcbdata), ::core::mem::transmute_copy(&pbdata)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            uniqueId: uniqueId::<Identity, Impl, OFFSET>,
            index: index::<Identity, Impl, OFFSET>,
            timestamp: timestamp::<Identity, Impl, OFFSET>,
            pdbAvailableAtILMerge: pdbAvailableAtILMerge::<Identity, Impl, OFFSET>,
            fileName: fileName::<Identity, Impl, OFFSET>,
            get_version: get_version::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiaInputAssemblyFile as ::windows::core::Interface>::IID
    }
}
pub trait IDiaLineNumber_Impl: Sized {
    fn compiland(&self) -> ::windows::core::Result<IDiaSymbol>;
    fn sourceFile(&self) -> ::windows::core::Result<IDiaSourceFile>;
    fn lineNumber(&self) -> ::windows::core::Result<u32>;
    fn lineNumberEnd(&self) -> ::windows::core::Result<u32>;
    fn columnNumber(&self) -> ::windows::core::Result<u32>;
    fn columnNumberEnd(&self) -> ::windows::core::Result<u32>;
    fn addressSection(&self) -> ::windows::core::Result<u32>;
    fn addressOffset(&self) -> ::windows::core::Result<u32>;
    fn relativeVirtualAddress(&self) -> ::windows::core::Result<u32>;
    fn virtualAddress(&self) -> ::windows::core::Result<u64>;
    fn length(&self) -> ::windows::core::Result<u32>;
    fn sourceFileId(&self) -> ::windows::core::Result<u32>;
    fn statement(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn compilandId(&self) -> ::windows::core::Result<u32>;
}
impl ::windows::core::RuntimeName for IDiaLineNumber {}
impl IDiaLineNumber_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaLineNumber_Impl, const OFFSET: isize>() -> IDiaLineNumber_Vtbl {
        unsafe extern "system" fn compiland<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaLineNumber_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.compiland() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn sourceFile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaLineNumber_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.sourceFile() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn lineNumber<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaLineNumber_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.lineNumber() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn lineNumberEnd<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaLineNumber_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.lineNumberEnd() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn columnNumber<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaLineNumber_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.columnNumber() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn columnNumberEnd<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaLineNumber_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.columnNumberEnd() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn addressSection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaLineNumber_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.addressSection() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn addressOffset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaLineNumber_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.addressOffset() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn relativeVirtualAddress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaLineNumber_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.relativeVirtualAddress() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn virtualAddress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaLineNumber_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.virtualAddress() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn length<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaLineNumber_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.length() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn sourceFileId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaLineNumber_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.sourceFileId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn statement<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaLineNumber_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.statement() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn compilandId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaLineNumber_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.compilandId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            compiland: compiland::<Identity, Impl, OFFSET>,
            sourceFile: sourceFile::<Identity, Impl, OFFSET>,
            lineNumber: lineNumber::<Identity, Impl, OFFSET>,
            lineNumberEnd: lineNumberEnd::<Identity, Impl, OFFSET>,
            columnNumber: columnNumber::<Identity, Impl, OFFSET>,
            columnNumberEnd: columnNumberEnd::<Identity, Impl, OFFSET>,
            addressSection: addressSection::<Identity, Impl, OFFSET>,
            addressOffset: addressOffset::<Identity, Impl, OFFSET>,
            relativeVirtualAddress: relativeVirtualAddress::<Identity, Impl, OFFSET>,
            virtualAddress: virtualAddress::<Identity, Impl, OFFSET>,
            length: length::<Identity, Impl, OFFSET>,
            sourceFileId: sourceFileId::<Identity, Impl, OFFSET>,
            statement: statement::<Identity, Impl, OFFSET>,
            compilandId: compilandId::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiaLineNumber as ::windows::core::Interface>::IID
    }
}
pub trait IDiaLoadCallback_Impl: Sized {
    fn NotifyDebugDir(&self, fexecutable: ::windows::Win32::Foundation::BOOL, cbdata: u32, pbdata: *const u8) -> ::windows::core::Result<()>;
    fn NotifyOpenDBG(&self, dbgpath: &::windows::core::PCWSTR, resultcode: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn NotifyOpenPDB(&self, pdbpath: &::windows::core::PCWSTR, resultcode: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn RestrictRegistryAccess(&self) -> ::windows::core::Result<()>;
    fn RestrictSymbolServerAccess(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDiaLoadCallback {}
impl IDiaLoadCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaLoadCallback_Impl, const OFFSET: isize>() -> IDiaLoadCallback_Vtbl {
        unsafe extern "system" fn NotifyDebugDir<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaLoadCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fexecutable: ::windows::Win32::Foundation::BOOL, cbdata: u32, pbdata: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NotifyDebugDir(::core::mem::transmute_copy(&fexecutable), ::core::mem::transmute_copy(&cbdata), ::core::mem::transmute_copy(&pbdata)).into()
        }
        unsafe extern "system" fn NotifyOpenDBG<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaLoadCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dbgpath: ::windows::core::PCWSTR, resultcode: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NotifyOpenDBG(::core::mem::transmute(&dbgpath), ::core::mem::transmute_copy(&resultcode)).into()
        }
        unsafe extern "system" fn NotifyOpenPDB<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaLoadCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdbpath: ::windows::core::PCWSTR, resultcode: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NotifyOpenPDB(::core::mem::transmute(&pdbpath), ::core::mem::transmute_copy(&resultcode)).into()
        }
        unsafe extern "system" fn RestrictRegistryAccess<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaLoadCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RestrictRegistryAccess().into()
        }
        unsafe extern "system" fn RestrictSymbolServerAccess<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaLoadCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RestrictSymbolServerAccess().into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            NotifyDebugDir: NotifyDebugDir::<Identity, Impl, OFFSET>,
            NotifyOpenDBG: NotifyOpenDBG::<Identity, Impl, OFFSET>,
            NotifyOpenPDB: NotifyOpenPDB::<Identity, Impl, OFFSET>,
            RestrictRegistryAccess: RestrictRegistryAccess::<Identity, Impl, OFFSET>,
            RestrictSymbolServerAccess: RestrictSymbolServerAccess::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiaLoadCallback as ::windows::core::Interface>::IID
    }
}
pub trait IDiaLoadCallback2_Impl: Sized + IDiaLoadCallback_Impl {
    fn RestrictOriginalPathAccess(&self) -> ::windows::core::Result<()>;
    fn RestrictReferencePathAccess(&self) -> ::windows::core::Result<()>;
    fn RestrictDBGAccess(&self) -> ::windows::core::Result<()>;
    fn RestrictSystemRootAccess(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDiaLoadCallback2 {}
impl IDiaLoadCallback2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaLoadCallback2_Impl, const OFFSET: isize>() -> IDiaLoadCallback2_Vtbl {
        unsafe extern "system" fn RestrictOriginalPathAccess<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaLoadCallback2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RestrictOriginalPathAccess().into()
        }
        unsafe extern "system" fn RestrictReferencePathAccess<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaLoadCallback2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RestrictReferencePathAccess().into()
        }
        unsafe extern "system" fn RestrictDBGAccess<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaLoadCallback2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RestrictDBGAccess().into()
        }
        unsafe extern "system" fn RestrictSystemRootAccess<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaLoadCallback2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RestrictSystemRootAccess().into()
        }
        Self {
            base__: IDiaLoadCallback_Vtbl::new::<Identity, Impl, OFFSET>(),
            RestrictOriginalPathAccess: RestrictOriginalPathAccess::<Identity, Impl, OFFSET>,
            RestrictReferencePathAccess: RestrictReferencePathAccess::<Identity, Impl, OFFSET>,
            RestrictDBGAccess: RestrictDBGAccess::<Identity, Impl, OFFSET>,
            RestrictSystemRootAccess: RestrictSystemRootAccess::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiaLoadCallback2 as ::windows::core::Interface>::IID || iid == &<IDiaLoadCallback as ::windows::core::Interface>::IID
    }
}
pub trait IDiaPropertyStorage_Impl: Sized {
    fn ReadMultiple(&self, cpspec: u32, rgpspec: *const ::windows::Win32::System::Com::StructuredStorage::PROPSPEC) -> ::windows::core::Result<::windows::Win32::System::Com::StructuredStorage::PROPVARIANT>;
    fn ReadPropertyNames(&self, cpropid: u32, rgpropid: *const u32, rglpwstrname: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Enum(&self) -> ::windows::core::Result<::windows::Win32::System::Com::StructuredStorage::IEnumSTATPROPSTG>;
    fn ReadDWORD(&self, id: u32) -> ::windows::core::Result<u32>;
    fn ReadLONG(&self, id: u32) -> ::windows::core::Result<i32>;
    fn ReadBOOL(&self, id: u32) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn ReadULONGLONG(&self, id: u32) -> ::windows::core::Result<u64>;
    fn ReadBSTR(&self, id: u32) -> ::windows::core::Result<::windows::core::BSTR>;
}
impl ::windows::core::RuntimeName for IDiaPropertyStorage {}
impl IDiaPropertyStorage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaPropertyStorage_Impl, const OFFSET: isize>() -> IDiaPropertyStorage_Vtbl {
        unsafe extern "system" fn ReadMultiple<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaPropertyStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpspec: u32, rgpspec: *const ::windows::Win32::System::Com::StructuredStorage::PROPSPEC, rgvar: *mut ::windows::Win32::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReadMultiple(::core::mem::transmute_copy(&cpspec), ::core::mem::transmute_copy(&rgpspec)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(rgvar, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadPropertyNames<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaPropertyStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpropid: u32, rgpropid: *const u32, rglpwstrname: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReadPropertyNames(::core::mem::transmute_copy(&cpropid), ::core::mem::transmute_copy(&rgpropid), ::core::mem::transmute_copy(&rglpwstrname)).into()
        }
        unsafe extern "system" fn Enum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaPropertyStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Enum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadDWORD<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaPropertyStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: u32, pvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReadDWORD(::core::mem::transmute_copy(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadLONG<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaPropertyStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: u32, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReadLONG(::core::mem::transmute_copy(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadBOOL<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaPropertyStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: u32, pvalue: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReadBOOL(::core::mem::transmute_copy(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadULONGLONG<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaPropertyStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: u32, pvalue: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReadULONGLONG(::core::mem::transmute_copy(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadBSTR<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaPropertyStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: u32, pvalue: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReadBSTR(::core::mem::transmute_copy(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ReadMultiple: ReadMultiple::<Identity, Impl, OFFSET>,
            ReadPropertyNames: ReadPropertyNames::<Identity, Impl, OFFSET>,
            Enum: Enum::<Identity, Impl, OFFSET>,
            ReadDWORD: ReadDWORD::<Identity, Impl, OFFSET>,
            ReadLONG: ReadLONG::<Identity, Impl, OFFSET>,
            ReadBOOL: ReadBOOL::<Identity, Impl, OFFSET>,
            ReadULONGLONG: ReadULONGLONG::<Identity, Impl, OFFSET>,
            ReadBSTR: ReadBSTR::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiaPropertyStorage as ::windows::core::Interface>::IID
    }
}
pub trait IDiaReadExeAtOffsetCallback_Impl: Sized {
    fn ReadExecutableAt(&self, fileoffset: u64, cbdata: u32, pcbdata: *mut u32, pbdata: *mut u8) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDiaReadExeAtOffsetCallback {}
impl IDiaReadExeAtOffsetCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaReadExeAtOffsetCallback_Impl, const OFFSET: isize>() -> IDiaReadExeAtOffsetCallback_Vtbl {
        unsafe extern "system" fn ReadExecutableAt<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaReadExeAtOffsetCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fileoffset: u64, cbdata: u32, pcbdata: *mut u32, pbdata: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReadExecutableAt(::core::mem::transmute_copy(&fileoffset), ::core::mem::transmute_copy(&cbdata), ::core::mem::transmute_copy(&pcbdata), ::core::mem::transmute_copy(&pbdata)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ReadExecutableAt: ReadExecutableAt::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiaReadExeAtOffsetCallback as ::windows::core::Interface>::IID
    }
}
pub trait IDiaReadExeAtRVACallback_Impl: Sized {
    fn ReadExecutableAtRVA(&self, relativevirtualaddress: u32, cbdata: u32, pcbdata: *mut u32, pbdata: *mut u8) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDiaReadExeAtRVACallback {}
impl IDiaReadExeAtRVACallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaReadExeAtRVACallback_Impl, const OFFSET: isize>() -> IDiaReadExeAtRVACallback_Vtbl {
        unsafe extern "system" fn ReadExecutableAtRVA<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaReadExeAtRVACallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relativevirtualaddress: u32, cbdata: u32, pcbdata: *mut u32, pbdata: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReadExecutableAtRVA(::core::mem::transmute_copy(&relativevirtualaddress), ::core::mem::transmute_copy(&cbdata), ::core::mem::transmute_copy(&pcbdata), ::core::mem::transmute_copy(&pbdata)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ReadExecutableAtRVA: ReadExecutableAtRVA::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiaReadExeAtRVACallback as ::windows::core::Interface>::IID
    }
}
pub trait IDiaSectionContrib_Impl: Sized {
    fn compiland(&self) -> ::windows::core::Result<IDiaSymbol>;
    fn addressSection(&self) -> ::windows::core::Result<u32>;
    fn addressOffset(&self) -> ::windows::core::Result<u32>;
    fn relativeVirtualAddress(&self) -> ::windows::core::Result<u32>;
    fn virtualAddress(&self) -> ::windows::core::Result<u64>;
    fn length(&self) -> ::windows::core::Result<u32>;
    fn notPaged(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn code(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn initializedData(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn uninitializedData(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn remove(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn comdat(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn discardable(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn notCached(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn share(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn execute(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn read(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn write(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn dataCrc(&self) -> ::windows::core::Result<u32>;
    fn relocationsCrc(&self) -> ::windows::core::Result<u32>;
    fn compilandId(&self) -> ::windows::core::Result<u32>;
    fn code16bit(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
}
impl ::windows::core::RuntimeName for IDiaSectionContrib {}
impl IDiaSectionContrib_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSectionContrib_Impl, const OFFSET: isize>() -> IDiaSectionContrib_Vtbl {
        unsafe extern "system" fn compiland<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSectionContrib_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.compiland() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn addressSection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSectionContrib_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.addressSection() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn addressOffset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSectionContrib_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.addressOffset() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn relativeVirtualAddress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSectionContrib_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.relativeVirtualAddress() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn virtualAddress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSectionContrib_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.virtualAddress() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn length<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSectionContrib_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.length() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn notPaged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSectionContrib_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.notPaged() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn code<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSectionContrib_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.code() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn initializedData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSectionContrib_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.initializedData() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn uninitializedData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSectionContrib_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.uninitializedData() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn remove<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSectionContrib_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.remove() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn comdat<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSectionContrib_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.comdat() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn discardable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSectionContrib_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.discardable() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn notCached<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSectionContrib_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.notCached() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn share<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSectionContrib_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.share() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn execute<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSectionContrib_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.execute() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn read<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSectionContrib_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.read() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn write<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSectionContrib_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.write() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn dataCrc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSectionContrib_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.dataCrc() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn relocationsCrc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSectionContrib_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.relocationsCrc() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn compilandId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSectionContrib_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.compilandId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn code16bit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSectionContrib_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.code16bit() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            compiland: compiland::<Identity, Impl, OFFSET>,
            addressSection: addressSection::<Identity, Impl, OFFSET>,
            addressOffset: addressOffset::<Identity, Impl, OFFSET>,
            relativeVirtualAddress: relativeVirtualAddress::<Identity, Impl, OFFSET>,
            virtualAddress: virtualAddress::<Identity, Impl, OFFSET>,
            length: length::<Identity, Impl, OFFSET>,
            notPaged: notPaged::<Identity, Impl, OFFSET>,
            code: code::<Identity, Impl, OFFSET>,
            initializedData: initializedData::<Identity, Impl, OFFSET>,
            uninitializedData: uninitializedData::<Identity, Impl, OFFSET>,
            remove: remove::<Identity, Impl, OFFSET>,
            comdat: comdat::<Identity, Impl, OFFSET>,
            discardable: discardable::<Identity, Impl, OFFSET>,
            notCached: notCached::<Identity, Impl, OFFSET>,
            share: share::<Identity, Impl, OFFSET>,
            execute: execute::<Identity, Impl, OFFSET>,
            read: read::<Identity, Impl, OFFSET>,
            write: write::<Identity, Impl, OFFSET>,
            dataCrc: dataCrc::<Identity, Impl, OFFSET>,
            relocationsCrc: relocationsCrc::<Identity, Impl, OFFSET>,
            compilandId: compilandId::<Identity, Impl, OFFSET>,
            code16bit: code16bit::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiaSectionContrib as ::windows::core::Interface>::IID
    }
}
pub trait IDiaSegment_Impl: Sized {
    fn frame(&self) -> ::windows::core::Result<u32>;
    fn offset(&self) -> ::windows::core::Result<u32>;
    fn length(&self) -> ::windows::core::Result<u32>;
    fn read(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn write(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn execute(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn addressSection(&self) -> ::windows::core::Result<u32>;
    fn relativeVirtualAddress(&self) -> ::windows::core::Result<u32>;
    fn virtualAddress(&self) -> ::windows::core::Result<u64>;
}
impl ::windows::core::RuntimeName for IDiaSegment {}
impl IDiaSegment_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSegment_Impl, const OFFSET: isize>() -> IDiaSegment_Vtbl {
        unsafe extern "system" fn frame<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSegment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.frame() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn offset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSegment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.offset() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn length<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSegment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.length() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn read<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSegment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.read() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn write<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSegment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.write() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn execute<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSegment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.execute() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn addressSection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSegment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.addressSection() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn relativeVirtualAddress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSegment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.relativeVirtualAddress() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn virtualAddress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSegment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.virtualAddress() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            frame: frame::<Identity, Impl, OFFSET>,
            offset: offset::<Identity, Impl, OFFSET>,
            length: length::<Identity, Impl, OFFSET>,
            read: read::<Identity, Impl, OFFSET>,
            write: write::<Identity, Impl, OFFSET>,
            execute: execute::<Identity, Impl, OFFSET>,
            addressSection: addressSection::<Identity, Impl, OFFSET>,
            relativeVirtualAddress: relativeVirtualAddress::<Identity, Impl, OFFSET>,
            virtualAddress: virtualAddress::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiaSegment as ::windows::core::Interface>::IID
    }
}
pub trait IDiaSession_Impl: Sized {
    fn loadAddress(&self) -> ::windows::core::Result<u64>;
    fn SetloadAddress(&self, newval: u64) -> ::windows::core::Result<()>;
    fn globalScope(&self) -> ::windows::core::Result<IDiaSymbol>;
    fn getEnumTables(&self) -> ::windows::core::Result<IDiaEnumTables>;
    fn getSymbolsByAddr(&self) -> ::windows::core::Result<IDiaEnumSymbolsByAddr>;
    fn findChildren(&self, parent: &::core::option::Option<IDiaSymbol>, symtag: SymTag, name: &::windows::core::PCWSTR, compareflags: u32) -> ::windows::core::Result<IDiaEnumSymbols>;
    fn findChildrenEx(&self, parent: &::core::option::Option<IDiaSymbol>, symtag: SymTag, name: &::windows::core::PCWSTR, compareflags: u32) -> ::windows::core::Result<IDiaEnumSymbols>;
    fn findChildrenExByAddr(&self, parent: &::core::option::Option<IDiaSymbol>, symtag: SymTag, name: &::windows::core::PCWSTR, compareflags: u32, isect: u32, offset: u32) -> ::windows::core::Result<IDiaEnumSymbols>;
    fn findChildrenExByVA(&self, parent: &::core::option::Option<IDiaSymbol>, symtag: SymTag, name: &::windows::core::PCWSTR, compareflags: u32, va: u64) -> ::windows::core::Result<IDiaEnumSymbols>;
    fn findChildrenExByRVA(&self, parent: &::core::option::Option<IDiaSymbol>, symtag: SymTag, name: &::windows::core::PCWSTR, compareflags: u32, rva: u32) -> ::windows::core::Result<IDiaEnumSymbols>;
    fn findSymbolByAddr(&self, isect: u32, offset: u32, symtag: SymTag) -> ::windows::core::Result<IDiaSymbol>;
    fn findSymbolByRVA(&self, rva: u32, symtag: SymTag) -> ::windows::core::Result<IDiaSymbol>;
    fn findSymbolByVA(&self, va: u64, symtag: SymTag) -> ::windows::core::Result<IDiaSymbol>;
    fn findSymbolByToken(&self, token: u32, symtag: SymTag) -> ::windows::core::Result<IDiaSymbol>;
    fn symsAreEquiv(&self, symbola: &::core::option::Option<IDiaSymbol>, symbolb: &::core::option::Option<IDiaSymbol>) -> ::windows::core::Result<()>;
    fn symbolById(&self, id: u32) -> ::windows::core::Result<IDiaSymbol>;
    fn findSymbolByRVAEx(&self, rva: u32, symtag: SymTag, ppsymbol: *mut ::core::option::Option<IDiaSymbol>, displacement: *mut i32) -> ::windows::core::Result<()>;
    fn findSymbolByVAEx(&self, va: u64, symtag: SymTag, ppsymbol: *mut ::core::option::Option<IDiaSymbol>, displacement: *mut i32) -> ::windows::core::Result<()>;
    fn findFile(&self, pcompiland: &::core::option::Option<IDiaSymbol>, name: &::windows::core::PCWSTR, compareflags: u32) -> ::windows::core::Result<IDiaEnumSourceFiles>;
    fn findFileById(&self, uniqueid: u32) -> ::windows::core::Result<IDiaSourceFile>;
    fn findLines(&self, compiland: &::core::option::Option<IDiaSymbol>, file: &::core::option::Option<IDiaSourceFile>) -> ::windows::core::Result<IDiaEnumLineNumbers>;
    fn findLinesByAddr(&self, seg: u32, offset: u32, length: u32) -> ::windows::core::Result<IDiaEnumLineNumbers>;
    fn findLinesByRVA(&self, rva: u32, length: u32) -> ::windows::core::Result<IDiaEnumLineNumbers>;
    fn findLinesByVA(&self, va: u64, length: u32) -> ::windows::core::Result<IDiaEnumLineNumbers>;
    fn findLinesByLinenum(&self, compiland: &::core::option::Option<IDiaSymbol>, file: &::core::option::Option<IDiaSourceFile>, linenum: u32, column: u32) -> ::windows::core::Result<IDiaEnumLineNumbers>;
    fn findInjectedSource(&self, srcfile: &::windows::core::PCWSTR) -> ::windows::core::Result<IDiaEnumInjectedSources>;
    fn getEnumDebugStreams(&self) -> ::windows::core::Result<IDiaEnumDebugStreams>;
    fn findInlineFramesByAddr(&self, parent: &::core::option::Option<IDiaSymbol>, isect: u32, offset: u32) -> ::windows::core::Result<IDiaEnumSymbols>;
    fn findInlineFramesByRVA(&self, parent: &::core::option::Option<IDiaSymbol>, rva: u32) -> ::windows::core::Result<IDiaEnumSymbols>;
    fn findInlineFramesByVA(&self, parent: &::core::option::Option<IDiaSymbol>, va: u64) -> ::windows::core::Result<IDiaEnumSymbols>;
    fn findInlineeLines(&self, parent: &::core::option::Option<IDiaSymbol>) -> ::windows::core::Result<IDiaEnumLineNumbers>;
    fn findInlineeLinesByAddr(&self, parent: &::core::option::Option<IDiaSymbol>, isect: u32, offset: u32, length: u32) -> ::windows::core::Result<IDiaEnumLineNumbers>;
    fn findInlineeLinesByRVA(&self, parent: &::core::option::Option<IDiaSymbol>, rva: u32, length: u32) -> ::windows::core::Result<IDiaEnumLineNumbers>;
    fn findInlineeLinesByVA(&self, parent: &::core::option::Option<IDiaSymbol>, va: u64, length: u32) -> ::windows::core::Result<IDiaEnumLineNumbers>;
    fn findInlineeLinesByLinenum(&self, compiland: &::core::option::Option<IDiaSymbol>, file: &::core::option::Option<IDiaSourceFile>, linenum: u32, column: u32) -> ::windows::core::Result<IDiaEnumLineNumbers>;
    fn findInlineesByName(&self, name: &::windows::core::PCWSTR, option: u32) -> ::windows::core::Result<IDiaEnumSymbols>;
    fn findAcceleratorInlineeLinesByLinenum(&self, parent: &::core::option::Option<IDiaSymbol>, file: &::core::option::Option<IDiaSourceFile>, linenum: u32, column: u32) -> ::windows::core::Result<IDiaEnumLineNumbers>;
    fn findSymbolsForAcceleratorPointerTag(&self, parent: &::core::option::Option<IDiaSymbol>, tagvalue: u32) -> ::windows::core::Result<IDiaEnumSymbols>;
    fn findSymbolsByRVAForAcceleratorPointerTag(&self, parent: &::core::option::Option<IDiaSymbol>, tagvalue: u32, rva: u32) -> ::windows::core::Result<IDiaEnumSymbols>;
    fn findAcceleratorInlineesByName(&self, name: &::windows::core::PCWSTR, option: u32) -> ::windows::core::Result<IDiaEnumSymbols>;
    fn addressForVA(&self, va: u64, pisect: *mut u32, poffset: *mut u32) -> ::windows::core::Result<()>;
    fn addressForRVA(&self, rva: u32, pisect: *mut u32, poffset: *mut u32) -> ::windows::core::Result<()>;
    fn findILOffsetsByAddr(&self, isect: u32, offset: u32, length: u32) -> ::windows::core::Result<IDiaEnumLineNumbers>;
    fn findILOffsetsByRVA(&self, rva: u32, length: u32) -> ::windows::core::Result<IDiaEnumLineNumbers>;
    fn findILOffsetsByVA(&self, va: u64, length: u32) -> ::windows::core::Result<IDiaEnumLineNumbers>;
    fn findInputAssemblyFiles(&self) -> ::windows::core::Result<IDiaEnumInputAssemblyFiles>;
    fn findInputAssembly(&self, index: u32) -> ::windows::core::Result<IDiaInputAssemblyFile>;
    fn findInputAssemblyById(&self, uniqueid: u32) -> ::windows::core::Result<IDiaInputAssemblyFile>;
    fn getFuncMDTokenMapSize(&self) -> ::windows::core::Result<u32>;
    fn getFuncMDTokenMap(&self, cb: u32, pcb: *mut u32, pb: *mut u8) -> ::windows::core::Result<()>;
    fn getTypeMDTokenMapSize(&self) -> ::windows::core::Result<u32>;
    fn getTypeMDTokenMap(&self, cb: u32, pcb: *mut u32, pb: *mut u8) -> ::windows::core::Result<()>;
    fn getNumberOfFunctionFragments_VA(&self, vafunc: u64, cbfunc: u32) -> ::windows::core::Result<u32>;
    fn getNumberOfFunctionFragments_RVA(&self, rvafunc: u32, cbfunc: u32) -> ::windows::core::Result<u32>;
    fn getFunctionFragments_VA(&self, vafunc: u64, cbfunc: u32, cfragments: u32, pvafragment: *mut u64, plenfragment: *mut u32) -> ::windows::core::Result<()>;
    fn getFunctionFragments_RVA(&self, rvafunc: u32, cbfunc: u32, cfragments: u32, prvafragment: *mut u32, plenfragment: *mut u32) -> ::windows::core::Result<()>;
    fn getExports(&self) -> ::windows::core::Result<IDiaEnumSymbols>;
    fn getHeapAllocationSites(&self) -> ::windows::core::Result<IDiaEnumSymbols>;
    fn findInputAssemblyFile(&self, psymbol: &::core::option::Option<IDiaSymbol>) -> ::windows::core::Result<IDiaInputAssemblyFile>;
}
impl ::windows::core::RuntimeName for IDiaSession {}
impl IDiaSession_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSession_Impl, const OFFSET: isize>() -> IDiaSession_Vtbl {
        unsafe extern "system" fn loadAddress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.loadAddress() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetloadAddress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetloadAddress(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn globalScope<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.globalScope() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getEnumTables<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumtables: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getEnumTables() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumtables, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getSymbolsByAddr<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumbyaddr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getSymbolsByAddr() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumbyaddr, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn findChildren<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parent: *mut ::core::ffi::c_void, symtag: SymTag, name: ::windows::core::PCWSTR, compareflags: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.findChildren(::core::mem::transmute(&parent), ::core::mem::transmute_copy(&symtag), ::core::mem::transmute(&name), ::core::mem::transmute_copy(&compareflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn findChildrenEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parent: *mut ::core::ffi::c_void, symtag: SymTag, name: ::windows::core::PCWSTR, compareflags: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.findChildrenEx(::core::mem::transmute(&parent), ::core::mem::transmute_copy(&symtag), ::core::mem::transmute(&name), ::core::mem::transmute_copy(&compareflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn findChildrenExByAddr<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parent: *mut ::core::ffi::c_void, symtag: SymTag, name: ::windows::core::PCWSTR, compareflags: u32, isect: u32, offset: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.findChildrenExByAddr(::core::mem::transmute(&parent), ::core::mem::transmute_copy(&symtag), ::core::mem::transmute(&name), ::core::mem::transmute_copy(&compareflags), ::core::mem::transmute_copy(&isect), ::core::mem::transmute_copy(&offset)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn findChildrenExByVA<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parent: *mut ::core::ffi::c_void, symtag: SymTag, name: ::windows::core::PCWSTR, compareflags: u32, va: u64, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.findChildrenExByVA(::core::mem::transmute(&parent), ::core::mem::transmute_copy(&symtag), ::core::mem::transmute(&name), ::core::mem::transmute_copy(&compareflags), ::core::mem::transmute_copy(&va)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn findChildrenExByRVA<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parent: *mut ::core::ffi::c_void, symtag: SymTag, name: ::windows::core::PCWSTR, compareflags: u32, rva: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.findChildrenExByRVA(::core::mem::transmute(&parent), ::core::mem::transmute_copy(&symtag), ::core::mem::transmute(&name), ::core::mem::transmute_copy(&compareflags), ::core::mem::transmute_copy(&rva)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn findSymbolByAddr<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isect: u32, offset: u32, symtag: SymTag, ppsymbol: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.findSymbolByAddr(::core::mem::transmute_copy(&isect), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&symtag)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsymbol, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn findSymbolByRVA<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rva: u32, symtag: SymTag, ppsymbol: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.findSymbolByRVA(::core::mem::transmute_copy(&rva), ::core::mem::transmute_copy(&symtag)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsymbol, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn findSymbolByVA<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, va: u64, symtag: SymTag, ppsymbol: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.findSymbolByVA(::core::mem::transmute_copy(&va), ::core::mem::transmute_copy(&symtag)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsymbol, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn findSymbolByToken<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: u32, symtag: SymTag, ppsymbol: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.findSymbolByToken(::core::mem::transmute_copy(&token), ::core::mem::transmute_copy(&symtag)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsymbol, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn symsAreEquiv<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, symbola: *mut ::core::ffi::c_void, symbolb: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.symsAreEquiv(::core::mem::transmute(&symbola), ::core::mem::transmute(&symbolb)).into()
        }
        unsafe extern "system" fn symbolById<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: u32, ppsymbol: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.symbolById(::core::mem::transmute_copy(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsymbol, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn findSymbolByRVAEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rva: u32, symtag: SymTag, ppsymbol: *mut *mut ::core::ffi::c_void, displacement: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.findSymbolByRVAEx(::core::mem::transmute_copy(&rva), ::core::mem::transmute_copy(&symtag), ::core::mem::transmute_copy(&ppsymbol), ::core::mem::transmute_copy(&displacement)).into()
        }
        unsafe extern "system" fn findSymbolByVAEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, va: u64, symtag: SymTag, ppsymbol: *mut *mut ::core::ffi::c_void, displacement: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.findSymbolByVAEx(::core::mem::transmute_copy(&va), ::core::mem::transmute_copy(&symtag), ::core::mem::transmute_copy(&ppsymbol), ::core::mem::transmute_copy(&displacement)).into()
        }
        unsafe extern "system" fn findFile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcompiland: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR, compareflags: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.findFile(::core::mem::transmute(&pcompiland), ::core::mem::transmute(&name), ::core::mem::transmute_copy(&compareflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn findFileById<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uniqueid: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.findFileById(::core::mem::transmute_copy(&uniqueid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn findLines<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, compiland: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.findLines(::core::mem::transmute(&compiland), ::core::mem::transmute(&file)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn findLinesByAddr<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, seg: u32, offset: u32, length: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.findLinesByAddr(::core::mem::transmute_copy(&seg), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&length)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn findLinesByRVA<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rva: u32, length: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.findLinesByRVA(::core::mem::transmute_copy(&rva), ::core::mem::transmute_copy(&length)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn findLinesByVA<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, va: u64, length: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.findLinesByVA(::core::mem::transmute_copy(&va), ::core::mem::transmute_copy(&length)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn findLinesByLinenum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, compiland: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, linenum: u32, column: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.findLinesByLinenum(::core::mem::transmute(&compiland), ::core::mem::transmute(&file), ::core::mem::transmute_copy(&linenum), ::core::mem::transmute_copy(&column)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn findInjectedSource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, srcfile: ::windows::core::PCWSTR, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.findInjectedSource(::core::mem::transmute(&srcfile)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getEnumDebugStreams<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumdebugstreams: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getEnumDebugStreams() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumdebugstreams, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn findInlineFramesByAddr<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parent: *mut ::core::ffi::c_void, isect: u32, offset: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.findInlineFramesByAddr(::core::mem::transmute(&parent), ::core::mem::transmute_copy(&isect), ::core::mem::transmute_copy(&offset)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn findInlineFramesByRVA<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parent: *mut ::core::ffi::c_void, rva: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.findInlineFramesByRVA(::core::mem::transmute(&parent), ::core::mem::transmute_copy(&rva)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn findInlineFramesByVA<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parent: *mut ::core::ffi::c_void, va: u64, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.findInlineFramesByVA(::core::mem::transmute(&parent), ::core::mem::transmute_copy(&va)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn findInlineeLines<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parent: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.findInlineeLines(::core::mem::transmute(&parent)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn findInlineeLinesByAddr<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parent: *mut ::core::ffi::c_void, isect: u32, offset: u32, length: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.findInlineeLinesByAddr(::core::mem::transmute(&parent), ::core::mem::transmute_copy(&isect), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&length)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn findInlineeLinesByRVA<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parent: *mut ::core::ffi::c_void, rva: u32, length: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.findInlineeLinesByRVA(::core::mem::transmute(&parent), ::core::mem::transmute_copy(&rva), ::core::mem::transmute_copy(&length)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn findInlineeLinesByVA<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parent: *mut ::core::ffi::c_void, va: u64, length: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.findInlineeLinesByVA(::core::mem::transmute(&parent), ::core::mem::transmute_copy(&va), ::core::mem::transmute_copy(&length)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn findInlineeLinesByLinenum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, compiland: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, linenum: u32, column: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.findInlineeLinesByLinenum(::core::mem::transmute(&compiland), ::core::mem::transmute(&file), ::core::mem::transmute_copy(&linenum), ::core::mem::transmute_copy(&column)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn findInlineesByName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR, option: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.findInlineesByName(::core::mem::transmute(&name), ::core::mem::transmute_copy(&option)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn findAcceleratorInlineeLinesByLinenum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parent: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, linenum: u32, column: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.findAcceleratorInlineeLinesByLinenum(::core::mem::transmute(&parent), ::core::mem::transmute(&file), ::core::mem::transmute_copy(&linenum), ::core::mem::transmute_copy(&column)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn findSymbolsForAcceleratorPointerTag<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parent: *mut ::core::ffi::c_void, tagvalue: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.findSymbolsForAcceleratorPointerTag(::core::mem::transmute(&parent), ::core::mem::transmute_copy(&tagvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn findSymbolsByRVAForAcceleratorPointerTag<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parent: *mut ::core::ffi::c_void, tagvalue: u32, rva: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.findSymbolsByRVAForAcceleratorPointerTag(::core::mem::transmute(&parent), ::core::mem::transmute_copy(&tagvalue), ::core::mem::transmute_copy(&rva)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn findAcceleratorInlineesByName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR, option: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.findAcceleratorInlineesByName(::core::mem::transmute(&name), ::core::mem::transmute_copy(&option)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn addressForVA<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, va: u64, pisect: *mut u32, poffset: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.addressForVA(::core::mem::transmute_copy(&va), ::core::mem::transmute_copy(&pisect), ::core::mem::transmute_copy(&poffset)).into()
        }
        unsafe extern "system" fn addressForRVA<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rva: u32, pisect: *mut u32, poffset: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.addressForRVA(::core::mem::transmute_copy(&rva), ::core::mem::transmute_copy(&pisect), ::core::mem::transmute_copy(&poffset)).into()
        }
        unsafe extern "system" fn findILOffsetsByAddr<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isect: u32, offset: u32, length: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.findILOffsetsByAddr(::core::mem::transmute_copy(&isect), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&length)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn findILOffsetsByRVA<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rva: u32, length: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.findILOffsetsByRVA(::core::mem::transmute_copy(&rva), ::core::mem::transmute_copy(&length)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn findILOffsetsByVA<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, va: u64, length: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.findILOffsetsByVA(::core::mem::transmute_copy(&va), ::core::mem::transmute_copy(&length)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn findInputAssemblyFiles<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.findInputAssemblyFiles() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn findInputAssembly<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.findInputAssembly(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn findInputAssemblyById<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uniqueid: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.findInputAssemblyById(::core::mem::transmute_copy(&uniqueid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getFuncMDTokenMapSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcb: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getFuncMDTokenMapSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcb, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getFuncMDTokenMap<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cb: u32, pcb: *mut u32, pb: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.getFuncMDTokenMap(::core::mem::transmute_copy(&cb), ::core::mem::transmute_copy(&pcb), ::core::mem::transmute_copy(&pb)).into()
        }
        unsafe extern "system" fn getTypeMDTokenMapSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcb: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getTypeMDTokenMapSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcb, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getTypeMDTokenMap<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cb: u32, pcb: *mut u32, pb: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.getTypeMDTokenMap(::core::mem::transmute_copy(&cb), ::core::mem::transmute_copy(&pcb), ::core::mem::transmute_copy(&pb)).into()
        }
        unsafe extern "system" fn getNumberOfFunctionFragments_VA<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vafunc: u64, cbfunc: u32, pnumfragments: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getNumberOfFunctionFragments_VA(::core::mem::transmute_copy(&vafunc), ::core::mem::transmute_copy(&cbfunc)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnumfragments, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getNumberOfFunctionFragments_RVA<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rvafunc: u32, cbfunc: u32, pnumfragments: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getNumberOfFunctionFragments_RVA(::core::mem::transmute_copy(&rvafunc), ::core::mem::transmute_copy(&cbfunc)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnumfragments, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getFunctionFragments_VA<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vafunc: u64, cbfunc: u32, cfragments: u32, pvafragment: *mut u64, plenfragment: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.getFunctionFragments_VA(::core::mem::transmute_copy(&vafunc), ::core::mem::transmute_copy(&cbfunc), ::core::mem::transmute_copy(&cfragments), ::core::mem::transmute_copy(&pvafragment), ::core::mem::transmute_copy(&plenfragment)).into()
        }
        unsafe extern "system" fn getFunctionFragments_RVA<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rvafunc: u32, cbfunc: u32, cfragments: u32, prvafragment: *mut u32, plenfragment: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.getFunctionFragments_RVA(::core::mem::transmute_copy(&rvafunc), ::core::mem::transmute_copy(&cbfunc), ::core::mem::transmute_copy(&cfragments), ::core::mem::transmute_copy(&prvafragment), ::core::mem::transmute_copy(&plenfragment)).into()
        }
        unsafe extern "system" fn getExports<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getExports() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getHeapAllocationSites<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getHeapAllocationSites() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn findInputAssemblyFile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psymbol: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.findInputAssemblyFile(::core::mem::transmute(&psymbol)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            loadAddress: loadAddress::<Identity, Impl, OFFSET>,
            SetloadAddress: SetloadAddress::<Identity, Impl, OFFSET>,
            globalScope: globalScope::<Identity, Impl, OFFSET>,
            getEnumTables: getEnumTables::<Identity, Impl, OFFSET>,
            getSymbolsByAddr: getSymbolsByAddr::<Identity, Impl, OFFSET>,
            findChildren: findChildren::<Identity, Impl, OFFSET>,
            findChildrenEx: findChildrenEx::<Identity, Impl, OFFSET>,
            findChildrenExByAddr: findChildrenExByAddr::<Identity, Impl, OFFSET>,
            findChildrenExByVA: findChildrenExByVA::<Identity, Impl, OFFSET>,
            findChildrenExByRVA: findChildrenExByRVA::<Identity, Impl, OFFSET>,
            findSymbolByAddr: findSymbolByAddr::<Identity, Impl, OFFSET>,
            findSymbolByRVA: findSymbolByRVA::<Identity, Impl, OFFSET>,
            findSymbolByVA: findSymbolByVA::<Identity, Impl, OFFSET>,
            findSymbolByToken: findSymbolByToken::<Identity, Impl, OFFSET>,
            symsAreEquiv: symsAreEquiv::<Identity, Impl, OFFSET>,
            symbolById: symbolById::<Identity, Impl, OFFSET>,
            findSymbolByRVAEx: findSymbolByRVAEx::<Identity, Impl, OFFSET>,
            findSymbolByVAEx: findSymbolByVAEx::<Identity, Impl, OFFSET>,
            findFile: findFile::<Identity, Impl, OFFSET>,
            findFileById: findFileById::<Identity, Impl, OFFSET>,
            findLines: findLines::<Identity, Impl, OFFSET>,
            findLinesByAddr: findLinesByAddr::<Identity, Impl, OFFSET>,
            findLinesByRVA: findLinesByRVA::<Identity, Impl, OFFSET>,
            findLinesByVA: findLinesByVA::<Identity, Impl, OFFSET>,
            findLinesByLinenum: findLinesByLinenum::<Identity, Impl, OFFSET>,
            findInjectedSource: findInjectedSource::<Identity, Impl, OFFSET>,
            getEnumDebugStreams: getEnumDebugStreams::<Identity, Impl, OFFSET>,
            findInlineFramesByAddr: findInlineFramesByAddr::<Identity, Impl, OFFSET>,
            findInlineFramesByRVA: findInlineFramesByRVA::<Identity, Impl, OFFSET>,
            findInlineFramesByVA: findInlineFramesByVA::<Identity, Impl, OFFSET>,
            findInlineeLines: findInlineeLines::<Identity, Impl, OFFSET>,
            findInlineeLinesByAddr: findInlineeLinesByAddr::<Identity, Impl, OFFSET>,
            findInlineeLinesByRVA: findInlineeLinesByRVA::<Identity, Impl, OFFSET>,
            findInlineeLinesByVA: findInlineeLinesByVA::<Identity, Impl, OFFSET>,
            findInlineeLinesByLinenum: findInlineeLinesByLinenum::<Identity, Impl, OFFSET>,
            findInlineesByName: findInlineesByName::<Identity, Impl, OFFSET>,
            findAcceleratorInlineeLinesByLinenum: findAcceleratorInlineeLinesByLinenum::<Identity, Impl, OFFSET>,
            findSymbolsForAcceleratorPointerTag: findSymbolsForAcceleratorPointerTag::<Identity, Impl, OFFSET>,
            findSymbolsByRVAForAcceleratorPointerTag: findSymbolsByRVAForAcceleratorPointerTag::<Identity, Impl, OFFSET>,
            findAcceleratorInlineesByName: findAcceleratorInlineesByName::<Identity, Impl, OFFSET>,
            addressForVA: addressForVA::<Identity, Impl, OFFSET>,
            addressForRVA: addressForRVA::<Identity, Impl, OFFSET>,
            findILOffsetsByAddr: findILOffsetsByAddr::<Identity, Impl, OFFSET>,
            findILOffsetsByRVA: findILOffsetsByRVA::<Identity, Impl, OFFSET>,
            findILOffsetsByVA: findILOffsetsByVA::<Identity, Impl, OFFSET>,
            findInputAssemblyFiles: findInputAssemblyFiles::<Identity, Impl, OFFSET>,
            findInputAssembly: findInputAssembly::<Identity, Impl, OFFSET>,
            findInputAssemblyById: findInputAssemblyById::<Identity, Impl, OFFSET>,
            getFuncMDTokenMapSize: getFuncMDTokenMapSize::<Identity, Impl, OFFSET>,
            getFuncMDTokenMap: getFuncMDTokenMap::<Identity, Impl, OFFSET>,
            getTypeMDTokenMapSize: getTypeMDTokenMapSize::<Identity, Impl, OFFSET>,
            getTypeMDTokenMap: getTypeMDTokenMap::<Identity, Impl, OFFSET>,
            getNumberOfFunctionFragments_VA: getNumberOfFunctionFragments_VA::<Identity, Impl, OFFSET>,
            getNumberOfFunctionFragments_RVA: getNumberOfFunctionFragments_RVA::<Identity, Impl, OFFSET>,
            getFunctionFragments_VA: getFunctionFragments_VA::<Identity, Impl, OFFSET>,
            getFunctionFragments_RVA: getFunctionFragments_RVA::<Identity, Impl, OFFSET>,
            getExports: getExports::<Identity, Impl, OFFSET>,
            getHeapAllocationSites: getHeapAllocationSites::<Identity, Impl, OFFSET>,
            findInputAssemblyFile: findInputAssemblyFile::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiaSession as ::windows::core::Interface>::IID
    }
}
pub trait IDiaSourceFile_Impl: Sized {
    fn uniqueId(&self) -> ::windows::core::Result<u32>;
    fn fileName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn checksumType(&self) -> ::windows::core::Result<u32>;
    fn compilands(&self) -> ::windows::core::Result<IDiaEnumSymbols>;
    fn get_checksum(&self, cbdata: u32, pcbdata: *mut u32, pbdata: *mut u8) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDiaSourceFile {}
impl IDiaSourceFile_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSourceFile_Impl, const OFFSET: isize>() -> IDiaSourceFile_Vtbl {
        unsafe extern "system" fn uniqueId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSourceFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.uniqueId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fileName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSourceFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.fileName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn checksumType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSourceFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.checksumType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn compilands<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSourceFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.compilands() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_checksum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSourceFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbdata: u32, pcbdata: *mut u32, pbdata: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_checksum(::core::mem::transmute_copy(&cbdata), ::core::mem::transmute_copy(&pcbdata), ::core::mem::transmute_copy(&pbdata)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            uniqueId: uniqueId::<Identity, Impl, OFFSET>,
            fileName: fileName::<Identity, Impl, OFFSET>,
            checksumType: checksumType::<Identity, Impl, OFFSET>,
            compilands: compilands::<Identity, Impl, OFFSET>,
            get_checksum: get_checksum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiaSourceFile as ::windows::core::Interface>::IID
    }
}
pub trait IDiaStackFrame_Impl: Sized {
    fn r#type(&self) -> ::windows::core::Result<u32>;
    fn base(&self) -> ::windows::core::Result<u64>;
    fn size(&self) -> ::windows::core::Result<u32>;
    fn returnAddress(&self) -> ::windows::core::Result<u64>;
    fn localsBase(&self) -> ::windows::core::Result<u64>;
    fn lengthLocals(&self) -> ::windows::core::Result<u32>;
    fn lengthParams(&self) -> ::windows::core::Result<u32>;
    fn lengthProlog(&self) -> ::windows::core::Result<u32>;
    fn lengthSavedRegisters(&self) -> ::windows::core::Result<u32>;
    fn systemExceptionHandling(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn cplusplusExceptionHandling(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn functionStart(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn allocatesBasePointer(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn maxStack(&self) -> ::windows::core::Result<u32>;
    fn get_registerValue(&self, index: u32) -> ::windows::core::Result<u64>;
}
impl ::windows::core::RuntimeName for IDiaStackFrame {}
impl IDiaStackFrame_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaStackFrame_Impl, const OFFSET: isize>() -> IDiaStackFrame_Vtbl {
        unsafe extern "system" fn r#type<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaStackFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.r#type() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn base<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaStackFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.base() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn size<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaStackFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.size() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn returnAddress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaStackFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.returnAddress() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn localsBase<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaStackFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.localsBase() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn lengthLocals<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaStackFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.lengthLocals() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn lengthParams<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaStackFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.lengthParams() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn lengthProlog<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaStackFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.lengthProlog() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn lengthSavedRegisters<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaStackFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.lengthSavedRegisters() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn systemExceptionHandling<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaStackFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.systemExceptionHandling() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn cplusplusExceptionHandling<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaStackFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.cplusplusExceptionHandling() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn functionStart<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaStackFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.functionStart() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn allocatesBasePointer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaStackFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.allocatesBasePointer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn maxStack<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaStackFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.maxStack() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_registerValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaStackFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pretval: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_registerValue(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            r#type: r#type::<Identity, Impl, OFFSET>,
            base: base::<Identity, Impl, OFFSET>,
            size: size::<Identity, Impl, OFFSET>,
            returnAddress: returnAddress::<Identity, Impl, OFFSET>,
            localsBase: localsBase::<Identity, Impl, OFFSET>,
            lengthLocals: lengthLocals::<Identity, Impl, OFFSET>,
            lengthParams: lengthParams::<Identity, Impl, OFFSET>,
            lengthProlog: lengthProlog::<Identity, Impl, OFFSET>,
            lengthSavedRegisters: lengthSavedRegisters::<Identity, Impl, OFFSET>,
            systemExceptionHandling: systemExceptionHandling::<Identity, Impl, OFFSET>,
            cplusplusExceptionHandling: cplusplusExceptionHandling::<Identity, Impl, OFFSET>,
            functionStart: functionStart::<Identity, Impl, OFFSET>,
            allocatesBasePointer: allocatesBasePointer::<Identity, Impl, OFFSET>,
            maxStack: maxStack::<Identity, Impl, OFFSET>,
            get_registerValue: get_registerValue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiaStackFrame as ::windows::core::Interface>::IID
    }
}
pub trait IDiaStackWalkFrame_Impl: Sized {
    fn get_registerValue(&self, index: u32) -> ::windows::core::Result<u64>;
    fn put_registerValue(&self, index: u32, newval: u64) -> ::windows::core::Result<()>;
    fn readMemory(&self, r#type: MemoryTypeEnum, va: u64, cbdata: u32, pcbdata: *mut u32, pbdata: *mut u8) -> ::windows::core::Result<()>;
    fn searchForReturnAddress(&self, frame: &::core::option::Option<IDiaFrameData>) -> ::windows::core::Result<u64>;
    fn searchForReturnAddressStart(&self, frame: &::core::option::Option<IDiaFrameData>, startaddress: u64) -> ::windows::core::Result<u64>;
}
impl ::windows::core::RuntimeName for IDiaStackWalkFrame {}
impl IDiaStackWalkFrame_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaStackWalkFrame_Impl, const OFFSET: isize>() -> IDiaStackWalkFrame_Vtbl {
        unsafe extern "system" fn get_registerValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaStackWalkFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pretval: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_registerValue(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_registerValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaStackWalkFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, newval: u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.put_registerValue(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn readMemory<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaStackWalkFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: MemoryTypeEnum, va: u64, cbdata: u32, pcbdata: *mut u32, pbdata: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.readMemory(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&va), ::core::mem::transmute_copy(&cbdata), ::core::mem::transmute_copy(&pcbdata), ::core::mem::transmute_copy(&pbdata)).into()
        }
        unsafe extern "system" fn searchForReturnAddress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaStackWalkFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frame: *mut ::core::ffi::c_void, returnaddress: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.searchForReturnAddress(::core::mem::transmute(&frame)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(returnaddress, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn searchForReturnAddressStart<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaStackWalkFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frame: *mut ::core::ffi::c_void, startaddress: u64, returnaddress: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.searchForReturnAddressStart(::core::mem::transmute(&frame), ::core::mem::transmute_copy(&startaddress)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(returnaddress, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_registerValue: get_registerValue::<Identity, Impl, OFFSET>,
            put_registerValue: put_registerValue::<Identity, Impl, OFFSET>,
            readMemory: readMemory::<Identity, Impl, OFFSET>,
            searchForReturnAddress: searchForReturnAddress::<Identity, Impl, OFFSET>,
            searchForReturnAddressStart: searchForReturnAddressStart::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiaStackWalkFrame as ::windows::core::Interface>::IID
    }
}
pub trait IDiaStackWalkHelper_Impl: Sized {
    fn get_registerValue(&self, index: u32) -> ::windows::core::Result<u64>;
    fn put_registerValue(&self, index: u32, newval: u64) -> ::windows::core::Result<()>;
    fn readMemory(&self, r#type: MemoryTypeEnum, va: u64, cbdata: u32, pcbdata: *mut u32, pbdata: *mut u8) -> ::windows::core::Result<()>;
    fn searchForReturnAddress(&self, frame: &::core::option::Option<IDiaFrameData>) -> ::windows::core::Result<u64>;
    fn searchForReturnAddressStart(&self, frame: &::core::option::Option<IDiaFrameData>, startaddress: u64) -> ::windows::core::Result<u64>;
    fn frameForVA(&self, va: u64) -> ::windows::core::Result<IDiaFrameData>;
    fn symbolForVA(&self, va: u64) -> ::windows::core::Result<IDiaSymbol>;
    fn pdataForVA(&self, va: u64, cbdata: u32, pcbdata: *mut u32, pbdata: *mut u8) -> ::windows::core::Result<()>;
    fn imageForVA(&self, vacontext: u64) -> ::windows::core::Result<u64>;
    fn addressForVA(&self, va: u64, pisect: *mut u32, poffset: *mut u32) -> ::windows::core::Result<()>;
    fn numberOfFunctionFragmentsForVA(&self, vafunc: u64, cbfunc: u32) -> ::windows::core::Result<u32>;
    fn functionFragmentsForVA(&self, vafunc: u64, cbfunc: u32, cfragments: u32, pvafragment: *mut u64, plenfragment: *mut u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDiaStackWalkHelper {}
impl IDiaStackWalkHelper_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaStackWalkHelper_Impl, const OFFSET: isize>() -> IDiaStackWalkHelper_Vtbl {
        unsafe extern "system" fn get_registerValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaStackWalkHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pretval: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_registerValue(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_registerValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaStackWalkHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, newval: u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.put_registerValue(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn readMemory<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaStackWalkHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: MemoryTypeEnum, va: u64, cbdata: u32, pcbdata: *mut u32, pbdata: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.readMemory(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&va), ::core::mem::transmute_copy(&cbdata), ::core::mem::transmute_copy(&pcbdata), ::core::mem::transmute_copy(&pbdata)).into()
        }
        unsafe extern "system" fn searchForReturnAddress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaStackWalkHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frame: *mut ::core::ffi::c_void, returnaddress: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.searchForReturnAddress(::core::mem::transmute(&frame)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(returnaddress, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn searchForReturnAddressStart<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaStackWalkHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frame: *mut ::core::ffi::c_void, startaddress: u64, returnaddress: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.searchForReturnAddressStart(::core::mem::transmute(&frame), ::core::mem::transmute_copy(&startaddress)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(returnaddress, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn frameForVA<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaStackWalkHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, va: u64, ppframe: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.frameForVA(::core::mem::transmute_copy(&va)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppframe, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn symbolForVA<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaStackWalkHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, va: u64, ppsymbol: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.symbolForVA(::core::mem::transmute_copy(&va)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsymbol, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn pdataForVA<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaStackWalkHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, va: u64, cbdata: u32, pcbdata: *mut u32, pbdata: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.pdataForVA(::core::mem::transmute_copy(&va), ::core::mem::transmute_copy(&cbdata), ::core::mem::transmute_copy(&pcbdata), ::core::mem::transmute_copy(&pbdata)).into()
        }
        unsafe extern "system" fn imageForVA<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaStackWalkHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vacontext: u64, pvaimagestart: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.imageForVA(::core::mem::transmute_copy(&vacontext)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvaimagestart, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn addressForVA<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaStackWalkHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, va: u64, pisect: *mut u32, poffset: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.addressForVA(::core::mem::transmute_copy(&va), ::core::mem::transmute_copy(&pisect), ::core::mem::transmute_copy(&poffset)).into()
        }
        unsafe extern "system" fn numberOfFunctionFragmentsForVA<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaStackWalkHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vafunc: u64, cbfunc: u32, pnumfragments: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.numberOfFunctionFragmentsForVA(::core::mem::transmute_copy(&vafunc), ::core::mem::transmute_copy(&cbfunc)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnumfragments, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn functionFragmentsForVA<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaStackWalkHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vafunc: u64, cbfunc: u32, cfragments: u32, pvafragment: *mut u64, plenfragment: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.functionFragmentsForVA(::core::mem::transmute_copy(&vafunc), ::core::mem::transmute_copy(&cbfunc), ::core::mem::transmute_copy(&cfragments), ::core::mem::transmute_copy(&pvafragment), ::core::mem::transmute_copy(&plenfragment)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_registerValue: get_registerValue::<Identity, Impl, OFFSET>,
            put_registerValue: put_registerValue::<Identity, Impl, OFFSET>,
            readMemory: readMemory::<Identity, Impl, OFFSET>,
            searchForReturnAddress: searchForReturnAddress::<Identity, Impl, OFFSET>,
            searchForReturnAddressStart: searchForReturnAddressStart::<Identity, Impl, OFFSET>,
            frameForVA: frameForVA::<Identity, Impl, OFFSET>,
            symbolForVA: symbolForVA::<Identity, Impl, OFFSET>,
            pdataForVA: pdataForVA::<Identity, Impl, OFFSET>,
            imageForVA: imageForVA::<Identity, Impl, OFFSET>,
            addressForVA: addressForVA::<Identity, Impl, OFFSET>,
            numberOfFunctionFragmentsForVA: numberOfFunctionFragmentsForVA::<Identity, Impl, OFFSET>,
            functionFragmentsForVA: functionFragmentsForVA::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiaStackWalkHelper as ::windows::core::Interface>::IID
    }
}
pub trait IDiaStackWalkHelper2_Impl: Sized + IDiaStackWalkHelper_Impl {}
impl ::windows::core::RuntimeName for IDiaStackWalkHelper2 {}
impl IDiaStackWalkHelper2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaStackWalkHelper2_Impl, const OFFSET: isize>() -> IDiaStackWalkHelper2_Vtbl {
        Self { base__: IDiaStackWalkHelper_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiaStackWalkHelper2 as ::windows::core::Interface>::IID || iid == &<IDiaStackWalkHelper as ::windows::core::Interface>::IID
    }
}
pub trait IDiaStackWalker_Impl: Sized {
    fn getEnumFrames(&self, phelper: &::core::option::Option<IDiaStackWalkHelper>) -> ::windows::core::Result<IDiaEnumStackFrames>;
    fn getEnumFrames2(&self, cpuid: CV_CPU_TYPE_e, phelper: &::core::option::Option<IDiaStackWalkHelper>) -> ::windows::core::Result<IDiaEnumStackFrames>;
}
impl ::windows::core::RuntimeName for IDiaStackWalker {}
impl IDiaStackWalker_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaStackWalker_Impl, const OFFSET: isize>() -> IDiaStackWalker_Vtbl {
        unsafe extern "system" fn getEnumFrames<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaStackWalker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phelper: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getEnumFrames(::core::mem::transmute(&phelper)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getEnumFrames2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaStackWalker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpuid: CV_CPU_TYPE_e, phelper: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getEnumFrames2(::core::mem::transmute_copy(&cpuid), ::core::mem::transmute(&phelper)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            getEnumFrames: getEnumFrames::<Identity, Impl, OFFSET>,
            getEnumFrames2: getEnumFrames2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiaStackWalker as ::windows::core::Interface>::IID
    }
}
pub trait IDiaStackWalker2_Impl: Sized + IDiaStackWalker_Impl {}
impl ::windows::core::RuntimeName for IDiaStackWalker2 {}
impl IDiaStackWalker2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaStackWalker2_Impl, const OFFSET: isize>() -> IDiaStackWalker2_Vtbl {
        Self { base__: IDiaStackWalker_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiaStackWalker2 as ::windows::core::Interface>::IID || iid == &<IDiaStackWalker as ::windows::core::Interface>::IID
    }
}
pub trait IDiaSymbol_Impl: Sized {
    fn symIndexId(&self) -> ::windows::core::Result<u32>;
    fn symTag(&self) -> ::windows::core::Result<u32>;
    fn name(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn lexicalParent(&self) -> ::windows::core::Result<IDiaSymbol>;
    fn classParent(&self) -> ::windows::core::Result<IDiaSymbol>;
    fn r#type(&self) -> ::windows::core::Result<IDiaSymbol>;
    fn dataKind(&self) -> ::windows::core::Result<u32>;
    fn locationType(&self) -> ::windows::core::Result<u32>;
    fn addressSection(&self) -> ::windows::core::Result<u32>;
    fn addressOffset(&self) -> ::windows::core::Result<u32>;
    fn relativeVirtualAddress(&self) -> ::windows::core::Result<u32>;
    fn virtualAddress(&self) -> ::windows::core::Result<u64>;
    fn registerId(&self) -> ::windows::core::Result<u32>;
    fn offset(&self) -> ::windows::core::Result<i32>;
    fn length(&self) -> ::windows::core::Result<u64>;
    fn slot(&self) -> ::windows::core::Result<u32>;
    fn volatileType(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn constType(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn unalignedType(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn access(&self) -> ::windows::core::Result<u32>;
    fn libraryName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn platform(&self) -> ::windows::core::Result<u32>;
    fn language(&self) -> ::windows::core::Result<u32>;
    fn editAndContinueEnabled(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn frontEndMajor(&self) -> ::windows::core::Result<u32>;
    fn frontEndMinor(&self) -> ::windows::core::Result<u32>;
    fn frontEndBuild(&self) -> ::windows::core::Result<u32>;
    fn backEndMajor(&self) -> ::windows::core::Result<u32>;
    fn backEndMinor(&self) -> ::windows::core::Result<u32>;
    fn backEndBuild(&self) -> ::windows::core::Result<u32>;
    fn sourceFileName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn unused(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn thunkOrdinal(&self) -> ::windows::core::Result<u32>;
    fn thisAdjust(&self) -> ::windows::core::Result<i32>;
    fn virtualBaseOffset(&self) -> ::windows::core::Result<u32>;
    fn r#virtual(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn intro(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn pure(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn callingConvention(&self) -> ::windows::core::Result<u32>;
    fn value(&self) -> ::windows::core::Result<::windows::Win32::System::Com::VARIANT>;
    fn baseType(&self) -> ::windows::core::Result<u32>;
    fn token(&self) -> ::windows::core::Result<u32>;
    fn timeStamp(&self) -> ::windows::core::Result<u32>;
    fn guid(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn symbolsFileName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn reference(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn count(&self) -> ::windows::core::Result<u32>;
    fn bitPosition(&self) -> ::windows::core::Result<u32>;
    fn arrayIndexType(&self) -> ::windows::core::Result<IDiaSymbol>;
    fn packed(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn constructor(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn overloadedOperator(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn nested(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn hasNestedTypes(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn hasAssignmentOperator(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn hasCastOperator(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn scoped(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn virtualBaseClass(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn indirectVirtualBaseClass(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn virtualBasePointerOffset(&self) -> ::windows::core::Result<i32>;
    fn virtualTableShape(&self) -> ::windows::core::Result<IDiaSymbol>;
    fn lexicalParentId(&self) -> ::windows::core::Result<u32>;
    fn classParentId(&self) -> ::windows::core::Result<u32>;
    fn typeId(&self) -> ::windows::core::Result<u32>;
    fn arrayIndexTypeId(&self) -> ::windows::core::Result<u32>;
    fn virtualTableShapeId(&self) -> ::windows::core::Result<u32>;
    fn code(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn function(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn managed(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn msil(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn virtualBaseDispIndex(&self) -> ::windows::core::Result<u32>;
    fn undecoratedName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn age(&self) -> ::windows::core::Result<u32>;
    fn signature(&self) -> ::windows::core::Result<u32>;
    fn compilerGenerated(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn addressTaken(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn rank(&self) -> ::windows::core::Result<u32>;
    fn lowerBound(&self) -> ::windows::core::Result<IDiaSymbol>;
    fn upperBound(&self) -> ::windows::core::Result<IDiaSymbol>;
    fn lowerBoundId(&self) -> ::windows::core::Result<u32>;
    fn upperBoundId(&self) -> ::windows::core::Result<u32>;
    fn get_dataBytes(&self, cbdata: u32, pcbdata: *mut u32, pbdata: *mut u8) -> ::windows::core::Result<()>;
    fn findChildren(&self, symtag: SymTag, name: &::windows::core::PCWSTR, compareflags: u32) -> ::windows::core::Result<IDiaEnumSymbols>;
    fn findChildrenEx(&self, symtag: SymTag, name: &::windows::core::PCWSTR, compareflags: u32) -> ::windows::core::Result<IDiaEnumSymbols>;
    fn findChildrenExByAddr(&self, symtag: SymTag, name: &::windows::core::PCWSTR, compareflags: u32, isect: u32, offset: u32) -> ::windows::core::Result<IDiaEnumSymbols>;
    fn findChildrenExByVA(&self, symtag: SymTag, name: &::windows::core::PCWSTR, compareflags: u32, va: u64) -> ::windows::core::Result<IDiaEnumSymbols>;
    fn findChildrenExByRVA(&self, symtag: SymTag, name: &::windows::core::PCWSTR, compareflags: u32, rva: u32) -> ::windows::core::Result<IDiaEnumSymbols>;
    fn targetSection(&self) -> ::windows::core::Result<u32>;
    fn targetOffset(&self) -> ::windows::core::Result<u32>;
    fn targetRelativeVirtualAddress(&self) -> ::windows::core::Result<u32>;
    fn targetVirtualAddress(&self) -> ::windows::core::Result<u64>;
    fn machineType(&self) -> ::windows::core::Result<u32>;
    fn oemId(&self) -> ::windows::core::Result<u32>;
    fn oemSymbolId(&self) -> ::windows::core::Result<u32>;
    fn get_types(&self, ctypes: u32, pctypes: *mut u32, ptypes: *mut ::core::option::Option<IDiaSymbol>) -> ::windows::core::Result<()>;
    fn get_typeIds(&self, ctypeids: u32, pctypeids: *mut u32, pdwtypeids: *mut u32) -> ::windows::core::Result<()>;
    fn objectPointerType(&self) -> ::windows::core::Result<IDiaSymbol>;
    fn udtKind(&self) -> ::windows::core::Result<u32>;
    fn get_undecoratedNameEx(&self, undecorateoptions: u32) -> ::windows::core::Result<::windows::core::BSTR>;
    fn noReturn(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn customCallingConvention(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn noInline(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn optimizedCodeDebugInfo(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn notReached(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn interruptReturn(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn farReturn(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn isStatic(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn hasDebugInfo(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn isLTCG(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn isDataAligned(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn hasSecurityChecks(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn compilerName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn hasAlloca(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn hasSetJump(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn hasLongJump(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn hasInlAsm(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn hasEH(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn hasSEH(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn hasEHa(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn isNaked(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn isAggregated(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn isSplitted(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn container(&self) -> ::windows::core::Result<IDiaSymbol>;
    fn inlSpec(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn noStackOrdering(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn virtualBaseTableType(&self) -> ::windows::core::Result<IDiaSymbol>;
    fn hasManagedCode(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn isHotpatchable(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn isCVTCIL(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn isMSILNetmodule(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn isCTypes(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn isStripped(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn frontEndQFE(&self) -> ::windows::core::Result<u32>;
    fn backEndQFE(&self) -> ::windows::core::Result<u32>;
    fn wasInlined(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn strictGSCheck(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn isCxxReturnUdt(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn isConstructorVirtualBase(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn RValueReference(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn unmodifiedType(&self) -> ::windows::core::Result<IDiaSymbol>;
    fn framePointerPresent(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn isSafeBuffers(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn intrinsic(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn sealed(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn hfaFloat(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn hfaDouble(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn liveRangeStartAddressSection(&self) -> ::windows::core::Result<u32>;
    fn liveRangeStartAddressOffset(&self) -> ::windows::core::Result<u32>;
    fn liveRangeStartRelativeVirtualAddress(&self) -> ::windows::core::Result<u32>;
    fn countLiveRanges(&self) -> ::windows::core::Result<u32>;
    fn liveRangeLength(&self) -> ::windows::core::Result<u64>;
    fn offsetInUdt(&self) -> ::windows::core::Result<u32>;
    fn paramBasePointerRegisterId(&self) -> ::windows::core::Result<u32>;
    fn localBasePointerRegisterId(&self) -> ::windows::core::Result<u32>;
    fn isLocationControlFlowDependent(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn stride(&self) -> ::windows::core::Result<u32>;
    fn numberOfRows(&self) -> ::windows::core::Result<u32>;
    fn numberOfColumns(&self) -> ::windows::core::Result<u32>;
    fn isMatrixRowMajor(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn get_numericProperties(&self, cnt: u32, pcnt: *mut u32, pproperties: *mut u32) -> ::windows::core::Result<()>;
    fn get_modifierValues(&self, cnt: u32, pcnt: *mut u32, pmodifiers: *mut u16) -> ::windows::core::Result<()>;
    fn isReturnValue(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn isOptimizedAway(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn builtInKind(&self) -> ::windows::core::Result<u32>;
    fn registerType(&self) -> ::windows::core::Result<u32>;
    fn baseDataSlot(&self) -> ::windows::core::Result<u32>;
    fn baseDataOffset(&self) -> ::windows::core::Result<u32>;
    fn textureSlot(&self) -> ::windows::core::Result<u32>;
    fn samplerSlot(&self) -> ::windows::core::Result<u32>;
    fn uavSlot(&self) -> ::windows::core::Result<u32>;
    fn sizeInUdt(&self) -> ::windows::core::Result<u32>;
    fn memorySpaceKind(&self) -> ::windows::core::Result<u32>;
    fn unmodifiedTypeId(&self) -> ::windows::core::Result<u32>;
    fn subTypeId(&self) -> ::windows::core::Result<u32>;
    fn subType(&self) -> ::windows::core::Result<IDiaSymbol>;
    fn numberOfModifiers(&self) -> ::windows::core::Result<u32>;
    fn numberOfRegisterIndices(&self) -> ::windows::core::Result<u32>;
    fn isHLSLData(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn isPointerToDataMember(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn isPointerToMemberFunction(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn isSingleInheritance(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn isMultipleInheritance(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn isVirtualInheritance(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn restrictedType(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn isPointerBasedOnSymbolValue(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn baseSymbol(&self) -> ::windows::core::Result<IDiaSymbol>;
    fn baseSymbolId(&self) -> ::windows::core::Result<u32>;
    fn objectFileName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn isAcceleratorGroupSharedLocal(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn isAcceleratorPointerTagLiveRange(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn isAcceleratorStubFunction(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn numberOfAcceleratorPointerTags(&self) -> ::windows::core::Result<u32>;
    fn isSdl(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn isWinRTPointer(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn isRefUdt(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn isValueUdt(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn isInterfaceUdt(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn findInlineFramesByAddr(&self, isect: u32, offset: u32) -> ::windows::core::Result<IDiaEnumSymbols>;
    fn findInlineFramesByRVA(&self, rva: u32) -> ::windows::core::Result<IDiaEnumSymbols>;
    fn findInlineFramesByVA(&self, va: u64) -> ::windows::core::Result<IDiaEnumSymbols>;
    fn findInlineeLines(&self) -> ::windows::core::Result<IDiaEnumLineNumbers>;
    fn findInlineeLinesByAddr(&self, isect: u32, offset: u32, length: u32) -> ::windows::core::Result<IDiaEnumLineNumbers>;
    fn findInlineeLinesByRVA(&self, rva: u32, length: u32) -> ::windows::core::Result<IDiaEnumLineNumbers>;
    fn findInlineeLinesByVA(&self, va: u64, length: u32) -> ::windows::core::Result<IDiaEnumLineNumbers>;
    fn findSymbolsForAcceleratorPointerTag(&self, tagvalue: u32) -> ::windows::core::Result<IDiaEnumSymbols>;
    fn findSymbolsByRVAForAcceleratorPointerTag(&self, tagvalue: u32, rva: u32) -> ::windows::core::Result<IDiaEnumSymbols>;
    fn get_acceleratorPointerTags(&self, cnt: u32, pcnt: *mut u32, ppointertags: *mut u32) -> ::windows::core::Result<()>;
    fn getSrcLineOnTypeDefn(&self) -> ::windows::core::Result<IDiaLineNumber>;
    fn isPGO(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn hasValidPGOCounts(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn isOptimizedForSpeed(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn PGOEntryCount(&self) -> ::windows::core::Result<u32>;
    fn PGOEdgeCount(&self) -> ::windows::core::Result<u32>;
    fn PGODynamicInstructionCount(&self) -> ::windows::core::Result<u64>;
    fn staticSize(&self) -> ::windows::core::Result<u32>;
    fn finalLiveStaticSize(&self) -> ::windows::core::Result<u32>;
    fn phaseName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn hasControlFlowCheck(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn constantExport(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn dataExport(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn privateExport(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn noNameExport(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn exportHasExplicitlyAssignedOrdinal(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn exportIsForwarder(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn ordinal(&self) -> ::windows::core::Result<u32>;
    fn frameSize(&self) -> ::windows::core::Result<u32>;
    fn exceptionHandlerAddressSection(&self) -> ::windows::core::Result<u32>;
    fn exceptionHandlerAddressOffset(&self) -> ::windows::core::Result<u32>;
    fn exceptionHandlerRelativeVirtualAddress(&self) -> ::windows::core::Result<u32>;
    fn exceptionHandlerVirtualAddress(&self) -> ::windows::core::Result<u64>;
    fn findInputAssemblyFile(&self) -> ::windows::core::Result<IDiaInputAssemblyFile>;
    fn characteristics(&self) -> ::windows::core::Result<u32>;
    fn coffGroup(&self) -> ::windows::core::Result<IDiaSymbol>;
    fn bindID(&self) -> ::windows::core::Result<u32>;
    fn bindSpace(&self) -> ::windows::core::Result<u32>;
    fn bindSlot(&self) -> ::windows::core::Result<u32>;
}
impl ::windows::core::RuntimeName for IDiaSymbol {}
impl IDiaSymbol_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>() -> IDiaSymbol_Vtbl {
        unsafe extern "system" fn symIndexId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.symIndexId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn symTag<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.symTag() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn lexicalParent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.lexicalParent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn classParent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.classParent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn r#type<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.r#type() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn dataKind<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.dataKind() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn locationType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.locationType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn addressSection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.addressSection() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn addressOffset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.addressOffset() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn relativeVirtualAddress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.relativeVirtualAddress() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn virtualAddress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.virtualAddress() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn registerId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.registerId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn offset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.offset() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn length<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.length() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn slot<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.slot() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn volatileType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.volatileType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn constType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.constType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn unalignedType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.unalignedType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn access<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.access() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn libraryName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.libraryName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn platform<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.platform() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn language<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.language() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn editAndContinueEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.editAndContinueEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn frontEndMajor<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.frontEndMajor() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn frontEndMinor<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.frontEndMinor() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn frontEndBuild<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.frontEndBuild() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn backEndMajor<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.backEndMajor() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn backEndMinor<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.backEndMinor() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn backEndBuild<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.backEndBuild() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn sourceFileName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.sourceFileName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn unused<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.unused() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn thunkOrdinal<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.thunkOrdinal() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn thisAdjust<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.thisAdjust() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn virtualBaseOffset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.virtualBaseOffset() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn r#virtual<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.r#virtual() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn intro<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.intro() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn pure<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.pure() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn callingConvention<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.callingConvention() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn value<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.value() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn baseType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.baseType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn token<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.token() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn timeStamp<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.timeStamp() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn guid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.guid() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn symbolsFileName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.symbolsFileName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn reference<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.reference() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn bitPosition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.bitPosition() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn arrayIndexType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.arrayIndexType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn packed<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.packed() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn constructor<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.constructor() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn overloadedOperator<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.overloadedOperator() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn nested<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.nested() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn hasNestedTypes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.hasNestedTypes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn hasAssignmentOperator<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.hasAssignmentOperator() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn hasCastOperator<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.hasCastOperator() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn scoped<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.scoped() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn virtualBaseClass<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.virtualBaseClass() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn indirectVirtualBaseClass<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.indirectVirtualBaseClass() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn virtualBasePointerOffset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.virtualBasePointerOffset() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn virtualTableShape<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.virtualTableShape() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn lexicalParentId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.lexicalParentId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn classParentId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.classParentId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn typeId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.typeId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn arrayIndexTypeId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.arrayIndexTypeId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn virtualTableShapeId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.virtualTableShapeId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn code<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.code() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn function<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.function() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn managed<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.managed() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn msil<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.msil() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn virtualBaseDispIndex<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.virtualBaseDispIndex() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn undecoratedName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.undecoratedName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn age<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.age() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn signature<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.signature() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn compilerGenerated<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.compilerGenerated() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn addressTaken<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.addressTaken() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn rank<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.rank() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn lowerBound<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.lowerBound() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn upperBound<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.upperBound() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn lowerBoundId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.lowerBoundId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn upperBoundId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.upperBoundId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_dataBytes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbdata: u32, pcbdata: *mut u32, pbdata: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_dataBytes(::core::mem::transmute_copy(&cbdata), ::core::mem::transmute_copy(&pcbdata), ::core::mem::transmute_copy(&pbdata)).into()
        }
        unsafe extern "system" fn findChildren<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, symtag: SymTag, name: ::windows::core::PCWSTR, compareflags: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.findChildren(::core::mem::transmute_copy(&symtag), ::core::mem::transmute(&name), ::core::mem::transmute_copy(&compareflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn findChildrenEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, symtag: SymTag, name: ::windows::core::PCWSTR, compareflags: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.findChildrenEx(::core::mem::transmute_copy(&symtag), ::core::mem::transmute(&name), ::core::mem::transmute_copy(&compareflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn findChildrenExByAddr<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, symtag: SymTag, name: ::windows::core::PCWSTR, compareflags: u32, isect: u32, offset: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.findChildrenExByAddr(::core::mem::transmute_copy(&symtag), ::core::mem::transmute(&name), ::core::mem::transmute_copy(&compareflags), ::core::mem::transmute_copy(&isect), ::core::mem::transmute_copy(&offset)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn findChildrenExByVA<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, symtag: SymTag, name: ::windows::core::PCWSTR, compareflags: u32, va: u64, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.findChildrenExByVA(::core::mem::transmute_copy(&symtag), ::core::mem::transmute(&name), ::core::mem::transmute_copy(&compareflags), ::core::mem::transmute_copy(&va)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn findChildrenExByRVA<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, symtag: SymTag, name: ::windows::core::PCWSTR, compareflags: u32, rva: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.findChildrenExByRVA(::core::mem::transmute_copy(&symtag), ::core::mem::transmute(&name), ::core::mem::transmute_copy(&compareflags), ::core::mem::transmute_copy(&rva)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn targetSection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.targetSection() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn targetOffset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.targetOffset() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn targetRelativeVirtualAddress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.targetRelativeVirtualAddress() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn targetVirtualAddress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.targetVirtualAddress() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn machineType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.machineType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn oemId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.oemId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn oemSymbolId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.oemSymbolId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_types<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ctypes: u32, pctypes: *mut u32, ptypes: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_types(::core::mem::transmute_copy(&ctypes), ::core::mem::transmute_copy(&pctypes), ::core::mem::transmute_copy(&ptypes)).into()
        }
        unsafe extern "system" fn get_typeIds<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ctypeids: u32, pctypeids: *mut u32, pdwtypeids: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_typeIds(::core::mem::transmute_copy(&ctypeids), ::core::mem::transmute_copy(&pctypeids), ::core::mem::transmute_copy(&pdwtypeids)).into()
        }
        unsafe extern "system" fn objectPointerType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.objectPointerType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn udtKind<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.udtKind() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_undecoratedNameEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, undecorateoptions: u32, name: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_undecoratedNameEx(::core::mem::transmute_copy(&undecorateoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn noReturn<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.noReturn() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn customCallingConvention<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.customCallingConvention() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn noInline<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.noInline() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn optimizedCodeDebugInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.optimizedCodeDebugInfo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn notReached<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.notReached() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn interruptReturn<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.interruptReturn() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn farReturn<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.farReturn() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isStatic<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.isStatic() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn hasDebugInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.hasDebugInfo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isLTCG<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.isLTCG() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isDataAligned<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.isDataAligned() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn hasSecurityChecks<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.hasSecurityChecks() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn compilerName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.compilerName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn hasAlloca<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.hasAlloca() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn hasSetJump<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.hasSetJump() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn hasLongJump<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.hasLongJump() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn hasInlAsm<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.hasInlAsm() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn hasEH<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.hasEH() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn hasSEH<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.hasSEH() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn hasEHa<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.hasEHa() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isNaked<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.isNaked() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isAggregated<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.isAggregated() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isSplitted<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.isSplitted() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn container<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.container() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn inlSpec<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.inlSpec() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn noStackOrdering<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.noStackOrdering() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn virtualBaseTableType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.virtualBaseTableType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn hasManagedCode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.hasManagedCode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isHotpatchable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.isHotpatchable() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isCVTCIL<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.isCVTCIL() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isMSILNetmodule<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.isMSILNetmodule() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isCTypes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.isCTypes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isStripped<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.isStripped() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn frontEndQFE<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.frontEndQFE() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn backEndQFE<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.backEndQFE() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn wasInlined<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.wasInlined() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn strictGSCheck<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.strictGSCheck() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isCxxReturnUdt<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.isCxxReturnUdt() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isConstructorVirtualBase<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.isConstructorVirtualBase() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RValueReference<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RValueReference() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn unmodifiedType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.unmodifiedType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn framePointerPresent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.framePointerPresent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isSafeBuffers<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.isSafeBuffers() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn intrinsic<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.intrinsic() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn sealed<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.sealed() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn hfaFloat<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.hfaFloat() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn hfaDouble<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.hfaDouble() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn liveRangeStartAddressSection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.liveRangeStartAddressSection() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn liveRangeStartAddressOffset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.liveRangeStartAddressOffset() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn liveRangeStartRelativeVirtualAddress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.liveRangeStartRelativeVirtualAddress() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn countLiveRanges<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.countLiveRanges() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn liveRangeLength<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.liveRangeLength() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn offsetInUdt<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.offsetInUdt() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn paramBasePointerRegisterId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.paramBasePointerRegisterId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn localBasePointerRegisterId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.localBasePointerRegisterId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isLocationControlFlowDependent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.isLocationControlFlowDependent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn stride<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.stride() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn numberOfRows<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.numberOfRows() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn numberOfColumns<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.numberOfColumns() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isMatrixRowMajor<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.isMatrixRowMajor() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_numericProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cnt: u32, pcnt: *mut u32, pproperties: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_numericProperties(::core::mem::transmute_copy(&cnt), ::core::mem::transmute_copy(&pcnt), ::core::mem::transmute_copy(&pproperties)).into()
        }
        unsafe extern "system" fn get_modifierValues<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cnt: u32, pcnt: *mut u32, pmodifiers: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_modifierValues(::core::mem::transmute_copy(&cnt), ::core::mem::transmute_copy(&pcnt), ::core::mem::transmute_copy(&pmodifiers)).into()
        }
        unsafe extern "system" fn isReturnValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.isReturnValue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isOptimizedAway<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.isOptimizedAway() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn builtInKind<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.builtInKind() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn registerType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.registerType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn baseDataSlot<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.baseDataSlot() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn baseDataOffset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.baseDataOffset() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn textureSlot<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.textureSlot() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn samplerSlot<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.samplerSlot() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn uavSlot<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.uavSlot() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn sizeInUdt<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.sizeInUdt() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn memorySpaceKind<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.memorySpaceKind() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn unmodifiedTypeId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.unmodifiedTypeId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn subTypeId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.subTypeId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn subType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.subType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn numberOfModifiers<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.numberOfModifiers() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn numberOfRegisterIndices<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.numberOfRegisterIndices() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isHLSLData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.isHLSLData() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isPointerToDataMember<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.isPointerToDataMember() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isPointerToMemberFunction<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.isPointerToMemberFunction() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isSingleInheritance<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.isSingleInheritance() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isMultipleInheritance<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.isMultipleInheritance() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isVirtualInheritance<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.isVirtualInheritance() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn restrictedType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.restrictedType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isPointerBasedOnSymbolValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.isPointerBasedOnSymbolValue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn baseSymbol<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.baseSymbol() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn baseSymbolId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.baseSymbolId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn objectFileName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.objectFileName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isAcceleratorGroupSharedLocal<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.isAcceleratorGroupSharedLocal() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isAcceleratorPointerTagLiveRange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.isAcceleratorPointerTagLiveRange() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isAcceleratorStubFunction<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.isAcceleratorStubFunction() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn numberOfAcceleratorPointerTags<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.numberOfAcceleratorPointerTags() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isSdl<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.isSdl() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isWinRTPointer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.isWinRTPointer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isRefUdt<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.isRefUdt() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isValueUdt<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.isValueUdt() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isInterfaceUdt<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.isInterfaceUdt() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn findInlineFramesByAddr<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isect: u32, offset: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.findInlineFramesByAddr(::core::mem::transmute_copy(&isect), ::core::mem::transmute_copy(&offset)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn findInlineFramesByRVA<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rva: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.findInlineFramesByRVA(::core::mem::transmute_copy(&rva)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn findInlineFramesByVA<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, va: u64, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.findInlineFramesByVA(::core::mem::transmute_copy(&va)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn findInlineeLines<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.findInlineeLines() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn findInlineeLinesByAddr<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isect: u32, offset: u32, length: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.findInlineeLinesByAddr(::core::mem::transmute_copy(&isect), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&length)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn findInlineeLinesByRVA<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rva: u32, length: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.findInlineeLinesByRVA(::core::mem::transmute_copy(&rva), ::core::mem::transmute_copy(&length)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn findInlineeLinesByVA<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, va: u64, length: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.findInlineeLinesByVA(::core::mem::transmute_copy(&va), ::core::mem::transmute_copy(&length)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn findSymbolsForAcceleratorPointerTag<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tagvalue: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.findSymbolsForAcceleratorPointerTag(::core::mem::transmute_copy(&tagvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn findSymbolsByRVAForAcceleratorPointerTag<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tagvalue: u32, rva: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.findSymbolsByRVAForAcceleratorPointerTag(::core::mem::transmute_copy(&tagvalue), ::core::mem::transmute_copy(&rva)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_acceleratorPointerTags<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cnt: u32, pcnt: *mut u32, ppointertags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_acceleratorPointerTags(::core::mem::transmute_copy(&cnt), ::core::mem::transmute_copy(&pcnt), ::core::mem::transmute_copy(&ppointertags)).into()
        }
        unsafe extern "system" fn getSrcLineOnTypeDefn<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getSrcLineOnTypeDefn() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isPGO<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.isPGO() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn hasValidPGOCounts<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.hasValidPGOCounts() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isOptimizedForSpeed<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.isOptimizedForSpeed() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PGOEntryCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PGOEntryCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PGOEdgeCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PGOEdgeCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PGODynamicInstructionCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PGODynamicInstructionCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn staticSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.staticSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn finalLiveStaticSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.finalLiveStaticSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn phaseName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.phaseName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn hasControlFlowCheck<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.hasControlFlowCheck() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn constantExport<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.constantExport() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn dataExport<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.dataExport() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn privateExport<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.privateExport() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn noNameExport<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.noNameExport() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn exportHasExplicitlyAssignedOrdinal<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.exportHasExplicitlyAssignedOrdinal() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn exportIsForwarder<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.exportIsForwarder() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ordinal<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ordinal() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn frameSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.frameSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn exceptionHandlerAddressSection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.exceptionHandlerAddressSection() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn exceptionHandlerAddressOffset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.exceptionHandlerAddressOffset() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn exceptionHandlerRelativeVirtualAddress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.exceptionHandlerRelativeVirtualAddress() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn exceptionHandlerVirtualAddress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.exceptionHandlerVirtualAddress() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn findInputAssemblyFile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.findInputAssemblyFile() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn characteristics<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.characteristics() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn coffGroup<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.coffGroup() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn bindID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.bindID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn bindSpace<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.bindSpace() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn bindSlot<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.bindSlot() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            symIndexId: symIndexId::<Identity, Impl, OFFSET>,
            symTag: symTag::<Identity, Impl, OFFSET>,
            name: name::<Identity, Impl, OFFSET>,
            lexicalParent: lexicalParent::<Identity, Impl, OFFSET>,
            classParent: classParent::<Identity, Impl, OFFSET>,
            r#type: r#type::<Identity, Impl, OFFSET>,
            dataKind: dataKind::<Identity, Impl, OFFSET>,
            locationType: locationType::<Identity, Impl, OFFSET>,
            addressSection: addressSection::<Identity, Impl, OFFSET>,
            addressOffset: addressOffset::<Identity, Impl, OFFSET>,
            relativeVirtualAddress: relativeVirtualAddress::<Identity, Impl, OFFSET>,
            virtualAddress: virtualAddress::<Identity, Impl, OFFSET>,
            registerId: registerId::<Identity, Impl, OFFSET>,
            offset: offset::<Identity, Impl, OFFSET>,
            length: length::<Identity, Impl, OFFSET>,
            slot: slot::<Identity, Impl, OFFSET>,
            volatileType: volatileType::<Identity, Impl, OFFSET>,
            constType: constType::<Identity, Impl, OFFSET>,
            unalignedType: unalignedType::<Identity, Impl, OFFSET>,
            access: access::<Identity, Impl, OFFSET>,
            libraryName: libraryName::<Identity, Impl, OFFSET>,
            platform: platform::<Identity, Impl, OFFSET>,
            language: language::<Identity, Impl, OFFSET>,
            editAndContinueEnabled: editAndContinueEnabled::<Identity, Impl, OFFSET>,
            frontEndMajor: frontEndMajor::<Identity, Impl, OFFSET>,
            frontEndMinor: frontEndMinor::<Identity, Impl, OFFSET>,
            frontEndBuild: frontEndBuild::<Identity, Impl, OFFSET>,
            backEndMajor: backEndMajor::<Identity, Impl, OFFSET>,
            backEndMinor: backEndMinor::<Identity, Impl, OFFSET>,
            backEndBuild: backEndBuild::<Identity, Impl, OFFSET>,
            sourceFileName: sourceFileName::<Identity, Impl, OFFSET>,
            unused: unused::<Identity, Impl, OFFSET>,
            thunkOrdinal: thunkOrdinal::<Identity, Impl, OFFSET>,
            thisAdjust: thisAdjust::<Identity, Impl, OFFSET>,
            virtualBaseOffset: virtualBaseOffset::<Identity, Impl, OFFSET>,
            r#virtual: r#virtual::<Identity, Impl, OFFSET>,
            intro: intro::<Identity, Impl, OFFSET>,
            pure: pure::<Identity, Impl, OFFSET>,
            callingConvention: callingConvention::<Identity, Impl, OFFSET>,
            value: value::<Identity, Impl, OFFSET>,
            baseType: baseType::<Identity, Impl, OFFSET>,
            token: token::<Identity, Impl, OFFSET>,
            timeStamp: timeStamp::<Identity, Impl, OFFSET>,
            guid: guid::<Identity, Impl, OFFSET>,
            symbolsFileName: symbolsFileName::<Identity, Impl, OFFSET>,
            reference: reference::<Identity, Impl, OFFSET>,
            count: count::<Identity, Impl, OFFSET>,
            bitPosition: bitPosition::<Identity, Impl, OFFSET>,
            arrayIndexType: arrayIndexType::<Identity, Impl, OFFSET>,
            packed: packed::<Identity, Impl, OFFSET>,
            constructor: constructor::<Identity, Impl, OFFSET>,
            overloadedOperator: overloadedOperator::<Identity, Impl, OFFSET>,
            nested: nested::<Identity, Impl, OFFSET>,
            hasNestedTypes: hasNestedTypes::<Identity, Impl, OFFSET>,
            hasAssignmentOperator: hasAssignmentOperator::<Identity, Impl, OFFSET>,
            hasCastOperator: hasCastOperator::<Identity, Impl, OFFSET>,
            scoped: scoped::<Identity, Impl, OFFSET>,
            virtualBaseClass: virtualBaseClass::<Identity, Impl, OFFSET>,
            indirectVirtualBaseClass: indirectVirtualBaseClass::<Identity, Impl, OFFSET>,
            virtualBasePointerOffset: virtualBasePointerOffset::<Identity, Impl, OFFSET>,
            virtualTableShape: virtualTableShape::<Identity, Impl, OFFSET>,
            lexicalParentId: lexicalParentId::<Identity, Impl, OFFSET>,
            classParentId: classParentId::<Identity, Impl, OFFSET>,
            typeId: typeId::<Identity, Impl, OFFSET>,
            arrayIndexTypeId: arrayIndexTypeId::<Identity, Impl, OFFSET>,
            virtualTableShapeId: virtualTableShapeId::<Identity, Impl, OFFSET>,
            code: code::<Identity, Impl, OFFSET>,
            function: function::<Identity, Impl, OFFSET>,
            managed: managed::<Identity, Impl, OFFSET>,
            msil: msil::<Identity, Impl, OFFSET>,
            virtualBaseDispIndex: virtualBaseDispIndex::<Identity, Impl, OFFSET>,
            undecoratedName: undecoratedName::<Identity, Impl, OFFSET>,
            age: age::<Identity, Impl, OFFSET>,
            signature: signature::<Identity, Impl, OFFSET>,
            compilerGenerated: compilerGenerated::<Identity, Impl, OFFSET>,
            addressTaken: addressTaken::<Identity, Impl, OFFSET>,
            rank: rank::<Identity, Impl, OFFSET>,
            lowerBound: lowerBound::<Identity, Impl, OFFSET>,
            upperBound: upperBound::<Identity, Impl, OFFSET>,
            lowerBoundId: lowerBoundId::<Identity, Impl, OFFSET>,
            upperBoundId: upperBoundId::<Identity, Impl, OFFSET>,
            get_dataBytes: get_dataBytes::<Identity, Impl, OFFSET>,
            findChildren: findChildren::<Identity, Impl, OFFSET>,
            findChildrenEx: findChildrenEx::<Identity, Impl, OFFSET>,
            findChildrenExByAddr: findChildrenExByAddr::<Identity, Impl, OFFSET>,
            findChildrenExByVA: findChildrenExByVA::<Identity, Impl, OFFSET>,
            findChildrenExByRVA: findChildrenExByRVA::<Identity, Impl, OFFSET>,
            targetSection: targetSection::<Identity, Impl, OFFSET>,
            targetOffset: targetOffset::<Identity, Impl, OFFSET>,
            targetRelativeVirtualAddress: targetRelativeVirtualAddress::<Identity, Impl, OFFSET>,
            targetVirtualAddress: targetVirtualAddress::<Identity, Impl, OFFSET>,
            machineType: machineType::<Identity, Impl, OFFSET>,
            oemId: oemId::<Identity, Impl, OFFSET>,
            oemSymbolId: oemSymbolId::<Identity, Impl, OFFSET>,
            get_types: get_types::<Identity, Impl, OFFSET>,
            get_typeIds: get_typeIds::<Identity, Impl, OFFSET>,
            objectPointerType: objectPointerType::<Identity, Impl, OFFSET>,
            udtKind: udtKind::<Identity, Impl, OFFSET>,
            get_undecoratedNameEx: get_undecoratedNameEx::<Identity, Impl, OFFSET>,
            noReturn: noReturn::<Identity, Impl, OFFSET>,
            customCallingConvention: customCallingConvention::<Identity, Impl, OFFSET>,
            noInline: noInline::<Identity, Impl, OFFSET>,
            optimizedCodeDebugInfo: optimizedCodeDebugInfo::<Identity, Impl, OFFSET>,
            notReached: notReached::<Identity, Impl, OFFSET>,
            interruptReturn: interruptReturn::<Identity, Impl, OFFSET>,
            farReturn: farReturn::<Identity, Impl, OFFSET>,
            isStatic: isStatic::<Identity, Impl, OFFSET>,
            hasDebugInfo: hasDebugInfo::<Identity, Impl, OFFSET>,
            isLTCG: isLTCG::<Identity, Impl, OFFSET>,
            isDataAligned: isDataAligned::<Identity, Impl, OFFSET>,
            hasSecurityChecks: hasSecurityChecks::<Identity, Impl, OFFSET>,
            compilerName: compilerName::<Identity, Impl, OFFSET>,
            hasAlloca: hasAlloca::<Identity, Impl, OFFSET>,
            hasSetJump: hasSetJump::<Identity, Impl, OFFSET>,
            hasLongJump: hasLongJump::<Identity, Impl, OFFSET>,
            hasInlAsm: hasInlAsm::<Identity, Impl, OFFSET>,
            hasEH: hasEH::<Identity, Impl, OFFSET>,
            hasSEH: hasSEH::<Identity, Impl, OFFSET>,
            hasEHa: hasEHa::<Identity, Impl, OFFSET>,
            isNaked: isNaked::<Identity, Impl, OFFSET>,
            isAggregated: isAggregated::<Identity, Impl, OFFSET>,
            isSplitted: isSplitted::<Identity, Impl, OFFSET>,
            container: container::<Identity, Impl, OFFSET>,
            inlSpec: inlSpec::<Identity, Impl, OFFSET>,
            noStackOrdering: noStackOrdering::<Identity, Impl, OFFSET>,
            virtualBaseTableType: virtualBaseTableType::<Identity, Impl, OFFSET>,
            hasManagedCode: hasManagedCode::<Identity, Impl, OFFSET>,
            isHotpatchable: isHotpatchable::<Identity, Impl, OFFSET>,
            isCVTCIL: isCVTCIL::<Identity, Impl, OFFSET>,
            isMSILNetmodule: isMSILNetmodule::<Identity, Impl, OFFSET>,
            isCTypes: isCTypes::<Identity, Impl, OFFSET>,
            isStripped: isStripped::<Identity, Impl, OFFSET>,
            frontEndQFE: frontEndQFE::<Identity, Impl, OFFSET>,
            backEndQFE: backEndQFE::<Identity, Impl, OFFSET>,
            wasInlined: wasInlined::<Identity, Impl, OFFSET>,
            strictGSCheck: strictGSCheck::<Identity, Impl, OFFSET>,
            isCxxReturnUdt: isCxxReturnUdt::<Identity, Impl, OFFSET>,
            isConstructorVirtualBase: isConstructorVirtualBase::<Identity, Impl, OFFSET>,
            RValueReference: RValueReference::<Identity, Impl, OFFSET>,
            unmodifiedType: unmodifiedType::<Identity, Impl, OFFSET>,
            framePointerPresent: framePointerPresent::<Identity, Impl, OFFSET>,
            isSafeBuffers: isSafeBuffers::<Identity, Impl, OFFSET>,
            intrinsic: intrinsic::<Identity, Impl, OFFSET>,
            sealed: sealed::<Identity, Impl, OFFSET>,
            hfaFloat: hfaFloat::<Identity, Impl, OFFSET>,
            hfaDouble: hfaDouble::<Identity, Impl, OFFSET>,
            liveRangeStartAddressSection: liveRangeStartAddressSection::<Identity, Impl, OFFSET>,
            liveRangeStartAddressOffset: liveRangeStartAddressOffset::<Identity, Impl, OFFSET>,
            liveRangeStartRelativeVirtualAddress: liveRangeStartRelativeVirtualAddress::<Identity, Impl, OFFSET>,
            countLiveRanges: countLiveRanges::<Identity, Impl, OFFSET>,
            liveRangeLength: liveRangeLength::<Identity, Impl, OFFSET>,
            offsetInUdt: offsetInUdt::<Identity, Impl, OFFSET>,
            paramBasePointerRegisterId: paramBasePointerRegisterId::<Identity, Impl, OFFSET>,
            localBasePointerRegisterId: localBasePointerRegisterId::<Identity, Impl, OFFSET>,
            isLocationControlFlowDependent: isLocationControlFlowDependent::<Identity, Impl, OFFSET>,
            stride: stride::<Identity, Impl, OFFSET>,
            numberOfRows: numberOfRows::<Identity, Impl, OFFSET>,
            numberOfColumns: numberOfColumns::<Identity, Impl, OFFSET>,
            isMatrixRowMajor: isMatrixRowMajor::<Identity, Impl, OFFSET>,
            get_numericProperties: get_numericProperties::<Identity, Impl, OFFSET>,
            get_modifierValues: get_modifierValues::<Identity, Impl, OFFSET>,
            isReturnValue: isReturnValue::<Identity, Impl, OFFSET>,
            isOptimizedAway: isOptimizedAway::<Identity, Impl, OFFSET>,
            builtInKind: builtInKind::<Identity, Impl, OFFSET>,
            registerType: registerType::<Identity, Impl, OFFSET>,
            baseDataSlot: baseDataSlot::<Identity, Impl, OFFSET>,
            baseDataOffset: baseDataOffset::<Identity, Impl, OFFSET>,
            textureSlot: textureSlot::<Identity, Impl, OFFSET>,
            samplerSlot: samplerSlot::<Identity, Impl, OFFSET>,
            uavSlot: uavSlot::<Identity, Impl, OFFSET>,
            sizeInUdt: sizeInUdt::<Identity, Impl, OFFSET>,
            memorySpaceKind: memorySpaceKind::<Identity, Impl, OFFSET>,
            unmodifiedTypeId: unmodifiedTypeId::<Identity, Impl, OFFSET>,
            subTypeId: subTypeId::<Identity, Impl, OFFSET>,
            subType: subType::<Identity, Impl, OFFSET>,
            numberOfModifiers: numberOfModifiers::<Identity, Impl, OFFSET>,
            numberOfRegisterIndices: numberOfRegisterIndices::<Identity, Impl, OFFSET>,
            isHLSLData: isHLSLData::<Identity, Impl, OFFSET>,
            isPointerToDataMember: isPointerToDataMember::<Identity, Impl, OFFSET>,
            isPointerToMemberFunction: isPointerToMemberFunction::<Identity, Impl, OFFSET>,
            isSingleInheritance: isSingleInheritance::<Identity, Impl, OFFSET>,
            isMultipleInheritance: isMultipleInheritance::<Identity, Impl, OFFSET>,
            isVirtualInheritance: isVirtualInheritance::<Identity, Impl, OFFSET>,
            restrictedType: restrictedType::<Identity, Impl, OFFSET>,
            isPointerBasedOnSymbolValue: isPointerBasedOnSymbolValue::<Identity, Impl, OFFSET>,
            baseSymbol: baseSymbol::<Identity, Impl, OFFSET>,
            baseSymbolId: baseSymbolId::<Identity, Impl, OFFSET>,
            objectFileName: objectFileName::<Identity, Impl, OFFSET>,
            isAcceleratorGroupSharedLocal: isAcceleratorGroupSharedLocal::<Identity, Impl, OFFSET>,
            isAcceleratorPointerTagLiveRange: isAcceleratorPointerTagLiveRange::<Identity, Impl, OFFSET>,
            isAcceleratorStubFunction: isAcceleratorStubFunction::<Identity, Impl, OFFSET>,
            numberOfAcceleratorPointerTags: numberOfAcceleratorPointerTags::<Identity, Impl, OFFSET>,
            isSdl: isSdl::<Identity, Impl, OFFSET>,
            isWinRTPointer: isWinRTPointer::<Identity, Impl, OFFSET>,
            isRefUdt: isRefUdt::<Identity, Impl, OFFSET>,
            isValueUdt: isValueUdt::<Identity, Impl, OFFSET>,
            isInterfaceUdt: isInterfaceUdt::<Identity, Impl, OFFSET>,
            findInlineFramesByAddr: findInlineFramesByAddr::<Identity, Impl, OFFSET>,
            findInlineFramesByRVA: findInlineFramesByRVA::<Identity, Impl, OFFSET>,
            findInlineFramesByVA: findInlineFramesByVA::<Identity, Impl, OFFSET>,
            findInlineeLines: findInlineeLines::<Identity, Impl, OFFSET>,
            findInlineeLinesByAddr: findInlineeLinesByAddr::<Identity, Impl, OFFSET>,
            findInlineeLinesByRVA: findInlineeLinesByRVA::<Identity, Impl, OFFSET>,
            findInlineeLinesByVA: findInlineeLinesByVA::<Identity, Impl, OFFSET>,
            findSymbolsForAcceleratorPointerTag: findSymbolsForAcceleratorPointerTag::<Identity, Impl, OFFSET>,
            findSymbolsByRVAForAcceleratorPointerTag: findSymbolsByRVAForAcceleratorPointerTag::<Identity, Impl, OFFSET>,
            get_acceleratorPointerTags: get_acceleratorPointerTags::<Identity, Impl, OFFSET>,
            getSrcLineOnTypeDefn: getSrcLineOnTypeDefn::<Identity, Impl, OFFSET>,
            isPGO: isPGO::<Identity, Impl, OFFSET>,
            hasValidPGOCounts: hasValidPGOCounts::<Identity, Impl, OFFSET>,
            isOptimizedForSpeed: isOptimizedForSpeed::<Identity, Impl, OFFSET>,
            PGOEntryCount: PGOEntryCount::<Identity, Impl, OFFSET>,
            PGOEdgeCount: PGOEdgeCount::<Identity, Impl, OFFSET>,
            PGODynamicInstructionCount: PGODynamicInstructionCount::<Identity, Impl, OFFSET>,
            staticSize: staticSize::<Identity, Impl, OFFSET>,
            finalLiveStaticSize: finalLiveStaticSize::<Identity, Impl, OFFSET>,
            phaseName: phaseName::<Identity, Impl, OFFSET>,
            hasControlFlowCheck: hasControlFlowCheck::<Identity, Impl, OFFSET>,
            constantExport: constantExport::<Identity, Impl, OFFSET>,
            dataExport: dataExport::<Identity, Impl, OFFSET>,
            privateExport: privateExport::<Identity, Impl, OFFSET>,
            noNameExport: noNameExport::<Identity, Impl, OFFSET>,
            exportHasExplicitlyAssignedOrdinal: exportHasExplicitlyAssignedOrdinal::<Identity, Impl, OFFSET>,
            exportIsForwarder: exportIsForwarder::<Identity, Impl, OFFSET>,
            ordinal: ordinal::<Identity, Impl, OFFSET>,
            frameSize: frameSize::<Identity, Impl, OFFSET>,
            exceptionHandlerAddressSection: exceptionHandlerAddressSection::<Identity, Impl, OFFSET>,
            exceptionHandlerAddressOffset: exceptionHandlerAddressOffset::<Identity, Impl, OFFSET>,
            exceptionHandlerRelativeVirtualAddress: exceptionHandlerRelativeVirtualAddress::<Identity, Impl, OFFSET>,
            exceptionHandlerVirtualAddress: exceptionHandlerVirtualAddress::<Identity, Impl, OFFSET>,
            findInputAssemblyFile: findInputAssemblyFile::<Identity, Impl, OFFSET>,
            characteristics: characteristics::<Identity, Impl, OFFSET>,
            coffGroup: coffGroup::<Identity, Impl, OFFSET>,
            bindID: bindID::<Identity, Impl, OFFSET>,
            bindSpace: bindSpace::<Identity, Impl, OFFSET>,
            bindSlot: bindSlot::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiaSymbol as ::windows::core::Interface>::IID
    }
}
pub trait IDiaSymbol10_Impl: Sized + IDiaSymbol_Impl + IDiaSymbol2_Impl + IDiaSymbol3_Impl + IDiaSymbol4_Impl + IDiaSymbol5_Impl + IDiaSymbol6_Impl + IDiaSymbol7_Impl + IDiaSymbol8_Impl + IDiaSymbol9_Impl {
    fn get_sourceLink(&self, cb: u32, pcb: *mut u32, pb: *mut u8) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDiaSymbol10 {}
impl IDiaSymbol10_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol10_Impl, const OFFSET: isize>() -> IDiaSymbol10_Vtbl {
        unsafe extern "system" fn get_sourceLink<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol10_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cb: u32, pcb: *mut u32, pb: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_sourceLink(::core::mem::transmute_copy(&cb), ::core::mem::transmute_copy(&pcb), ::core::mem::transmute_copy(&pb)).into()
        }
        Self {
            base__: IDiaSymbol9_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_sourceLink: get_sourceLink::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiaSymbol10 as ::windows::core::Interface>::IID
            || iid == &<IDiaSymbol as ::windows::core::Interface>::IID
            || iid == &<IDiaSymbol2 as ::windows::core::Interface>::IID
            || iid == &<IDiaSymbol3 as ::windows::core::Interface>::IID
            || iid == &<IDiaSymbol4 as ::windows::core::Interface>::IID
            || iid == &<IDiaSymbol5 as ::windows::core::Interface>::IID
            || iid == &<IDiaSymbol6 as ::windows::core::Interface>::IID
            || iid == &<IDiaSymbol7 as ::windows::core::Interface>::IID
            || iid == &<IDiaSymbol8 as ::windows::core::Interface>::IID
            || iid == &<IDiaSymbol9 as ::windows::core::Interface>::IID
    }
}
pub trait IDiaSymbol2_Impl: Sized + IDiaSymbol_Impl {
    fn isObjCClass(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn isObjCCategory(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
    fn isObjCProtocol(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
}
impl ::windows::core::RuntimeName for IDiaSymbol2 {}
impl IDiaSymbol2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol2_Impl, const OFFSET: isize>() -> IDiaSymbol2_Vtbl {
        unsafe extern "system" fn isObjCClass<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.isObjCClass() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isObjCCategory<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.isObjCCategory() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isObjCProtocol<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.isObjCProtocol() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IDiaSymbol_Vtbl::new::<Identity, Impl, OFFSET>(),
            isObjCClass: isObjCClass::<Identity, Impl, OFFSET>,
            isObjCCategory: isObjCCategory::<Identity, Impl, OFFSET>,
            isObjCProtocol: isObjCProtocol::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiaSymbol2 as ::windows::core::Interface>::IID || iid == &<IDiaSymbol as ::windows::core::Interface>::IID
    }
}
pub trait IDiaSymbol3_Impl: Sized + IDiaSymbol_Impl + IDiaSymbol2_Impl {
    fn inlinee(&self) -> ::windows::core::Result<IDiaSymbol>;
    fn inlineeId(&self) -> ::windows::core::Result<u32>;
}
impl ::windows::core::RuntimeName for IDiaSymbol3 {}
impl IDiaSymbol3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol3_Impl, const OFFSET: isize>() -> IDiaSymbol3_Vtbl {
        unsafe extern "system" fn inlinee<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.inlinee() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn inlineeId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.inlineeId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IDiaSymbol2_Vtbl::new::<Identity, Impl, OFFSET>(),
            inlinee: inlinee::<Identity, Impl, OFFSET>,
            inlineeId: inlineeId::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiaSymbol3 as ::windows::core::Interface>::IID || iid == &<IDiaSymbol as ::windows::core::Interface>::IID || iid == &<IDiaSymbol2 as ::windows::core::Interface>::IID
    }
}
pub trait IDiaSymbol4_Impl: Sized + IDiaSymbol_Impl + IDiaSymbol2_Impl + IDiaSymbol3_Impl {
    fn noexcept(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
}
impl ::windows::core::RuntimeName for IDiaSymbol4 {}
impl IDiaSymbol4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol4_Impl, const OFFSET: isize>() -> IDiaSymbol4_Vtbl {
        unsafe extern "system" fn noexcept<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.noexcept() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IDiaSymbol3_Vtbl::new::<Identity, Impl, OFFSET>(),
            noexcept: noexcept::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiaSymbol4 as ::windows::core::Interface>::IID || iid == &<IDiaSymbol as ::windows::core::Interface>::IID || iid == &<IDiaSymbol2 as ::windows::core::Interface>::IID || iid == &<IDiaSymbol3 as ::windows::core::Interface>::IID
    }
}
pub trait IDiaSymbol5_Impl: Sized + IDiaSymbol_Impl + IDiaSymbol2_Impl + IDiaSymbol3_Impl + IDiaSymbol4_Impl {
    fn hasAbsoluteAddress(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
}
impl ::windows::core::RuntimeName for IDiaSymbol5 {}
impl IDiaSymbol5_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol5_Impl, const OFFSET: isize>() -> IDiaSymbol5_Vtbl {
        unsafe extern "system" fn hasAbsoluteAddress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.hasAbsoluteAddress() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IDiaSymbol4_Vtbl::new::<Identity, Impl, OFFSET>(),
            hasAbsoluteAddress: hasAbsoluteAddress::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiaSymbol5 as ::windows::core::Interface>::IID || iid == &<IDiaSymbol as ::windows::core::Interface>::IID || iid == &<IDiaSymbol2 as ::windows::core::Interface>::IID || iid == &<IDiaSymbol3 as ::windows::core::Interface>::IID || iid == &<IDiaSymbol4 as ::windows::core::Interface>::IID
    }
}
pub trait IDiaSymbol6_Impl: Sized + IDiaSymbol_Impl + IDiaSymbol2_Impl + IDiaSymbol3_Impl + IDiaSymbol4_Impl + IDiaSymbol5_Impl {
    fn isStaticMemberFunc(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
}
impl ::windows::core::RuntimeName for IDiaSymbol6 {}
impl IDiaSymbol6_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol6_Impl, const OFFSET: isize>() -> IDiaSymbol6_Vtbl {
        unsafe extern "system" fn isStaticMemberFunc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.isStaticMemberFunc() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IDiaSymbol5_Vtbl::new::<Identity, Impl, OFFSET>(),
            isStaticMemberFunc: isStaticMemberFunc::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiaSymbol6 as ::windows::core::Interface>::IID || iid == &<IDiaSymbol as ::windows::core::Interface>::IID || iid == &<IDiaSymbol2 as ::windows::core::Interface>::IID || iid == &<IDiaSymbol3 as ::windows::core::Interface>::IID || iid == &<IDiaSymbol4 as ::windows::core::Interface>::IID || iid == &<IDiaSymbol5 as ::windows::core::Interface>::IID
    }
}
pub trait IDiaSymbol7_Impl: Sized + IDiaSymbol_Impl + IDiaSymbol2_Impl + IDiaSymbol3_Impl + IDiaSymbol4_Impl + IDiaSymbol5_Impl + IDiaSymbol6_Impl {
    fn isSignRet(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
}
impl ::windows::core::RuntimeName for IDiaSymbol7 {}
impl IDiaSymbol7_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol7_Impl, const OFFSET: isize>() -> IDiaSymbol7_Vtbl {
        unsafe extern "system" fn isSignRet<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.isSignRet() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IDiaSymbol6_Vtbl::new::<Identity, Impl, OFFSET>(),
            isSignRet: isSignRet::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiaSymbol7 as ::windows::core::Interface>::IID || iid == &<IDiaSymbol as ::windows::core::Interface>::IID || iid == &<IDiaSymbol2 as ::windows::core::Interface>::IID || iid == &<IDiaSymbol3 as ::windows::core::Interface>::IID || iid == &<IDiaSymbol4 as ::windows::core::Interface>::IID || iid == &<IDiaSymbol5 as ::windows::core::Interface>::IID || iid == &<IDiaSymbol6 as ::windows::core::Interface>::IID
    }
}
pub trait IDiaSymbol8_Impl: Sized + IDiaSymbol_Impl + IDiaSymbol2_Impl + IDiaSymbol3_Impl + IDiaSymbol4_Impl + IDiaSymbol5_Impl + IDiaSymbol6_Impl + IDiaSymbol7_Impl {
    fn coroutineKind(&self) -> ::windows::core::Result<u32>;
    fn associatedSymbolKind(&self) -> ::windows::core::Result<u32>;
    fn associatedSymbolSection(&self) -> ::windows::core::Result<u32>;
    fn associatedSymbolOffset(&self) -> ::windows::core::Result<u32>;
    fn associatedSymbolRva(&self) -> ::windows::core::Result<u32>;
    fn associatedSymbolAddr(&self) -> ::windows::core::Result<u64>;
}
impl ::windows::core::RuntimeName for IDiaSymbol8 {}
impl IDiaSymbol8_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol8_Impl, const OFFSET: isize>() -> IDiaSymbol8_Vtbl {
        unsafe extern "system" fn coroutineKind<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.coroutineKind() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn associatedSymbolKind<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.associatedSymbolKind() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn associatedSymbolSection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.associatedSymbolSection() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn associatedSymbolOffset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.associatedSymbolOffset() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn associatedSymbolRva<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.associatedSymbolRva() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn associatedSymbolAddr<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.associatedSymbolAddr() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IDiaSymbol7_Vtbl::new::<Identity, Impl, OFFSET>(),
            coroutineKind: coroutineKind::<Identity, Impl, OFFSET>,
            associatedSymbolKind: associatedSymbolKind::<Identity, Impl, OFFSET>,
            associatedSymbolSection: associatedSymbolSection::<Identity, Impl, OFFSET>,
            associatedSymbolOffset: associatedSymbolOffset::<Identity, Impl, OFFSET>,
            associatedSymbolRva: associatedSymbolRva::<Identity, Impl, OFFSET>,
            associatedSymbolAddr: associatedSymbolAddr::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiaSymbol8 as ::windows::core::Interface>::IID || iid == &<IDiaSymbol as ::windows::core::Interface>::IID || iid == &<IDiaSymbol2 as ::windows::core::Interface>::IID || iid == &<IDiaSymbol3 as ::windows::core::Interface>::IID || iid == &<IDiaSymbol4 as ::windows::core::Interface>::IID || iid == &<IDiaSymbol5 as ::windows::core::Interface>::IID || iid == &<IDiaSymbol6 as ::windows::core::Interface>::IID || iid == &<IDiaSymbol7 as ::windows::core::Interface>::IID
    }
}
pub trait IDiaSymbol9_Impl: Sized + IDiaSymbol_Impl + IDiaSymbol2_Impl + IDiaSymbol3_Impl + IDiaSymbol4_Impl + IDiaSymbol5_Impl + IDiaSymbol6_Impl + IDiaSymbol7_Impl + IDiaSymbol8_Impl {
    fn framePadSize(&self) -> ::windows::core::Result<u32>;
    fn framePadOffset(&self) -> ::windows::core::Result<u32>;
    fn isRTCs(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BOOL>;
}
impl ::windows::core::RuntimeName for IDiaSymbol9 {}
impl IDiaSymbol9_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol9_Impl, const OFFSET: isize>() -> IDiaSymbol9_Vtbl {
        unsafe extern "system" fn framePadSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.framePadSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn framePadOffset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.framePadOffset() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isRTCs<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaSymbol9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::Win32::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.isRTCs() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IDiaSymbol8_Vtbl::new::<Identity, Impl, OFFSET>(),
            framePadSize: framePadSize::<Identity, Impl, OFFSET>,
            framePadOffset: framePadOffset::<Identity, Impl, OFFSET>,
            isRTCs: isRTCs::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiaSymbol9 as ::windows::core::Interface>::IID
            || iid == &<IDiaSymbol as ::windows::core::Interface>::IID
            || iid == &<IDiaSymbol2 as ::windows::core::Interface>::IID
            || iid == &<IDiaSymbol3 as ::windows::core::Interface>::IID
            || iid == &<IDiaSymbol4 as ::windows::core::Interface>::IID
            || iid == &<IDiaSymbol5 as ::windows::core::Interface>::IID
            || iid == &<IDiaSymbol6 as ::windows::core::Interface>::IID
            || iid == &<IDiaSymbol7 as ::windows::core::Interface>::IID
            || iid == &<IDiaSymbol8 as ::windows::core::Interface>::IID
    }
}
pub trait IDiaTable_Impl: Sized + ::windows::Win32::System::Com::IEnumUnknown_Impl {
    fn Skip2(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset2(&self) -> ::windows::core::Result<()>;
    fn Clone2(&self) -> ::windows::core::Result<::windows::Win32::System::Com::IEnumUnknown>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn name(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn Item(&self, index: u32) -> ::windows::core::Result<::windows::core::IUnknown>;
}
impl ::windows::core::RuntimeName for IDiaTable {}
impl IDiaTable_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaTable_Impl, const OFFSET: isize>() -> IDiaTable_Vtbl {
        unsafe extern "system" fn Skip2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip2(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset2().into()
        }
        unsafe extern "system" fn Clone2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone2() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiaTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, element: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(element, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::Win32::System::Com::IEnumUnknown_Vtbl::new::<Identity, Impl, OFFSET>(),
            Skip2: Skip2::<Identity, Impl, OFFSET>,
            Reset2: Reset2::<Identity, Impl, OFFSET>,
            Clone2: Clone2::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            name: name::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiaTable as ::windows::core::Interface>::IID || iid == &<::windows::Win32::System::Com::IEnumUnknown as ::windows::core::Interface>::IID
    }
}
