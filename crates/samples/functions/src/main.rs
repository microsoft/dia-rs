use microsoft_dia::{nsfRegularExpression, DiaSource, IDiaDataSource, SymTagFunction};
use windows_core::*;

fn main() -> Result<()> {
    unsafe {
        let source: IDiaDataSource = microsoft_dia::helpers::NoRegCoCreate(
            s!(
                r#"C:\Program Files\Microsoft Visual Studio\2022\Enterprise\DIA SDK\bin\amd64\msdia140.dll"#
            ),
            &DiaSource,
        )?;
        let executable = std::env::current_exe().unwrap();
        source.loadDataForExe(&HSTRING::from(executable.as_os_str()), None, None)?;
        let session = source.openSession()?;
        let symbols = session.globalScope()?.findChildren(
            SymTagFunction,
            w!("sample_functions::*"),
            nsfRegularExpression.0 as u32,
        )?;

        println!(
            "Function symbols found in sample_functions::* ({}):",
            &executable.to_string_lossy()
        );

        for i in 0..symbols.Count()? {
            println!("\t{}", symbols.Item(i as u32)?.name()?);
        }

        Ok(())
    }
}
