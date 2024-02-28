use serde::{Deserialize, Serialize};
use worker::*;

#[derive(Debug, Deserialize, Serialize)]
struct GenericResponse {
    status: u16,
    message: String,
}

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    Router::new()
        .get_async("/name", handle_get)
        .get_async("/name_cors", handle_cors)
        .run(req, env)
        .await
}

pub async fn handle_get(_: Request, _ctx: RouteContext<()>) -> worker::Result<Response> {
    Response::from_json(&GenericResponse {
        status: 200,
        message: "Hello RUST people!".to_string(),
    })
}

pub async fn handle_cors(_: Request, _ctx: RouteContext<()>) -> worker::Result<Response> {
    let res = Response::from_json(&GenericResponse {
        status: 200,
        message: "Hello RUST people!".to_string(),
    })?;
    let cors = Cors::default().with_origins(vec!["http://127.0.0.1:1420"]);
    res.with_cors(&cors)
}