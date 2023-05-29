use axum::{
    routing::{get, post},
    Router,
};

use tower_http::cors::{Any, CorsLayer};

mod handlers;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let cors = CorsLayer::new().allow_origin(Any);

    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let pool = sqlx::PgPool::connect(&database_url)
        .await
        .expect("Error with pool connection");

    sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS products (
          id serial,
          name text,
          price integer
        );"#,
    )
    .execute(&pool)
    .await;

    let app = Router::new()
        .route("/", get(root))
        .route(
            "/api/products",
            get(handlers::get_products).post(handlers::create_product),
        )
        .route(
            "/api/products/:id",
            get(handlers::get_product)
                .put(handlers::update_product)
                .delete(handlers::delete_product),
        )
        .with_state(pool)
        .layer(cors);

    tracing::debug!("listening on {}", "0.0.0.0:3000");
    println!("listening on {}", "0.0.0.0:3000");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello world"
}
