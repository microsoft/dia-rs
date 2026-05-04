use microsoft_dia::{DiaSource, IDiaDataSource, IDiaSession};
use windows_core::*;

pub fn get_test_session() -> Result<IDiaSession> {
    unsafe {
        let vs_install_dir =
            std::env::var("VSINSTALLDIR").expect("VSINSTALLDIR environment variable not set");

        let dll_relative = if cfg!(target_arch = "x86_64") {
            r"DIA SDK\bin\amd64\msdia140.dll"
        } else if cfg!(target_arch = "aarch64") {
            r"DIA SDK\bin\arm64\msdia140.dll"
        } else if cfg!(target_arch = "x86") {
            r"DIA SDK\bin\msdia140.dll"
        } else {
            panic!("Unsupported target architecture");
        };

        let dll_path = HSTRING::from(format!("{}{}", vs_install_dir, dll_relative));
        let source: IDiaDataSource =
            microsoft_dia::helpers::NoRegCoCreate(PCWSTR(dll_path.as_ptr()), &DiaSource)?;

        let executable = std::env::current_exe().unwrap();
        source.loadDataForExe(
            HSTRING::from(executable.as_os_str()).as_ptr(),
            std::ptr::null(),
            None,
        )?;
        source.openSession()
    }
}
