use microsoft_dia::{DiaSource, IDiaDataSource, IDiaSession};
use windows_core::*;

pub fn get_test_session() -> Result<IDiaSession> {
    unsafe {
        let path = if cfg!(target_arch = "x86_64") {
            s!(
                r"C:\Program Files\Microsoft Visual Studio\2022\Enterprise\DIA SDK\bin\amd64\msdia140.dll"
            )
        } else if cfg!(target_arch = "aarch64") {
            s!(
                r"C:\Program Files\Microsoft Visual Studio\2022\Enterprise\DIA SDK\bin\arm64\msdia140.dll"
            )
        } else if cfg!(target_arch = "x86") {
            s!(r"C:\Program Files\Microsoft Visual Studio\2022\Enterprise\DIA SDK\bin\msdia140.dll")
        } else {
            panic!("Unsupported target architecture");
        };
        let source: IDiaDataSource = microsoft_dia::helpers::NoRegCoCreate(path, &DiaSource)?;
        let executable = std::env::current_exe().unwrap();
        source.loadDataForExe(&HSTRING::from(executable.as_os_str()), None, None)?;
        source.openSession()
    }
}
