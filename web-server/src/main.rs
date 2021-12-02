use warp::Filter;

#[tokio::main]
async fn main() {
    println!("hello world");
    let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}", name));
    warp::serve(hello).run(([127, 0, 0, 1], 4040)).await
}
