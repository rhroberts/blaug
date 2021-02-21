mod blog;

use blog::posts;
#[macro_use]
extern crate lazy_static;
use log;
use pretty_env_logger;
use tera::{Context, Tera};
use warp::{http, Filter};

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/*.html") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error: {}", e);
                ::std::process::exit(1);
            }
        };
        tera
    };
}

fn from_template(
    template: &str,
    context: tera::Context,
) -> http::Result<http::Response<String>> {
    let resp =
        http::Response::builder().body(TEMPLATES.render(template, &context).unwrap());
    return resp;
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    // GET / => 200 OK with body "Hello."
    // let home = warp::path::end().map(|| "Hello.");
    // let tera = match Tera::new("templates/*.html") {
    //     Ok(t) => t,
    //     Err(e) => {
    //         println!("Parsing error(s): {}", e);
    //         ::std::process::exit(1);
    //     }
    // };
    let mut context = Context::new();
    context.insert("test", &100);
    // let tmp = http::Response::builder().body(TEMPLATES.render("index.html", &context));
    let tmp = from_template("index.html", context);
    println!("{}", tmp);
    let home = warp::path::end().map(|| tmp);
    // GET /blog
    let blog = warp::path!("blog").and(warp::fs::file("static/blog/html/index.html"));
    // GET /blog/[article_title]
    let posts = warp::path!("blog" / String).map(|post| posts::get_post(post));
    // GET /static/[asset]
    let images = warp::path("static").and(warp::fs::dir("static/blog/images"));
    // API
    let routes = warp::get().and(home.or(blog).or(posts).or(images));
    log::info!("Starting server...");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
