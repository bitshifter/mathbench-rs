use cargo_toml::Manifest;
use clap::{App, Arg};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    env,
    fs::{self, File},
    io::{self, BufRead, Cursor, Write},
    process::{Command, Stdio},
};
use tempfile::{tempdir, TempDir};

#[derive(Debug, Serialize, Deserialize)]
struct Target {
    crate_types: Vec<String>,
    doctest: bool,
    edition: String,
    kind: Vec<String>,
    name: String,
    src_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TimingInfo {
    duration: f64,
    mode: String,
    package_id: String,
    reason: String,
    rmeta_time: f64,
    target: Target,
}

fn create_temp_dir(name: &str, version: &str) -> io::Result<TempDir> {
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

    Ok(dir)
}

fn build_crate(target_name: &str, dir: TempDir) -> io::Result<Vec<u8>> {
    let output = Command::new("cargo")
        .current_dir(dir.path())
        .args(&["+nightly", "build", "--release", "-Z", "timings=html,info,json"])
        .stderr(Stdio::inherit())
        .output()?;

    let path = env::current_dir()?;
    let timing_html = format!("cargo-timing-{}.html", target_name);
    fs::copy(dir.path().join("cargo-timing.html"), path.join(timing_html))?;

    Ok(output.stdout)
}

fn collect_timing_info(output: Vec<u8>) -> io::Result<Vec<TimingInfo>> {
    let mut timing_infos = Vec::new();
    for line in Cursor::new(output).lines() {
        let line = line.unwrap();
        dbg!(&line);
        let timing_info: TimingInfo = serde_json::from_str(&line)?;
        timing_infos.push(timing_info);
    }
    Ok(timing_infos)
}

fn summarize_timing_infos(target_name: &str, timing_infos: &[TimingInfo]) {
    let total_units = timing_infos.len();
    let mut total_time = 0.0;
    let mut self_time = 0.0;
    for timing_info in timing_infos {
        println!("{:?}", timing_info);
        if timing_info.target.name == target_name {
            self_time = timing_info.duration;
            total_time = timing_info.rmeta_time + timing_info.duration;
        }
    }
    println!(
        "{} total time: {}, self time: {}, units {}",
        target_name, total_time, self_time, total_units
    );
}

fn benchlib(target_name: &str, target_version: &str) -> io::Result<()> {
    let dir = create_temp_dir(target_name, target_version)?;

    let output = build_crate(target_name, dir)?;

    let timing_infos = collect_timing_info(output)?;

    summarize_timing_infos(target_name, &timing_infos);

    Ok(())
}

fn main() {
    let manifest = Manifest::from_path("Cargo.toml").unwrap();

    let lib_pairs: Vec<(&str, &str)> = manifest
        .dependencies
        .iter()
        .filter_map(|(name, dep)| {
            // assume that detailed dependencies are the libs we want
            if let Some(detail) = dep.detail() {
                Some((
                    name.as_str(),
                    detail
                        .version
                        .as_ref()
                        .expect("dependency version expected")
                        .as_str(),
                ))
            } else {
                None
            }
        })
        .collect();

    let possible_libs: Vec<&str> = lib_pairs.iter().map(|&(name, _)| name).collect();
    let matches = App::new("mathbench buildbench")
        .about("Benchmarks build times of supported mathbench libraries.")
        .arg(
            Arg::with_name("lib")
                .takes_value(true)
                .multiple(true)
                .required(false)
                .possible_values(&possible_libs),
        )
        .get_matches();

    let allowed_libs: HashSet<&str> = matches
        .values_of("lib")
        .map_or(HashSet::new(), |v| v.collect());

    for (name, version) in lib_pairs {
        if allowed_libs.is_empty() || allowed_libs.contains(name) {
            println!("Building {} {}", name, version);
            benchlib(name, version).unwrap();
        }
    }
}
