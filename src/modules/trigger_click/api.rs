use anyhow::Result;
use mouse_rs::{types::keys::Keys, Mouse};
use serde_json::{json, Value};
use warp::{reply::Json, Filter};

pub fn trigger_click_filter(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec![
            "User-Agent",
            "Sec-Fetch-Mode",
            "Referer",
            "Origin",
            "Access-Control-Request-Method",
            "Access-Control-Request-Headers",
        ])
        .allow_methods(vec!["POST", "GET"]);

    let path = warp::path("trigger_click");

    let post_click = path
        .and(warp::post())
        .and(warp::body::json())
        .and_then(trigger_click_handler);

    let get_click = path.and(warp::get()).and_then(get_click_handler);

    post_click.or(get_click).with(cors)
}

async fn get_click_handler() -> Result<Json, warp::Rejection> {
    let mouse = Mouse::new();
    mouse.click(&Keys::LEFT).expect("Unable to press button");

    let reply = json!(
        {
            "status": "ok",
            "data": {}
        }
    );

    Ok(warp::reply::json(&reply))
}

async fn trigger_click_handler(data: Value) -> Result<Json, warp::Rejection> {
    let x = data["x"].as_u64().unwrap();
    let y = data["y"].as_u64().unwrap();

    let mouse = Mouse::new();
    mouse.move_to(500, 500).expect("Unable to move mouse");
    mouse.click(&Keys::LEFT).expect("Unable to press button");

    let reply = json!(
        {
            "status": "ok",
            "data": {}
        }
    );

    Ok(warp::reply::json(&reply))
}
