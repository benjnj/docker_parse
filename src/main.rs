use std::fs::{self, File};
use std::io::Write;

#[derive(Debug)]
enum PackageType {
    BioConductor,
    DevTools,
    Remote,
    Normal,
    Unique,
}

#[derive(Debug)]
struct RPackage {
    package_name: String,
    package_type: PackageType,
}

fn process_r_pkgs(script: Vec<String>) -> Vec<RPackage> {
    let r_packages: Vec<RPackage> = script
        .iter()
        .filter(|x| x.contains("R"))
        .map(|item| {
            let split_content: Vec<_> = item.split(|c: char| c == '(' || c == ')').collect();
            let parsed_item = split_content.get(1).unwrap_or(&"Not Available");
            let matched_item = parsed_item.replace("'", "");
            let package_type = match item {
                x if x.contains("install.packages") => PackageType::Normal,
                x if x.contains("BiocManager") => PackageType::BioConductor,
                x if x.contains("devtools") => PackageType::DevTools,
                x if x.contains("remotes") => PackageType::Remote,
                _ => PackageType::Unique,
            };
            RPackage {
                package_name: matched_item,
                package_type,
            }
        })
        .collect();
    return r_packages;
}

fn write_r_pkgs(pkgs: &Vec<RPackage>) {
    let install_method = "RUN R --no-save -e";
    let path = "Dockerfile";
    let mut output = File::create(path).expect("couldn't create the file");
    let instructions = pkgs.iter().map(|pkg| {
        match pkg {
            RPackage {
                package_name,
                package_type: PackageType::Remote,
            } => {
                return format!(
                    "{} remotes::install_github('{}')\n",
                    install_method, package_name
                )
            }
            RPackage {
                package_name,
                package_type: PackageType::BioConductor,
            } => {
                return format!(
                    "{} BiocManager::install('{}')\n",
                    install_method, package_name
                )
            }
            RPackage {
                package_name,
                package_type: PackageType::DevTools,
            } => {
                return format!(
                    "{} devtools::install_github('{}')\n",
                    install_method, package_name
                )
            }
            RPackage {
                package_name,
                package_type: PackageType::Normal,
            } => {
                return format!("{} install.packages('{}')\n", install_method, package_name)
            }
            _ => "".to_string(),
        }
    }).collect::<Vec<String>>();
    for row in instructions.iter() {
        write!(output,"{}", row).expect("Package failed to write");
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
    write_r_pkgs(&r_pkgs);
}
