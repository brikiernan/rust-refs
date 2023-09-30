use axum::{ routing::get, Router };

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(root))
        .route("/foo", get(get_foo).post(post_foo))
        .route("/foo/bar", get(foo_bar));

    // run it with hyper on localhost:3000
    axum::Server
        ::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service()).await
        .unwrap()
}

async fn root() -> &'static str {
    "Hey Now Watching Axum!!"
}
async fn get_foo() -> &'static str {
    "Hey Now Get Foo Route"
}
async fn post_foo() -> &'static str {
    "Hey Now Post Foo Route"
}
async fn foo_bar() -> &'static str {
    "Hey Now Get Foo Bar Route"
}
