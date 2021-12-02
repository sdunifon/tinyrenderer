use warp::Filter;

#[tokio::main]
async fn main() {
    println!("hello world!");
    // let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}", name));
    // warp::serve(hello).run(([127, 0, 0, 1], 4040)).await
    // pretty_env_logger::init();

    warp::serve(warp::fs::dir("./web/dist"))
        .run(([127, 0, 0, 1], 3030))
        .await;
}
