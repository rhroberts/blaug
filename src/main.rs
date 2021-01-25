use std::{fs, path::Path};

use log;
use markdown;
use pretty_env_logger;
use warp::{
    http::{self, Response},
    Filter,
};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    // GET / => 200 OK with body "Hello, root!"
    let hello = warp::path::end().map(|| "Hello.");
    // GET /blog
    let blog = warp::path!("blog").map(|| "Welcome to my blog.");
    // GET /blog/[article_title]
    let posts = warp::path!("blog" / String).map(|post| get_post(post));

    // API
    let routes = warp::get().and(hello.or(blog).or(posts));

    log::info!("Starting server...");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

fn get_post(title: String) -> http::Result<Response<String>> {
    let css_path = Path::new("./assets/css/blog_posts.css");
    let post_string = format!("./assets/posts/{}.md", title);
    let post_path = Path::new(&post_string);
    if post_path.exists() {
        let mkd = match fs::read_to_string(post_path) {
            Err(error) => error.to_string(),
            Ok(contents) => contents,
        };
        let body: String = markdown::to_html(&mkd);
        let style = fs::read_to_string(css_path).unwrap();
        let highlight_js = String::from(
            "<link rel=\"stylesheet\" href=\"//cdn.jsdelivr.net/gh/highlightjs/cdn-release@10.5.0/build/styles/default.min.css\">\n<script src=\"//cdn.jsdelivr.net/gh/highlightjs/cdn-release@10.5.0/build/highlight.min.js\">\n</script>\n<script>\nhljs.initHighlightingOnLoad();\n</script>\n"
        );
        let head = format!(
            "<head>\n<style>\n{}</style>\n{}\n</head>\n\n",
            style, highlight_js
        );
        println!("{}", &head);
        return Response::builder().body(head + &body);
    } else {
        return Response::builder().body(format!(
            "{} does not exist!",
            post_path.display().to_string()
        ));
    }
}

fn get_post_list() -> fs::ReadDir {
    let path_result = fs::read_dir("./assets/posts");
    let paths = match path_result {
        Err(error) => panic!("Problem getting file paths due to error: {}", error),
        Ok(read_dir) => read_dir,
    };
    return paths;
}
