use std::collections::HashMap;
// use std::fmt::format;
use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::str::FromStr;

use walkdir::WalkDir;

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
enum PackageType {
    BioConductor,
    DevTools,
    Remote,
    Normal,
    Unique,
}

impl PackageType {
    fn as_str(&self) -> &'static str {
        match self {
            PackageType::BioConductor => "BiocManager::install",
            PackageType::DevTools => "devtools::install_github",
            PackageType::Remote => "remotes::install_github",
            PackageType::Normal => "install.packages",
            PackageType::Unique => "",
        }
    }
}

impl FromStr for PackageType {
    type Err = ();

    fn from_str(input: &str) -> Result<PackageType, Self::Err> {
        match input {
            x if x.contains("install.packages") => Ok(PackageType::BioConductor),
            x if x.contains("BiocManager") => Ok(PackageType::DevTools),
            x if x.contains("devtools") => Ok(PackageType::Remote),
            x if x.contains("remotes") => Ok(PackageType::Normal),
            _ => Ok(PackageType::Unique),
        }
    }
}

fn process_r_pkgs(script: Vec<String>) -> HashMap<PackageType, Vec<String>> {
    let r_packages: HashMap<PackageType, Vec<String>> = script
        .iter()
        .filter(|x| x.contains("R"))
        .map(|item| {
            let split_content: Vec<_> = item.split(|c: char| c == '(' || c == ')').collect();
            let parsed_item = split_content.get(1).unwrap_or(&"");
            let matched_item = parsed_item.replace("'", "");
            let package_type = PackageType::from_str(item).unwrap();
            (package_type, matched_item)
        })
        .fold(HashMap::new(), |mut acc, (key, val)| {
            let item = acc.entry(key).or_insert(Vec::new());
            if !item.contains(&val) {
                item.push(val)
            };
            acc
        });
    return r_packages;
}

fn write_r_pkgs(pkgs: HashMap<PackageType, Vec<String>>) {
    let install_method = "RUN R --no-save -e";
    let path = "Dockerfile";
    let mut output = File::create(path).expect("couldn't create the file");
    for (pkg_type, pkg_map) in pkgs.iter() {
        for chunk in pkg_map.chunks(10) {
            let pkg_name = chunk
                .iter()
                .map(|x| format!("'{}'", x))
                .collect::<Vec<String>>()
                .join(", ");
            write!(
                output,
                "{} \"{}(c({}))\"\n",
                install_method,
                pkg_type.as_str(),
                pkg_name
            )
            .expect("Package failed to write");
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let default_path = ".".to_string();
    let dir_path = args.get(1).unwrap_or(&default_path);
    let walk = WalkDir::new(dir_path).max_depth(2);
    let filenames: Vec<_> = walk
        .into_iter()
        .filter(|f| f.is_ok())
        .map(|f| f.unwrap())
        .collect();
    let data: Vec<_> = filenames
        .iter()
        .map(|p| p.path())
        .filter(|p| p.is_file() && p.extension().unwrap_or_default() == "sh")
        .map(|content| fs::read_to_string(content).unwrap_or_default())
        .collect();
    let parsed_data: Vec<_> = data
        .iter()
        .flat_map(|l| l.split("\n"))
        .filter(|s| !s.is_empty() && s.contains("RUN"))
        .map(|x| {
            x.replace("&&", "").split_whitespace()
                .filter(|&x| x != "RUN")
                .collect::<Vec<&str>>()
                .join(" ")
        })
        .collect();
    let r_pkgs = process_r_pkgs(parsed_data);
    write_r_pkgs(r_pkgs);
}
