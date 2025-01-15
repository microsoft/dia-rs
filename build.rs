fn main() {
    println!("cargo:rerun-if-changed=.windows/winmd/Microsoft.Dia.winmd");
    println!("cargo:rerun-if-changed=build.rs");

    windows_bindgen::bindgen([
        "--in",
        "default",
        ".windows/winmd/Microsoft.Dia.winmd",
        "--out",
        "src/public_bindings.rs",
        "--flat",
        "--filter",
        "Microsoft.Dia",
        "--reference",
        "windows,skip-root,Windows"
    ]);

    windows_bindgen::bindgen([
        "--in",
        "default",
        "--out",
        "src/helper_bindings.rs",
        "--flat",
        "--filter",
        "IClassFactory",
        "LoadLibraryExA",
        "GetProcAddress",
        "LOAD_WITH_ALTERED_SEARCH_PATH",
    ]);
}
