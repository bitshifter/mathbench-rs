use cargo_toml::Manifest;
use std::{
    fs::{self, File},
    io::{self, Write},
    process::Command,
};
use tempfile::tempdir;

fn benchlib(name: &str, version: &str) -> io::Result<()> {
    let dir = tempdir()?;

    let toml_path = dir.path().join("Cargo.toml");
    let mut file = File::create(toml_path)?;
    writeln!(file, "[package]")?;
    writeln!(file, "name = \"mathbench-{}-buildtime\"", name)?;
    writeln!(file, "version = \"0.1.0\"")?;
    writeln!(
        file,
        "authors = [\"Cameron Hart <cameron.hart@gmail.com>\"]"
    )?;
    writeln!(file, "edition = \"2018\"")?;
    writeln!(file, "[dependencies]")?;
    writeln!(file, "{} = \"{}\"", name, version)?;
    drop(file);

    let src_dir_path = dir.path().join("src");
    fs::create_dir(&src_dir_path)?;

    let src_path = src_dir_path.join("lib.rs");
    let mut file = File::create(src_path)?;
    writeln!(file, "")?;
    drop(file);

    let status = Command::new("cargo")
        .current_dir(dir.path())
        .args(&["+nightly", "build", "--release", "-Z", "timings=html,info,json"])
        .status()?;

    assert!(status.success());

    Ok(())
}

fn main() {
    let manifest = Manifest::from_path("Cargo.toml").unwrap();

    let libs: Vec<(&String, &String)> = manifest
        .dependencies
        .iter()
        .filter_map(|(name, dep)| {
            // assume that detailed dependencies are the libs we want
            if let Some(detail) = dep.detail() {
                Some((
                    name,
                    detail
                        .version
                        .as_ref()
                        .expect("dependency version expected"),
                ))
            } else {
                None
            }
        })
        .collect();

    for (name, version) in libs {
        benchlib(name, version).unwrap();
    }
}
