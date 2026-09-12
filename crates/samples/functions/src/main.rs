use microsoft_dia::{DiaSource, IDiaDataSource, SymTagFunction, nsfRegularExpression};
use windows_core::*;

fn main() -> Result<()> {
    unsafe {
        let source: IDiaDataSource = create_dia_source().inspect_err(|_| {
            eprintln!(
                "Failed to create DIA data source. Make sure msdia140.dll is \
            registered or present next to the executable."
            );
        })?;

        let executable = std::env::current_exe().unwrap();
        source
            .loadDataForExe(&HSTRING::from(executable.as_os_str()), None, None)
            .ok()?;
        let session = source.openSession()?;
        let symbols = session.globalScope()?.findChildren(
            SymTagFunction,
            w!("sample_functions::*"),
            nsfRegularExpression as u32,
        )?;

        println!(
            "Function symbols found in sample_functions::* ({}):",
            &executable.to_string_lossy()
        );

        for i in 0..symbols.Count()? {
            println!("\t{}", symbols.Item(i as u32)?.name()?.display());
        }

        Ok(())
    }
}

fn create_dia_source() -> Result<IDiaDataSource> {
    let paths = [
        s!(
            r#"C:\Program Files\Microsoft Visual Studio\18\Insiders\DIA SDK\bin\amd64\msdia140.dll"#
        ),
        s!(
            r#"C:\Program Files\Microsoft Visual Studio\18\Professional\DIA SDK\bin\amd64\msdia140.dll"#
        ),
        s!(
            r#"C:\Program Files\Microsoft Visual Studio\18\Enterprise\DIA SDK\bin\amd64\msdia140.dll"#
        ),
        s!(
            r#"C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\DIA SDK\bin\amd64\msdia140.dll"#
        ),
        s!(r#"C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\msdia140.dll"#),
        s!("msdia140.dll"),
    ];

    let mut last_error = Error::empty();
    for path in paths {
        match unsafe { microsoft_dia::helpers::NoRegCoCreate(path, &DiaSource) } {
            Ok(source) => return Ok(source),
            Err(e) => last_error = e,
        }
    }

    Err(last_error)
}
