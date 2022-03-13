use std::{env, convert::Infallible};
use warp::{Filter, Rejection, Reply, hyper::StatusCode};
use chrono::Local;
#[tokio::main]

async fn main() {
    let args: Vec<String> = env::args().collect();
    let downloadroute = warp::path("content").and(warp::fs::dir("./content/"));
    let router = downloadroute.recover(handle_rejection);
    let arg1 = &args[1];
    let srvport: u16 = arg1.parse().unwrap();

    if args.len() != 1 {
        println!("Syntax: webserver <port>")
    } else{
        println!("Serving HTTP on 0.0.0.0 port {}", srvport);
        warp::serve(router).run(([0, 0, 0, 0,], srvport)).await;
    }
}

async fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
    let date = Local::now();
    let (code, message) = if err.is_not_found() {
        println!("{}", date.format("[%D/%m/%y %H:%M:%S] code 404, message File not found"));
        (StatusCode::NOT_FOUND, "404".to_string())
    } else if err.find::<warp::reject::PayloadTooLarge>().is_some() {
        (StatusCode::BAD_REQUEST, "Payload too large".to_string())
    } else {
        eprintln!("unhandled error: {:?}", err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error".to_string(),
        )
    };
    Ok(warp::reply::with_status(message, code))
}
