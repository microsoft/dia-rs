#![allow(non_snake_case, clippy::missing_safety_doc)]

use windows::core::*;
use windows::Win32::System::Com::IClassFactory;
use windows::Win32::{
    Foundation::HANDLE,
    System::LibraryLoader::{GetProcAddress, LoadLibraryExA, LOAD_WITH_ALTERED_SEARCH_PATH},
};

type DllGetClassObject = unsafe extern "system" fn(*const GUID, *const GUID, *mut *mut std::ffi::c_void) -> HRESULT;

/// Creates a single object of a class associated with a specified CLSID
/// present in the specified dynamic-link library-based COM server.
///
/// This function is used as an alternative to `CoCreateInstance` which
/// requires the server implementing the specified class to be registered
/// prior to use.
///
pub unsafe fn NoRegCoCreate<T: Interface>(lib: PCSTR, rclsid: *const GUID) -> Result<T> {
    let instance = LoadLibraryExA(lib, HANDLE::default(), LOAD_WITH_ALTERED_SEARCH_PATH)?;
    if !instance.is_invalid() {
        if let Some(farproc) = GetProcAddress(instance, s!("DllGetClassObject")) {
            let get_class_object: DllGetClassObject = std::mem::transmute(farproc);
            let mut factory: Option<IClassFactory> = None;
            if get_class_object(rclsid, &IClassFactory::IID, &mut factory as *mut _ as *mut _).is_ok() {
                return factory.unwrap().CreateInstance(None);
            }
        }
    }

    Err(Error::from_win32())
}
