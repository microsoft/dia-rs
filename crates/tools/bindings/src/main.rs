fn main() {
    windows_clang::clang()
        .args(["-x", "c++", "--target=x86_64-pc-windows-msvc"])
        .input_text(
            r#"
            #include <windows.h>
            #include "../../../../../DIA SDK/include/cvconst.h"
            #include "../../../../../DIA SDK/include/dia2.h"
            #include "../../../../../DIA SDK/include/diacreate.h"
        "#,
        )
        .reference_default()
        .filter("cvconst.h")
        .filter("dia2.h")
        .filter("diacreate.h")
        .output("rdl/dia.rdl")
        .namespace("Microsoft.Dia")
        .library("msdia140.dll")
        .write()
        .unwrap();

    windows_rdl::reader()
        .input("rdl/dia.rdl")
        .reference_default()
        .output("winmd/Microsoft.Dia.winmd")
        .write()
        .unwrap();

    windows_bindgen::bindgen([
        "--in",
        "winmd/Microsoft.Dia.winmd",
        "--in",
        "default",
        "--out",
        "src/bindings.rs",
        "--flat",
        "--filter",
        "GetProcAddress",
        "IClassFactory",
        "LoadLibraryExA",
        "LOAD_WITH_ALTERED_SEARCH_PATH",
        "Microsoft.Dia",
    ]);
}
