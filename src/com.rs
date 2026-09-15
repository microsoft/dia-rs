//! COM plumbing for creating DIA objects from `msdia140.dll` without
//! COM registration.

use crate::bindings::*;
use windows_core::*;

type DllGetClassObject =
    unsafe extern "system" fn(*const GUID, *const GUID, *mut *mut core::ffi::c_void) -> HRESULT;

/// Creates a single object of a class associated with a specified CLSID
/// present in the specified dynamic-link library-based COM server.
///
/// This is an alternative to `CoCreateInstance` which requires the server
/// implementing the specified class to be registered prior to use.
///
/// `lib` is the path to `msdia140.dll`. It is loaded (without COM
/// registration) and `DllGetClassObject` is invoked to obtain an
/// `IClassFactory`, which is then used to create the requested class.
pub(crate) fn no_reg_co_create<T: Interface>(lib: &str, rclsid: &GUID) -> Result<T> {
    unsafe {
        let cstr = std::ffi::CString::new(lib).map_err(|_| Error::from_thread())?;
        let instance = LoadLibraryExA(
            PCSTR::from_raw(cstr.as_ptr() as *const u8),
            core::ptr::null_mut(),
            LOAD_WITH_ALTERED_SEARCH_PATH as u32,
        );
        if instance.is_null() {
            return Err(Error::from_thread());
        }

        let farproc = match GetProcAddress(instance, s!("DllGetClassObject")) {
            Some(fp) => fp,
            None => return Err(Error::from_thread()),
        };
        let get_class_object: DllGetClassObject = core::mem::transmute(farproc);

        let mut factory: Option<IClassFactory> = None;
        if get_class_object(
            rclsid,
            &IClassFactory::IID,
            &mut factory as *mut _ as *mut _,
        )
        .is_ok()
        {
            return factory.unwrap().CreateInstance(None);
        }
        Err(Error::from_thread())
    }
}
