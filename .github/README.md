[![crates.io](https://img.shields.io/crates/v/microsoft-dia.svg)](https://crates.io/crates/microsoft-dia)

## Rust for Debug Interface Access (DIA) SDK

The Microsoft Debug Interface Access (DIA) SDK provides access to the debug
information stored in program database (`.pdb`) files produced by the
Microsoft toolchain. `microsoft-dia` wraps it in an idiomatic Rust API:
`Result<T>` returns, safe callback traits, and `Iterator`s over enumerators.

Start by adding `windows-core` and `microsoft-dia` to `Cargo.toml`:

```toml
[dependencies.windows-core]
version = "0.100"

[dependencies.microsoft-dia]
version = "0.16"
```

### Open a data source and walk symbols

```rust,no_run
use microsoft_dia::{DataSource, NameSearchOptions, SymTag};
use windows_core::*;

fn main() -> Result<()> {
    // Creates the DIA source (no COM registration lookup needed at the call
    // site; the DLL is resolved from the well-known locations).
    let source = DataSource::open()
        .inspect_err(|_| eprintln!("Failed to create a DIA data source"))?;

    // Load the debug info for the running executable (path as `&str`).
    let executable = std::env::current_exe().unwrap();
    source.load_exe(executable.to_str().unwrap(), None)?;

    // Open a session and get the global-scope symbol.
    let session = source.open_session()?;
    let scope = session.global_scope()?;

    // Find all function symbols, then iterate the enumerator.
    let functions = scope.find_children(
        SymTag::Function,
        "sample_functions::*",
        NameSearchOptions::WILDCARD,
    )?;

    println!("Function symbols in self ({}):", executable.to_string_lossy());
    for sym in functions {
        println!("\t{}", sym.name()?);
    }

    Ok(())
}
```

The full public surface (records, enumerators, stack walking, load callbacks,
and the `SourceFile::checksum`/`SourceFile::checksum_type` getters used for
source-file integrity checks) is documented on each type. See the
`crates/samples/functions` sample for a working, runnable example.
