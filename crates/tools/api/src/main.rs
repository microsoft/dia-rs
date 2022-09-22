use std::io::Write;
use windows_metadata::reader::{File, Reader, Tree};

fn main() {
    let start = std::time::Instant::now();
    let mut output_path = std::path::PathBuf::from("");

    output_path.push("src/Microsoft");
    let _ = std::fs::remove_dir_all(&output_path);
    output_path.pop();

    let winmd_files = [File::new(".windows/winmd/Microsoft.Dia.winmd").unwrap(), File::new(".windows/winmd/Windows.Win32.winmd").unwrap(), File::new(".windows/winmd/Windows.Win32.Interop.winmd").unwrap()];
    let reader = Reader::new(&winmd_files);
    let root = reader.tree("Microsoft", &[]).expect("Microsoft namespace not found");

    let trees = root.flatten();
    trees.iter().for_each(|tree| gen_tree(&reader, &output_path, tree));

    output_path.pop();
    println!("Elapsed: {} ms", start.elapsed().as_millis());
}

fn gen_tree(reader: &Reader, output: &std::path::Path, tree: &Tree) {
    let mut path = std::path::PathBuf::from(output);

    path.push(tree.namespace.replace('.', "/"));
    std::fs::create_dir_all(&path).unwrap();

    let mut gen = windows_bindgen::Gen::new(reader);
    gen.namespace = tree.namespace;
    gen.cfg = false;

    let mut tokens = windows_bindgen::namespace(&gen, tree);
    tokens.push_str(r#"#[cfg(feature = "implement")] ::core::include!("impl.rs");"#);
    fmt_tokens(tree.namespace, &mut tokens);
    std::fs::write(path.join("mod.rs"), tokens).unwrap();

    let mut tokens = windows_bindgen::namespace_impl(&gen, tree);
    fmt_tokens(tree.namespace, &mut tokens);
    std::fs::write(path.join("impl.rs"), tokens).unwrap();
}

fn fmt_tokens(namespace: &str, tokens: &mut String) {
    let mut child = std::process::Command::new("rustfmt").stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).stderr(std::process::Stdio::null()).spawn().expect("Failed to spawn `rustfmt`");

    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    stdin.write_all(tokens.as_bytes()).unwrap();
    drop(stdin);
    let output = child.wait_with_output().unwrap();

    if output.status.success() {
        *tokens = String::from_utf8(output.stdout).expect("Failed to parse UTF-8");
    } else {
        println!("** {} - rustfmt failed", namespace);
    }
}
