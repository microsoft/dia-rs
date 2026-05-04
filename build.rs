use std::{env::var, fs::read_to_string, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=exclusions.txt");

    let combined = std::env::join_paths(
        std::env::split_paths(&var("INCLUDE").unwrap_or_default())
            .chain([PathBuf::from(".windows/inc")]),
    )
    .unwrap();
    std::env::set_var("INCLUDE", combined);

    let exclusions = read_to_string("exclusions.txt").unwrap_or_default();
    windows_rdl::clang()
        .args(["-x", "c++"])
        .library("msdia140.dll")
        .input_str(&format!(
            r#"
            {exclusions}
            #include <windows.h>
            #include "cvconst.h"
            #include "diacreate.h"
            #include "dia2.h"
            "#
        ))
        .input(r#".windows/winmd/Windows.Win32.winmd"#)
        .filter("diacreate.h")
        .filter("cvconst.h")
        .filter("dia2.h")
        .output("src/Microsoft.Dia.rdl")
        .namespace("Microsoft.Dia")
        .write()
        .unwrap();

    windows_rdl::Reader::new()
        .input("src/Microsoft.Dia.rdl")
        .input(".windows/winmd/Windows.Win32.winmd")
        .output(".windows/winmd/Microsoft.Dia.winmd")
        .write()
        .unwrap();

    windows_bindgen::bindgen([
        "--in",
        ".windows/winmd/Windows.winmd",
        ".windows/winmd/Windows.Win32.winmd",
        ".windows/winmd/Microsoft.Dia.winmd",
        "--out",
        "src/public_bindings.rs",
        "--flat",
        "--typedef",
        "--filter",
        "Microsoft.Dia",
        "--reference",
        "windows,skip-root,Windows",
    ])
    .unwrap();

    windows_bindgen::bindgen([
        "--in",
        ".windows/winmd/Windows.Win32.winmd",
        ".windows/winmd/Windows.winmd",
        "--out",
        "src/helper_bindings.rs",
        "--flat",
        "--filter",
        "IClassFactory",
        "LoadLibraryExW",
        "GetProcAddress",
        "LOAD_WITH_ALTERED_SEARCH_PATH",
    ])
    .unwrap();
}
