// mod blog;

// use blog::posts;
use markdown;
use std::{fs, path::Path};
use tera::Tera;
use tide;
use tide_tera::prelude::*;

// TODO: would be better to get this from h1 element, not filename
fn post_name_to_title(s: &str) -> String {
    // replace underscores from post title with spaces
    let new_s: String = s
        .chars()
        .map(|x| match x {
            '_' => ' ',
            _ => x,
        })
        .collect();
    // uppercase first letter of string
    let mut c = new_s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
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
    app.at("/").get(|req: tide::Request<Tera>| async move {
        let tera = req.state();
        tera.render_response(
            "index.html",
            &context! {
                "test" => "Hello from tera template!"
            },
        )
    });
    app.at("/blog").get(|req: tide::Request<Tera>| async move {
        let tera = req.state();
        tera.render_response("blog.html", &context! {})
    });
    // TODO: return 404 if requested post doesn't exist
    app.at("/blog/:post")
        .get(|req: tide::Request<Tera>| async move {
            let tera = req.state();
            let post = req.param("post")?;
            let post_string = format!("./static/posts/{}.md", post);
            let post_path = Path::new(&post_string);
            let mkd = fs::read_to_string(&post_path)?;
            tera.render_response(
                "blog_post.html",
                &context! {
                    "title" => post_name_to_title(&post),
                    "markdown_content" => &markdown::to_html(&mkd)
                },
            )
        });
    app.listen("127.0.0.1:3030").await?;
    Ok(())
}
