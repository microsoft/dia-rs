use std::{fs, path::PathBuf, process::Command};
use windows_metadata::reader::File;

fn main() {
    let output_path = PathBuf::from("src/bindings.rs");
    if output_path.exists() {
        fs::remove_file(&output_path).unwrap();
    }

    let files = File::with_default(&[".windows/winmd/Microsoft.Dia.winmd"]).unwrap();
    fs::write(
        &output_path,
        windows_bindgen::component("Microsoft.Dia", &files),
    )
    .unwrap();

    let mut child = Command::new("rustfmt")
        .args([&output_path])
        .spawn()
        .expect("Failed to start rustfmt");

    child.wait().expect("rustfmt failed");
}
