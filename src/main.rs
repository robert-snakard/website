use std::fs::File;
use std::io::Write;
use tera::{Tera, Context};

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let tera = Tera::new("assets/templates/**/*")?;
    let ctx = Context::new();
    let html = tera.render("index.html", &ctx)?;

    let mut file = File::create("index.html")?;
    file.write_all(&html.into_bytes())?;
    Ok(())
}
