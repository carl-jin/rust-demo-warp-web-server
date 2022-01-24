use crate::modules::trigger_click_filter;
use warp::Filter;
mod modules;

const WEB_STATIC: &str = "static/";

#[tokio::main]
async fn main() {
    //  APIs
    let hi = warp::path("hi").map(|| "hello from hi");
    let api = hi.or(trigger_click_filter());

    // static
    let content = warp::fs::dir(WEB_STATIC);
    let root = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file(format!("{}/index.html", WEB_STATIC)));
    let static_site = content.or(root);

    let routes = api.or(static_site);

    println!("server stared");
    warp::serve(routes).run(([127, 0, 0, 1], 8888)).await
}
