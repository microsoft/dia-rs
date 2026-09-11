use microsoft_dia::{DiaSource, IDiaDataSource, SymTagFunction, nsfRegularExpression};
use windows_core::*;

fn main() -> Result<()> {
    unsafe {
        let source: IDiaDataSource =
            microsoft_dia::helpers::NoRegCoCreate(s!("msdia140.dll"), &DiaSource).inspect_err(
                |e| {
                    eprintln!(
                        "Failed to create DIA data source ({e}). Make sure msdia140.dll is \
                    registered or present next to the executable. It can be found at \
                    <VS Root>\\DIA SDK\\bin\\amd64."
                    );
                },
            )?;

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
