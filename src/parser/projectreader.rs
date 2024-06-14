use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct ProjectParse {
    format: String,
    projects: Vec<Entry>
}

impl ProjectParse {
    pub fn from_entries(projects: Vec<Entry>, format: String) -> Self {
        Self { format, projects }
    }
    pub fn format(&self) -> String {
        let mut res = String::new();

        for project in &self.projects {
            res += &project.format(&self.format);
        }

        res
    }
}

#[derive(Serialize, Deserialize)]
pub struct Entry {
    name: String,
    descr: Vec<String>,
    extra: Option<Vec<String>>
}

impl Entry {
    pub fn format(&self, format: &str) -> String {
        let mut res = format.to_string();
        
        res = res.replace("{name}", &self.name);

        let mut descr = String::new();

        for item in &self.descr {
            descr += &format!("<li>{}</li>", item);
        }

        res = res.replace("{descr}", &descr);

        let extra = match &self.extra {
            Some(ex) => ex.join("<br/>"),
            None => "".into()
        };

        res = res.replace("{extra}", &extra);

        res
    }
}
