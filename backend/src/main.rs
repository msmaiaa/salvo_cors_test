use salvo::{
    extra::cors::CorsBuilder, handler, prelude::TcpListener, writer::Json, Request, Response,
    Router, Server,
};
use serde::Serialize;

#[derive(Serialize)]
struct Person {
    name: String,
    age: u8,
}

#[handler]
async fn hello_world(_: &mut Request, res: &mut Response) {
    res.render(Json(Person {
        name: "John".to_string(),
        age: 18,
    }))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let listen_addr = format!("0.0.0.0:{}", 1337);
    let cors_handler = CorsBuilder::new()
        .allow_credentials(true)
        .allow_any_origin()
        .allow_methods(vec!["OPTIONS", "GET", "POST", "DELETE", "PUT", "PATCH"])
        .allow_headers(vec![
            "CONTENT-TYPE",
            "Access-Control-Request-Method",
            "Access-Control-Allow-Origin",
            "Access-Control-Allow-Headers",
            "Access-Control-Max-Age",
            "authorization",
        ])
        .build();

    let router =
        Router::with_hoop(cors_handler).push(Router::with_path("/hello_world").get(hello_world));
    tracing::info!("Server started at http://{}/", listen_addr);

    Server::new(TcpListener::bind(&listen_addr))
        .serve(router)
        .await;
}
