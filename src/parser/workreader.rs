use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct WorkParse {
    format: String,
    work: Vec<WorkEntry>
}

impl WorkParse {
    pub fn from_entries(work: Vec<WorkEntry>, format: String) -> Self {
        Self { format, work }
    }
    pub fn format(&self) -> String {
        let mut res = String::new();

        for work in &self.work {
            res += &work.format(&self.format);
        }

        res
    }
}

#[derive(Serialize, Deserialize)]
pub struct WorkEntry {
    name: String,
    timespan: String,
    location: String,
    descr: Vec<String>,
    extra: Option<Vec<String>>
}

impl WorkEntry {
    pub fn format(&self, format: &str) -> String {
        let mut res = format.to_string();
        
        res = res.replace("{name}", &self.name);

        res = res.replace("{timespan}", &self.timespan);
        res = res.replace("{location}", &self.location);

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
