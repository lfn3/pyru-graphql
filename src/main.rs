use axum::Server;
use pyru_graphql::make_router;

#[tokio::main]
async fn main() {
    let app = make_router();
    
    Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
