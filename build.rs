use std::process::Command;

#[allow(dead_code)]
fn main() {
    let project_root = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let input_css_path: String = [&project_root, "/css/input.css"].iter().map(|s| *s).collect();
    let output_css_path: String = [&project_root, "/css/output.css"].iter().map(|s| *s).collect();

    Command::new("npx")
        .args(["tailwindcss", "-i", &input_css_path, "-o", &output_css_path])
        .output()
        .expect("Failed to build tailwind");
}
