use std::{fs::File, io::Read};

use clap::Parser;
use sitesmith::parser::{cli::CliArgs, htmlreader::HtmlWeaver, projectreader::ProjectParse, workreader::WorkParse};


fn main() -> std::io::Result<()> {
    let args = CliArgs::parse();

    let proj_fmt = "<div class='project-item'><h3>{name}</h3><ul>{descr}</ul>{extra}</div>";
    let work_fmt = "<div class='experience-item'><h3>{name}</h3><p>{location} | {timespan}</p><ul>{descr}</ul>{extra}</div>";

    let mut proj_str = String::new();
    let mut work_str = String::new();
    
    let mut template = String::new();

    File::open(args.projects)?.read_to_string(&mut proj_str)?;
    File::open(args.work)?.read_to_string(&mut work_str)?;
    
    File::open(args.template)?.read_to_string(&mut template)?;

    if let Ok(proj_entry) = serde_json::from_str(&proj_str) {

        if let Ok(work_entry) = serde_json::from_str(&work_str) {

            let proj_parse = ProjectParse::from_entries(proj_entry, proj_fmt.to_string());
            let work_parse = WorkParse::from_entries(work_entry, work_fmt.to_string());

            let weaver = HtmlWeaver::new(template, proj_parse, work_parse);

            if let Err(error) = weaver.generate(&args.output) {
                eprintln!("There was an issue generating your HTML file :(\nError: {}", error);
            }
        } else {

            eprintln!("Error reading Work Experience JSON file, please check https://github.com/BradenEverson/sitesmith for a concise example of JSON format")
        }
    } else {

            eprintln!("Error reading Project Experience JSON file, please check https://github.com/BradenEverson/sitesmith for a concise example of JSON format")
    }

    Ok(())

}
