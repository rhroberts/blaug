use warp::Filter;

#[tokio::main]
async fn main() {
    // GET / => 200 OK with body "Hello, Warp!"
    let hello = warp::path::end().map(|| "Hello, Warp!");

    warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;
}
