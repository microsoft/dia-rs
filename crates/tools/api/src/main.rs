use std::io::Write;
use windows_metadata::reader::{TypeReader, TypeTree};

fn main() {
    let start = std::time::Instant::now();
    let mut output_path = std::path::PathBuf::from("");

    output_path.push("src/Microsoft");
    let _ = std::fs::remove_dir_all(&output_path);
    output_path.pop();

    let reader = TypeReader::get();
    let root = reader.types.get_namespace("Microsoft").unwrap();

    let mut trees = Vec::new();
    collect_subtrees(&output_path, root, &mut trees);

    trees.iter().for_each(|tree| gen_tree(&output_path, tree));

    output_path.pop();
    println!("Elapsed: {} ms", start.elapsed().as_millis());
}

fn collect_subtrees<'a>(output: &std::path::Path, tree: &'a TypeTree, trees: &mut Vec<&'a TypeTree>) {
    trees.push(tree);

    tree.namespaces.values().for_each(|tree| collect_subtrees(output, tree, trees));

    let mut path = std::path::PathBuf::from(output);
    path.push(tree.namespace.replace('.', "/"));
    std::fs::create_dir_all(&path).unwrap();
}

fn gen_tree(output: &std::path::Path, tree: &TypeTree) {
    let mut path = std::path::PathBuf::from(output);

    path.push(tree.namespace.replace('.', "/"));

    let gen = windows_bindgen::Gen {
        namespace: tree.namespace,
        min_xaml: true,
        windows_extern: true,
        cfg: false,
        ..Default::default()
    };
    let mut tokens = windows_bindgen::gen_namespace(&gen);
    tokens.push_str(r#"#[cfg(feature = "implement")] ::core::include!("impl.rs");"#);
    fmt_tokens(tree.namespace, &mut tokens);
    std::fs::write(path.join("mod.rs"), tokens).unwrap();

    let mut tokens = windows_bindgen::gen_namespace_impl(&gen);
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
