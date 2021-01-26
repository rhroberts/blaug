use markdown;
use std::{fs, path::Path};
use warp::http::{self, Response};

pub fn get_post(title: String) -> http::Result<Response<String>> {
    let css_path = Path::new("./assets/css/blog_posts.css");
    let post_string = format!("./assets/posts/{}.md", title);
    let post_path = Path::new(&post_string);
    if post_path.exists() {
        let markdown = fs::read_to_string(post_path).unwrap();
        let stylesheet = fs::read_to_string(css_path).unwrap();
        let html: String = make_post_html(markdown, stylesheet, "atom-one-dark");
        return Response::builder().body(html);
    } else {
        return Response::builder().body(format!(
            "{} does not exist!",
            post_path.display().to_string()
        ));
    }
}

fn make_post_html(markdown: String, stylesheet: String, hljs_style: &str) -> String {
    let head: String = make_head_tag(format!(
        "{}{}",
        make_style_tag(stylesheet),
        make_hljs_boilerplate(hljs_style)
    ));
    let body: String = format!("\n<body>\n{}", markdown::to_html(&markdown));
    return format!("<html>\n{}{}\n</html>", head, body);
}

fn make_head_tag(head: String) -> String {
    return format!("\n<head>{}</head>\n", head);
}

fn make_link_tag(rel: &str, href: &str) -> String {
    return format!("\n<link rel=\"{}\" href=\"{}\">\n", rel, href);
}

fn make_script_tag(src: &str) -> String {
    return format!("\n<script>\n{}\n</script>\n", src);
}

fn make_script_src_tag(src: &str) -> String {
    return format!("\n<script src=\"{}\"></script>\n", src);
}

fn make_style_tag(style: String) -> String {
    return format!("\n<style>\n{}</style>\n", style);
}

fn make_hljs_boilerplate(style: &str) -> String {
    let src =
        "//cdn.jsdelivr.net/gh/highlightjs/cdn-release@10.5.0/build/highlight.min.js";
    let style_src = format!(
        "//cdn.jsdelivr.net/gh/highlightjs/cdn-release@10.5.0/build/styles/{}.min.css",
        style
    );
    return format!(
        "{}{}{}",
        make_link_tag("stylesheet", &style_src),
        make_script_src_tag(src),
        make_script_tag("hljs.initHighlightingOnLoad();")
    );
}

fn get_post_list() -> fs::ReadDir {
    let path_result = fs::read_dir("./assets/posts");
    let paths = match path_result {
        Err(error) => panic!("Problem getting file paths due to error: {}", error),
        Ok(read_dir) => read_dir,
    };
    return paths;
}
