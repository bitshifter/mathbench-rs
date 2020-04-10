use cargo_toml::Manifest;
use clap::{App, Arg};
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::HashSet,
    convert::{TryFrom, TryInto},
    env,
    error::Error,
    fs::{self, File},
    io::{self, BufRead, Cursor, Write},
    process::Command,
    str::FromStr,
};
use tempfile::{tempdir, TempDir};

#[derive(Debug, Default)]
struct TimingInfo {
    name: String,
    profile: Profile,
    features: Features,
    total_time: f64,
    self_time: f64,
    units: u32,
}

#[derive(Debug, Copy, Clone)]
enum Profile {
    Dev,
    Release,
}

impl Default for Profile {
    fn default() -> Self {
        Profile::Release
    }
}

impl TryFrom<&str> for Profile {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "dev" => Ok(Profile::Dev),
            "release" => Ok(Profile::Release),
            _ => Err(format!("Unknown profile \"{}\" found!", value)),
        }
    }
}

impl Profile {
    fn possible_values() -> &'static [&'static str] {
        &["dev", "release"]
    }

    fn get_flags(&self) -> Option<&str> {
        match self {
            Profile::Dev => None,
            Profile::Release => Some("--release"),
        }
    }
    fn as_str(&self) -> &str {
        match self {
            Profile::Dev => "dev",
            Profile::Release => "release",
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Features {
    Defaults,
    NoDefaults,
}

impl Default for Features {
    fn default() -> Self {
        Features::Defaults
    }
}

impl TryFrom<&str> for Features {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "defaults" => Ok(Features::Defaults),
            "no-defaults" => Ok(Features::NoDefaults),
            _ => Err(format!("Unknown feature {} found!", value)),
        }
    }
}

impl Features {
    fn possible_values() -> &'static [&'static str] {
        &["defaults", "no-defaults"]
    }
    fn as_str(&self) -> &str {
        match self {
            Features::Defaults => "defaults",
            Features::NoDefaults => "no-defaults",
        }
    }
    fn get_toml_value(&self) -> Option<&str> {
        match self {
            Features::Defaults => None,
            Features::NoDefaults => Some("default-features = false"),
        }
    }
}

