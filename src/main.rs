use tera::{Tera, Context};

fn main() -> Result<(), tera::Error>{
    let tera = Tera::new("assets/templates/**/*")?;

    let mut ctx = Context::new();
    ctx.insert("title", "helloworld");

    let html = tera.render("content.html", &ctx)?;
    println!("{}", html);
    Ok(())
}
