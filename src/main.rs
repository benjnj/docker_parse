use std::collections::{HashMap, HashSet};
// use std::fmt::format;
use std::fs::{self, File};
use std::io::Write;
use std::str::FromStr;

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

// #[derive(Debug)]
// struct RPackage {
//     package_name: String,
//     package_type: PackageType,
// }

fn process_r_pkgs(script: Vec<String>) -> HashMap<PackageType, HashSet<String>> {
    let r_packages: HashMap<PackageType, HashSet<String>> = script
        .iter()
        .filter(|x| x.contains("R"))
        .map(|item| {
            let split_content: Vec<_> = item.split(|c: char| c == '(' || c == ')').collect();
            let parsed_item = split_content.get(1).unwrap_or(&"Not Available");
            let matched_item = parsed_item.replace("'", "");
            let package_type = PackageType::from_str(item).unwrap();
            (package_type, matched_item)
        })
        .fold(HashMap::new(), |mut acc, (key, val)| {
            acc.entry(key).or_insert(HashSet::new()).insert(val);
            acc
        });
    return r_packages;
}

fn write_r_pkgs(pkgs: HashMap<PackageType, HashSet<String>>) {
    let install_method = "RUN R --no-save -e";
    let path = "Dockerfile";
    let mut output = File::create(path).expect("couldn't create the file");
    for (pkg_type, pkg_map) in pkgs.iter() {
        let mut chunks = pkg_map.iter().peekable();
        while chunks.peek().is_some() {
            let chunk_vec: Vec<&String> = chunks.by_ref().take(5).collect();
            let pkg_name = chunk_vec
                .iter()
                .map(|&x| format!("'{}'", x))
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
    let file = fs::read_to_string("chbv_debug.sh").expect("no file found");
    let content: Vec<String> = file
        .lines()
        .filter(|s| s.contains("RUN"))
        .map(|x| {
            x.split_whitespace()
                .filter(|&x| x != "RUN")
                .collect::<Vec<&str>>()
                .join(" ")
        })
        .collect();
    let r_pkgs = process_r_pkgs(content);
    println!("{:?}", r_pkgs);
    write_r_pkgs(r_pkgs);
}
