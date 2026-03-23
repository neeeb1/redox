use chrono::prelude::Utc;
use std::{fs::File, sync::LazyLock, io::Write};
use tera::{Context, Tera};

use crate::app::App;

static TEMPLATES: LazyLock<Tera> = LazyLock::new(|| {
    let tera = match Tera::new("templates/*") {
        Ok(t) => t,
        Err(e) => {
            println!("Failed to parse template(s): {}", e);
            std::process::exit(1);
        }
    };
    tera
});

impl App {
    pub fn render_template(&self) -> String {
        let mut context = Context::new();

        let now = &Utc::now().to_rfc2822().to_string();
        context.insert("date", now);
        context.insert("entries", &self.entries);
        TEMPLATES.render("daily.md", &context).unwrap()
    }
}

pub fn write_file(file: String) -> std::io::Result<()> {
    let path = "output/daily.md";
    let mut output = File::create(path)?;
    write!(output, "{}", file)
}