// mod blog;

// use blog::posts;
#[macro_use]
extern crate lazy_static;
use tera::Tera;
use tide;
use tide_tera::prelude::*;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let tera = match Tera::new("templates/*.html") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error: {}", e);
                ::std::process::exit(1);
            }
        };
        tera
    };
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::start();

    let mut tera = Tera::new("templates/*.html")?;
    tera.autoescape_on(vec!["html"]);

    let mut app = tide::with_state(tera);

    // serve static files
    app.at("/static").serve_dir("static/")?;
    // root
    app.at("/").get(|_| async move {
        TEMPLATES.render_response(
            "index.html",
            &context! {
                "test" => "Hello from tera template!"
            },
        )
    });
    app.listen("127.0.0.1:3030").await?;
    Ok(())
}
