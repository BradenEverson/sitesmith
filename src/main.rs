use std::{fs::File, io::Read};

use clap::Parser;
use sitesmith::parser::{cli::CliArgs, htmlreader::HtmlWeaver, projectreader::{Entry, ProjectParse}, workreader::{WorkEntry, WorkParse}};


fn main() -> std::io::Result<()> {
    let args = CliArgs::parse();

    let mut proj_str = String::new();
    let mut work_str = String::new();
    
    let mut template = String::new();

    File::open(args.projects)?.read_to_string(&mut proj_str)?;
    File::open(args.work)?.read_to_string(&mut work_str)?;
    
    File::open(args.template)?.read_to_string(&mut template)?;

    let proj_entry: Vec<Entry> = serde_json::from_str(&proj_str).unwrap();
    let work_entry: Vec<WorkEntry> = serde_json::from_str(&work_str).unwrap();

    let proj_fmt = "<div class='project-item'><h3>{name}</h3><ul>{descr}</ul>{extra}</div>";
    let work_fmt = "<div class='experience-item'><h3>{name}</h3><p>{timespan} | {location}</p><ul>{descr}</ul>{extra}</div>";

    let proj_parse = ProjectParse::from_entries(proj_entry, proj_fmt.to_string());
    let work_parse = WorkParse::from_entries(work_entry, work_fmt.to_string());

    let weaver = HtmlWeaver::new(template, proj_parse, work_parse);

    weaver.generate(&args.output)
}
