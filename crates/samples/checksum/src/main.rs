use microsoft_dia::{nsNone, DiaSource, IDiaDataSource, SymTagCompiland};
use windows::{
    core::*,
    Win32::System::Com::{CoInitializeEx, COINIT_MULTITHREADED},
};

fn main() -> Result<()> {
    unsafe {
        // Initialize COM and DIA
        CoInitializeEx(None, COINIT_MULTITHREADED).ok()?;
        let source: IDiaDataSource = microsoft_dia::helpers::NoRegCoCreate(
            s!(
                r#"C:\Program Files\Microsoft Visual Studio\2022\Enterprise\DIA SDK\bin\amd64\msdia140.dll"#
            ),
            &DiaSource,
        )?;

        // Open session against own symbols
        let executable = std::env::current_exe().unwrap();
        source.loadDataForExe(&HSTRING::from(executable.as_os_str()), None, None)?;
        let session = source.openSession()?;

        // Get compilands
        let symbols =
            session
                .globalScope()?
                .findChildren(SymTagCompiland, None, nsNone.0 as u32)?;

        // Get source files
        for _i in 0..symbols.Count()? {
            let files = session.findFile(None, PCWSTR::null(), nsNone.0 as u32)?;

            // Find files with a checksum and print out details
            for j in 0..files.Count()? {
                let file = files.Item(j as u32)?;
                if file.checksumType()? == 0 {
                    continue;
                }

                let mut byte_count = 0u32;
                file.get_checksum(&mut byte_count, None)?;

                let mut bytes = vec![0; byte_count as usize];
                file.get_checksum(&mut byte_count, Some(&mut bytes))?;

                println!("File: {}", file.fileName()?);
                println!(
                    "{:02x}: {}\n",
                    file.checksumType()?,
                    bytes
                        .iter()
                        .map(|b| format!("{:02x}", b).to_string())
                        .collect::<String>()
                );
            }
        }

        Ok(())
    }
}
