use std::io::Write;
use std::path::Path;

use itertools::Itertools;

pub fn generate_zshenv(
    prefix: impl AsRef<Path>,
    dot_dir: impl AsRef<Path>,
    code_dir: impl AsRef<Path>,
    local_dir: impl AsRef<Path>,
    bin_dir: impl AsRef<Path>,
    rc_file: Option<&str>,
) {
    let dot_dir = dot_dir.as_ref();
    let code_dir = code_dir.as_ref();
    let local_dir = local_dir.as_ref();
    let bin_dir = bin_dir.as_ref();

    let shell_dir = dot_dir.join("shell");
    let sh_dir = shell_dir.join("common");
    let zsh_dir = shell_dir.join("zsh");

    let export_paths = [
        ("DOTDIR", dot_dir),
        ("CODEDIR", code_dir),
        ("LOCALDIR", local_dir),
        ("BINDIR", bin_dir),
        ("SHDIR", &sh_dir),
        ("ZDOTDIR", &zsh_dir),
    ]
    .into_iter()
    .map(|(var, path)| format!("export {}={}", var, path.to_str().unwrap()))
    .collect_vec();

    let zshenv_path = prefix.as_ref().join(rc_file.unwrap_or(".zshenv"));
    let zshenv_content = [
        ["# AUTO GENERATED FILE. DO NOT EDIT".to_string(), "".to_string()].as_slice(),
        export_paths.as_slice(),
        ["".to_string()].as_slice(),
    ]
    .concat()
    .join("\n");
    log::debug!(path:? = zshenv_path; "Generating zshenv");
    log::trace!(content:% = zshenv_content; "Generating zshenv");

    std::fs::File::options()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&zshenv_path)
        .unwrap()
        .write_all(zshenv_content.as_bytes())
        .unwrap();
}
