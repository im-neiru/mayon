use cbindgen::{Braces, Builder, Language, Style};

fn main() {
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    Builder::new()
        .with_crate(crate_dir)
        .with_language(Language::C)
        .with_include_guard("MAYON_API_H")
        .with_style(Style::Both)
        .with_cpp_compat(true)
        .with_braces(Braces::SameLine)
        .generate()
        .expect("Unable to generate c-bindings for Mayon")
        .write_to_file("include/mayon.h");
}
