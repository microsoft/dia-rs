use microsoft_dia::{DiaSource, IDiaDataSource, IDiaSession};
use windows_core::*;

pub fn get_test_session() -> Result<IDiaSession> {
    unsafe {
        let paths = if cfg!(target_arch = "x86_64") {
            [
                s!(
                    r"C:\Program Files\Microsoft Visual Studio\18\Insiders\DIA SDK\bin\amd64\msdia140.dll"
                ),
                s!(
                    r"C:\Program Files\Microsoft Visual Studio\18\Professional\DIA SDK\bin\amd64\msdia140.dll"
                ),
                s!(
                    r"C:\Program Files\Microsoft Visual Studio\18\Enterprise\DIA SDK\bin\amd64\msdia140.dll"
                ),
                s!(
                    r"C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\DIA SDK\bin\amd64\msdia140.dll"
                ),
                s!(r"C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\msdia140.dll"),
                s!(r"msdia140.dll"),
            ]
        } else if cfg!(target_arch = "aarch64") {
            [
                s!(
                    r"C:\Program Files\Microsoft Visual Studio\18\Insiders\DIA SDK\bin\arm64\msdia140.dll"
                ),
                s!(
                    r"C:\Program Files\Microsoft Visual Studio\18\Professional\DIA SDK\bin\arm64\msdia140.dll"
                ),
                s!(
                    r"C:\Program Files\Microsoft Visual Studio\18\Enterprise\DIA SDK\bin\arm64\msdia140.dll"
                ),
                s!(
                    r"C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\DIA SDK\bin\arm64\msdia140.dll"
                ),
                s!(r"C:\Program Files (x86)\Windows Kits\10\Debuggers\arm64\msdia140.dll"),
                s!(r"msdia140.dll"),
            ]
        } else if cfg!(target_arch = "x86") {
            [
                s!(
                    r"C:\Program Files\Microsoft Visual Studio\18\Insiders\DIA SDK\bin\msdia140.dll"
                ),
                s!(
                    r"C:\Program Files\Microsoft Visual Studio\18\Professional\DIA SDK\bin\msdia140.dll"
                ),
                s!(
                    r"C:\Program Files\Microsoft Visual Studio\18\Enterprise\DIA SDK\bin\msdia140.dll"
                ),
                s!(
                    r"C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\DIA SDK\bin\msdia140.dll"
                ),
                s!(r"C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\msdia140.dll"),
                s!(r"msdia140.dll"),
            ]
        } else {
            panic!("Unsupported target architecture");
        };

        let mut last_error = Error::empty();
        for path in paths {
            match microsoft_dia::helpers::NoRegCoCreate::<IDiaDataSource>(path, &DiaSource) {
                Ok(source) => {
                    let executable = std::env::current_exe().unwrap();
                    source
                        .loadDataForExe(&HSTRING::from(executable.as_os_str()), None, None)
                        .ok()?;
                    return source.openSession();
                }
                Err(e) => last_error = e,
            }
        }
        Err(last_error)
    }
}