fn create_temp_build(name: &str, version: &str, features: Features) -> io::Result<TempDir> {
    let dir = tempdir()?;

    let toml_path = dir.path().join("Cargo.toml");
    let mut file = File::create(toml_path)?;
    writeln!(file, "[package]")?;
    writeln!(file, "name = \"mathbench-{}-buildtime\"", name)?;
    writeln!(file, "version = \"0.1.0\"")?;
    writeln!(file, "edition = \"2018\"")?;
    writeln!(file, "[dependencies.{}]", name)?;
    writeln!(file, "version = \"{}\"", version)?;
    if let Some(toml_value) = features.get_toml_value() {
        writeln!(file, "{}", toml_value)?;
    }
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

fn bench_crate(
    name: &str,
    version: &str,
    profile: Profile,
    features: Features,
    verbose: bool,
) -> io::Result<TimingInfo> {
    let dir = create_temp_build(name, version, features)?;

    let mut args = vec!["+nightly", "build", "-Z", "timings=html,info"];
    if let Some(profile_flags) = profile.get_flags() {
        args.push(profile_flags);
    }

    let output = Command::new("cargo")
        .current_dir(dir.path())
        .args(&["+nightly", "build", "--release", "-Z", "timings=html,info"])
        .output()?;

    if !output.status.success() {
        return Err(io::Error::new(io::ErrorKind::Other, "Build failed."));
    }

    let mut timing_info = TimingInfo::default();
    timing_info.name = name.to_string();
    timing_info.profile = profile;
    timing_info.features = features;

    lazy_static! {
        static ref COMPLETED_MATCH: Regex =
            Regex::new(r"^\s*Completed ([\S]+) ([\S]+) in ([\S]+)").unwrap();
        static ref FINISHED_MATCH: Regex =
            Regex::new(r"\s*Finished [\S]+ \[.+\] target\(s\) in ([\S]+)").unwrap();
    }

    for line in Cursor::new(output.stderr).lines() {
        let line = line?;
        if verbose {
            println!("{}", line);
        }
        if let Some(completed_captures) = COMPLETED_MATCH.captures(line.as_str()) {
            timing_info.units += 1;
            let completed_name = completed_captures.get(1).unwrap().as_str();
            if name == completed_name {
                timing_info.self_time = parse_time(completed_captures.get(3).unwrap().as_str());
            }
        } else if let Some(finished_captures) = FINISHED_MATCH.captures(line.as_str()) {
            timing_info.total_time = parse_time(finished_captures.get(1).unwrap().as_str());
        }
    }

    // copy timing file
    let current_dir = env::current_dir()?;
    let timing_html = format!(
        "cargo-timing-{}-{}-{}.html",
        name,
        profile.as_str(),
        features.as_str()
    );
    fs::copy(
        dir.path().join("cargo-timing.html"),
        current_dir.join(timing_html),
    )?;

    Ok(timing_info)
}

fn summarize(profiles: &[Profile], features: &[Features], results: &[TimingInfo]) {
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

    let mut titles = vec![Cell::new_align("crate", Alignment::LEFT)];
    if profiles.len() > 1 {
        titles.push(Cell::new_align("profile", Alignment::LEFT));
    }
    if features.len() > 1 {
        titles.push(Cell::new_align("features", Alignment::LEFT));
    }
    titles.push(Cell::new_align("total (s)", Alignment::RIGHT));
    titles.push(Cell::new_align("self (s)", Alignment::RIGHT));
    titles.push(Cell::new_align("units", Alignment::RIGHT));
    table.set_titles(Row::new(titles));

    for info in results {
        let mut row = vec![Cell::new_align(info.name.as_str(), Alignment::LEFT)];
        if profiles.len() > 1 {
            row.push(Cell::new_align(info.profile.as_str(), Alignment::LEFT));
        }
        if features.len() > 1 {
            row.push(Cell::new_align(info.features.as_str(), Alignment::LEFT));
        }
        row.push(Cell::new_align(
            &format!("{:.1}", info.total_time),
            Alignment::RIGHT,
        ));
        row.push(Cell::new_align(
            &format!("{:.1}", info.self_time),
            Alignment::RIGHT,
        ));
        row.push(Cell::new_align(
            &format!("{}", info.units),
            Alignment::RIGHT,
        ));
        table.add_row(Row::new(row));
    }

    table.printstd();
}

fn main() -> Result<(), Box<dyn Error>> {
    let manifest = Manifest::from_path("Cargo.toml")?;

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
        .args(&[
            Arg::with_name("lib")
                .takes_value(true)
                .multiple(true)
                .required(false)
                .possible_values(&possible_libs),
            Arg::with_name("features")
                .long("features")
                .short("F")
                .takes_value(true)
                .multiple(true)
                .possible_values(Features::possible_values()),
            Arg::with_name("profiles")
                .long("profile")
                .short("P")
                .takes_value(true)
                .multiple(true)
                .possible_values(Profile::possible_values()),
            Arg::with_name("verbose").long("verbose").short("v"),
        ])
        .get_matches();

    let allowed_libs: HashSet<&str> = matches
        .values_of("lib")
        .map_or(HashSet::new(), |v| v.collect());

    let profiles: Vec<Profile> = if let Some(profiles) = matches.values_of("profiles") {
        profiles.map(|v| v.try_into().unwrap()).collect()
    } else {
        vec![Profile::default()]
    };

    let features: Vec<Features> = if let Some(features) = matches.values_of("features") {
        features.map(|v| v.try_into().unwrap()).collect()
    } else {
        vec![Features::default()]
    };

    let verbose = matches.is_present("verbose");

    let mut results = Vec::new();
    for (name, version) in lib_pairs {
        if allowed_libs.is_empty() || allowed_libs.contains(name) {
            for profile in &profiles {
                for feature in &features {
                    println!(
                        "Building {} {} {} profile {}",
                        name,
                        version,
                        profile.as_str(),
                        feature.as_str()
                    );
                    match bench_crate(name, version, *profile, *feature, verbose) {
                        Ok(info) => results.push(info),
                        Err(e) => {
                            eprintln!(
                                "Error building {} {} {}: {:?}",
                                name,
                                profile.as_str(),
                                feature.as_str(),
                                e
                            );
                        }
                    }
                }
            }
        }
    }

    if !results.is_empty() {
        summarize(&profiles, &features, &results);
    }

    // TODO: output error if any build failed
    Ok(())
}
