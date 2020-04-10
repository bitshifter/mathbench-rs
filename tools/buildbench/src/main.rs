use cargo_toml::Manifest;
use clap::{App, Arg};
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::HashSet,
    env,
    fs::{self, File},
    io::{self, BufRead, BufReader, Error, ErrorKind, Write},
    process::{Command, Stdio},
    str::FromStr,
};
use tempfile::{tempdir, TempDir};

#[derive(Debug, Default)]
struct TimingInfo {
    name: String,
    profile: String,
    total_time: f64,
    self_time: f64,
    units: u32,
}

fn create_temp_dir(name: &str, version: &str) -> io::Result<TempDir> {
    let dir = tempdir()?;

    let toml_path = dir.path().join("Cargo.toml");
    let mut file = File::create(toml_path)?;
    writeln!(file, "[package]")?;
    writeln!(file, "name = \"mathbench-{}-buildtime\"", name)?;
    writeln!(file, "version = \"0.1.0\"")?;
    // writeln!(
    //     file,
    //     "authors = [\"Cameron Hart <cameron.hart@gmail.com>\"]"
    // )?;
    writeln!(file, "edition = \"2018\"")?;
    writeln!(file, "[dependencies]")?;
    writeln!(file, "{} = \"{}\"", name, version)?;
    drop(file);

    let src_dir_path = dir.path().join("src");
    fs::create_dir(&src_dir_path)?;

    let src_path = src_dir_path.join("lib.rs");
    let mut file = File::create(src_path)?;
    writeln!(file)?;

    Ok(dir)
}

fn parse_time(time: &str) -> f64 {
    let time_match = Regex::new(r"^(\d+\.\d+)s$").unwrap();
    let completed_time = time_match.captures(time).unwrap().get(1).unwrap().as_str();
    f64::from_str(completed_time).unwrap()
}

fn build_crate(name: &str, dir: TempDir) -> io::Result<TimingInfo> {
    let stderr = Command::new("cargo")
        .current_dir(dir.path())
        .args(&["+nightly", "build", "--release", "-Z", "timings=html,info"])
        .stderr(Stdio::piped())
        .spawn()?
        .stderr
        .ok_or_else(|| Error::new(ErrorKind::Other, "Could not capture standard error."))?;

    let reader = BufReader::new(stderr);

    let mut timing_info = TimingInfo::default();
    timing_info.name = name.to_string();

    lazy_static! {
        static ref COMPLETED_MATCH: Regex =
            Regex::new(r"^\s*Completed ([\S]+) ([\S]+) in ([\S]+)").unwrap();
        static ref FINISHED_MATCH: Regex =
            Regex::new(r"\s*Finished ([\S]+) \[.+\] target\(s\) in ([\S]+)").unwrap();
    }

    for line in reader.lines() {
        let line = line?;
        println!("{}", line);
        if let Some(completed_captures) = COMPLETED_MATCH.captures(line.as_str()) {
            timing_info.units += 1;
            let completed_name = completed_captures.get(1).unwrap().as_str();
            if name == completed_name {
                timing_info.self_time = parse_time(completed_captures.get(3).unwrap().as_str());
            }
        } else if let Some(finished_captures) = FINISHED_MATCH.captures(line.as_str()) {
            timing_info.profile = finished_captures.get(1).unwrap().as_str().to_string();
            timing_info.total_time = parse_time(finished_captures.get(2).unwrap().as_str());
        }
    }

    // copy timing file
    let current_dir = env::current_dir()?;
    let timing_html = format!("cargo-timing-{}.html", name);
    fs::copy(
        dir.path().join("cargo-timing.html"),
        current_dir.join(timing_html),
    )?;

    Ok(timing_info)
}

fn benchlib(target_name: &str, target_version: &str) -> io::Result<TimingInfo> {
    let dir = create_temp_dir(target_name, target_version)?;
    build_crate(target_name, dir)
}

fn summarize(results: &[TimingInfo]) {
    use prettytable::{
        format::{Alignment, FormatBuilder, LinePosition, LineSeparator},
        Cell, Row, Table,
    };

    let markdown_format = FormatBuilder::new()
        .padding(1, 1)
        .borders('|')
        .separator(LinePosition::Title, LineSeparator::new('-', '|', '|', '|'))
        .column_separator('|')
        .build();

    let mut table = Table::new();
    table.set_format(markdown_format);

    table.set_titles(Row::new(vec![
        Cell::new_align("name", Alignment::LEFT),
        Cell::new_align("total (s)", Alignment::RIGHT),
        Cell::new_align("self (s)", Alignment::RIGHT),
        Cell::new_align("units", Alignment::RIGHT),
    ]));

    for info in results {
        table.add_row(Row::new(vec![
            Cell::new_align(info.name.as_str(), Alignment::LEFT),
            Cell::new_align(&format!("{:.1}", info.total_time), Alignment::RIGHT),
            Cell::new_align(&format!("{:.1}", info.self_time), Alignment::RIGHT),
            Cell::new_align(&format!("{}", info.units), Alignment::RIGHT),
        ]));
    }

    table.printstd();
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

    let results: Vec<TimingInfo> = lib_pairs
        .iter()
        .filter_map(|(name, version)| {
            if allowed_libs.is_empty() || allowed_libs.contains(name) {
                println!("Building {} {}", name, version);
                benchlib(name, version).ok()
            } else {
                None
            }
        })
        .collect();

    summarize(&results);
}
