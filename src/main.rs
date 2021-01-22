use warp::Filter;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    // GET / => 200 OK with body "Hello, root!"
    let hello = warp::path::end().map(|| "Hello, root!");
    // GET /apple => 200 OK with body "Hello, apple!"
    let hello_friend = warp::path!(String).map(|friend: String| param_extract(friend));

    // API
    let routes = warp::get().and(hello.or(hello_friend));

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

fn param_extract(param: String) -> String {
    info!("I extracted {}!", &param);
    return format!("Hello, {}!", &param);
}
