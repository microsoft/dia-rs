use microsoft_dia::{nsfRegularExpression, DiaSource, IDiaDataSource, SymTagFunction};
use windows::Win32::System::Com::{CoInitializeEx, COINIT_MULTITHREADED};

fn main() -> windows::core::Result<()> {
    unsafe {
        CoInitializeEx(std::ptr::null_mut(), COINIT_MULTITHREADED)?;
        let source: IDiaDataSource = microsoft_dia::helpers::NoRegCoCreate(s!("msdia140.dll"), &DiaSource)?;
        let executable = std::env::current_exe().unwrap();
        source.loadDataForExe(executable.as_os_str(), None, None)?;
        let session = source.openSession()?;
        let symbols = session.globalScope()?.findChildren(SymTagFunction, "sample_functions::*", nsfRegularExpression.0 as u32)?;

        println!("Function symbols found in sample_functions::* ({}):", &executable.to_string_lossy());

        for i in 0..symbols.Count()? {
            println!("\t{}", symbols.Item(i as u32)?.name()?);
        }

        Ok(())
    }
}
