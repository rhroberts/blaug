mod blog;

use blog::posts;
use log;
use pretty_env_logger;
use warp::Filter;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    // GET / => 200 OK with body "Hello."
    let hello = warp::path::end().map(|| "Hello.");
    // GET /blog
    let blog = warp::path!("blog").and(warp::fs::file("static/blog/html/index.html"));
    // GET /blog/[article_title]
    let posts = warp::path!("blog" / String).map(|post| posts::get_post(post));
    // GET /static/[asset]
    let images = warp::path("static").and(warp::fs::dir("static/blog/images"));
    // API
    let routes = warp::get().and(hello.or(blog).or(posts).or(images));
    log::info!("Starting server...");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
