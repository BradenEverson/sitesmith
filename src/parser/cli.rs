use clap::Parser;

// sitesmith projects.json work.json template.html out.html
#[derive(Parser)]
pub struct CliArgs {
    /// The location of the json file holding projects
    #[arg(short, long)]
    pub projects: std::path::PathBuf,
    /// The location of the json file holding work experience
    #[arg(short, long)]
    pub work: std::path::PathBuf,
    /// The location of the html file holding your template
    #[arg(short, long)]
    pub template: std::path::PathBuf,
    /// The desired output html file
    #[arg(short, long)]
    pub output: String,
}
