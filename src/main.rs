
use axum::{routing::get, Router,
        extract::Path};

async fn hello_world() -> &'static str {
    "Hello, world!"
}
async fn task1(Path((num1, num2)): Path<(i32, i32)>) -> String{
    (num1 ^ num2).pow(3).to_string()
}
#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/", get(hello_world))
                            .route("/1/:num1/:num2", get(task1));

    Ok(router.into())
}
