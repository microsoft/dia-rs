fn main() {
    println!("cargo:rerun-if-changed=.windows/winmd/Microsoft.Dia.winmd");
    println!("cargo:rerun-if-changed=build.rs");

    windows_bindgen::bindgen([
        "--in",
        "default",
        ".windows/winmd/Microsoft.Dia.winmd",
        "--out",
        "src/bindings.rs",
        "--filter",
        "Microsoft.Dia",
    ]);
}
