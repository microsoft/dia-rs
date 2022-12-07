use std::{
    fs,
    io::Write,
    path::PathBuf,
    process::{Command, Stdio},
};
use windows_metadata::reader::File;

fn main() {
    let files = [
        File::new(".windows/winmd/Microsoft.Dia.winmd").unwrap(),
        File::new(".windows/winmd/Windows.Win32.winmd").unwrap(),
        File::new(".windows/winmd/Windows.Win32.Interop.winmd").unwrap(),
    ];

    let output_path = PathBuf::from("src/bindings.rs");
    if output_path.exists() {
        fs::remove_file(&output_path).unwrap();
    }

    let mut generated_tokens = windows_bindgen::component("Microsoft.Dia", &files);
    fmt_tokens(&mut generated_tokens);

    fs::write(output_path, generated_tokens).unwrap();
}

fn fmt_tokens(tokens: &mut String) {
    let mut child = Command::new("rustfmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to spawn `rustfmt`");

    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    stdin.write_all(tokens.as_bytes()).unwrap();
    drop(stdin);

    let output = child.wait_with_output().unwrap();
    if output.status.success() {
        *tokens = String::from_utf8(output.stdout).expect("Failed to parse UTF-8");
    } else {
        println!("rustfmt failed");
    }
}
