use std::io::Write;
use std::path::Path;

use itertools::Itertools;

pub fn generate_zshenv<
    P: AsRef<Path>,
    PD: AsRef<Path>,
    PC: AsRef<Path>,
    PL: AsRef<Path>,
    PB: AsRef<Path>,
>(
    prefix: P,
    dot_dir: PD,
    code_dir: PC,
    local_dir: PL,
    bin_dir: PB,
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

    let zshenv_path = prefix.as_ref().join(".zshenv");
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
        .expect("can not open .zshenv file to write")
        .write_all(zshenv_content.as_bytes())
        .expect("can not write to .zshenv file");
}
