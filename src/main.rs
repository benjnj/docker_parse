use docker_parse::parser::{parse_r_pkg, process_files};
use std::collections::HashMap;
use std::fs::File;
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
            x if x.contains("BiocManager") => Ok(PackageType::BioConductor),
            x if x.contains("devtools") => Ok(PackageType::DevTools),
            x if x.contains("remotes") || x.contains("github") => Ok(PackageType::Remote),
            x if x.contains("install.packages") => Ok(PackageType::Normal),
            _ => Ok(PackageType::Unique),
        }
    }
}

fn process_r_pkgs(script: Vec<String>) -> HashMap<PackageType, Vec<String>> {
    let r_packages: HashMap<PackageType, Vec<String>> = script
        .iter()
        .filter(|x| x.contains("R"))
        .map(|item| {
            let matched_item = match parse_r_pkg(item) {
                Ok(x) => x.1,
                Err(_) => Vec::new(),
            };
            let package_type = PackageType::from_str(item).unwrap();
            (package_type, matched_item)
        })
        .fold(HashMap::new(), |mut acc, (key, val)| {
            let item = acc.entry(key).or_insert(Vec::new());
            for pkg in val.iter() {
                if !item.contains(&pkg) && !pkg.is_empty() {
                    item.push(pkg.to_owned())
                };
            }
            acc
        });
    return r_packages;
}

fn write_r_pkgs(pkgs: HashMap<PackageType, Vec<String>>) -> std::io::Result<()> {
    let install_method = "RUN R --no-save -e";
    let path = "Dockerfile";
    const NUM_OF_PKGS: usize = 10;
    let mut output = File::create(path)?;
    for (pkg_type, pkg_map) in pkgs.iter() {
        for chunk in pkg_map.chunks(NUM_OF_PKGS) {
            let full_pkg = chunk
                .iter()
                .filter(|&x| x.contains("="))
                .map(|s| s.to_owned())
                .collect::<Vec<String>>();
            for full in full_pkg.iter() {
                write!(
                    output,
                    "{} \"{}({})\"\n",
                    install_method,
                    pkg_type.as_str(),
                    full
                )?
            }
            let pkg_name = chunk
                .iter()
                .filter(|&x| !x.contains("="))
                .map(|x| format!("'{}'", x))
                .collect::<Vec<String>>()
                .join(", ");
            write!(
                output,
                "{} \"{}(c({}))\"\n",
                install_method,
                pkg_type.as_str(),
                pkg_name
            )?;
        }
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    let parsed_data = process_files("sh");
    let r_pkgs = process_r_pkgs(parsed_data);
    write_r_pkgs(r_pkgs)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_working() {
        let test_case = parse_r_pkg("R --no-save -e \"install.packages('ggtree')\"");
        assert_eq!(test_case,Ok(("install.packages",vec!["ggtree".to_string()])));
    }
}
