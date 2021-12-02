use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(long = "port", short = "p", default_value = "4000")]
    port: u16,
}

#[tokio::main]
async fn main() {
    let args = Cli::from_args();
    let port = args.port;
    println!("Serving on port {}", port);
    pretty_env_logger::init();

    warp::serve(warp::fs::dir("./web/dist"))
        .run(([0, 0, 0, 0], port))
        .await;
}
