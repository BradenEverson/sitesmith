use std::{fs::File, io::Write};

use super::{projectreader::ProjectParse, workreader::WorkParse};


pub struct HtmlWeaver {
    site_template: String,
    projects: ProjectParse,
    work: WorkParse
}

impl HtmlWeaver {
    pub fn new(site_template: String, projects: ProjectParse, work: WorkParse) -> Self {
        Self { site_template, projects, work }
    }

    pub fn generate(&self, out: &str) -> std::io::Result<()> {

        let mut file = File::create(out)?;
        let res = self.site_template
            .replace("{projects}", &self.projects.format())
            .replace("{work}", &self.work.format());

        file.write_all(res.as_bytes())
    }

}
