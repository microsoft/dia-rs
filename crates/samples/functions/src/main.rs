use microsoft_dia::{DataSource, NameSearchOptions, SymTag};
use windows_core::*;

fn main() -> Result<()> {
    let source = DataSource::open()?;

    let executable = std::env::current_exe().unwrap();
    source.load_exe(executable.to_str().unwrap(), None)?;

    let session = source.open_session()?;
    let symbols = session.global_scope()?.find_children(
        SymTag::Function,
        "sample_functions::*",
        NameSearchOptions::WILDCARD,
    )?;

    println!(
        "Function symbols found in sample_functions::* ({}):",
        executable.to_string_lossy()
    );

    for sym in symbols {
        println!("\t{}", sym.name()?);
    }

    Ok(())
}
