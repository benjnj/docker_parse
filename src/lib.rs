pub mod parser {
    use nom::bytes::complete::{is_not, tag, take_till};
    use nom::character::complete::{alphanumeric0, char};
    use nom::combinator::opt;
    use nom::sequence::{delimited, preceded, tuple};
    use nom::IResult;
    use std::env;
    use std::fs;
    use walkdir::WalkDir;

    pub fn parse_r_pkg(pkg: &str) -> IResult<&str, Vec<String>> {
        let (input, _) = tag("R ")(pkg)?;
        let (input, _) = preceded(opt(tag("--no-save ")), tag("-e "))(input)?;
        let (_, output) = delimited(char('"'), is_not("\""), char('"'))(input)?;
        let (input, name) = take_till(|c| c == '(')(output)?;
        let (_, output) = delimited(char('('), is_not(")"), char(')'))(input)?;
        let (input, _) = tuple((alphanumeric0, opt(char('('))))(output)?;
        if input.contains("=") {
            return Ok(("full", vec![input.into()]));
        }
        let text: String = input
            .split(",")
            .collect::<Vec<_>>()
            .join(" ")
            .chars()
            .filter(|&c| c != '\'')
            .collect();
        let pkgs = text.split(" ").map(|s| s.to_owned()).collect();
        Ok((name, pkgs))
    }
    pub fn process_files(extension: &str) -> Vec<String> {
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
            .filter(|p| p.is_file() && p.extension().unwrap_or_default() == extension)
            .map(|content| fs::read_to_string(content).unwrap_or_default())
            .collect();
        let parsed_data: Vec<_> = data
            .iter()
            .flat_map(|l| l.split("\n"))
            .filter(|s| !s.is_empty() && s.contains("RUN"))
            .map(|x| {
                x.replace("&&", "")
                    .split_whitespace()
                    .filter(|&x| x != "RUN")
                    .collect::<Vec<&str>>()
                    .join(" ")
            })
            .collect();
        return parsed_data;
    }
}
