use std::collections::HashMap;
use std::path::Path;

use convert_case::Casing;

fn main() {
    built::write_built_file().expect("Failed to acquire build-time information");

    let version_path = Path::new("config").join("binary").join("version.toml");
    println!("cargo::rerun-if-changed={}", version_path.to_str().unwrap());
    let versions: HashMap<String, String> = toml::from_str(
        &std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join(version_path)).unwrap(),
    )
    .unwrap();

    std::fs::write(
        Path::new(&std::env::var("OUT_DIR").unwrap()).join("install-config.rs"),
        "use clap::ValueEnum;\n\n".to_string()
            + "#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, ValueEnum)]\n\n"
            + "pub enum InstallConfig {\n"
            + &versions
                .keys()
                .map(|k| k.to_case(convert_case::Case::Pascal) + ",\n")
                .collect::<Vec<_>>()
                .join("")
            + "}\n",
    )
    .unwrap();
}
