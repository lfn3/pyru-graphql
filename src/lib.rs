use async_graphql::{http::graphiql_source, EmptyMutation, EmptySubscription, Object};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::Extension,
    response::{self, IntoResponse},
    routing::get,
    AddExtensionLayer, Router,
};

pub struct Query {}

#[Object]
impl Query {
    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}

type Schema = async_graphql::Schema<Query, EmptyMutation, EmptySubscription>;

async fn graphql_handler(schema: Extension<Schema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

pub fn make_schema() -> Schema {
    Schema::new(Query {}, EmptyMutation, EmptySubscription)
}

async fn graphiql() -> impl IntoResponse {
    response::Html(graphiql_source("/", None))
}

pub fn make_router() -> Router {
    let schema = make_schema();
    Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .layer(AddExtensionLayer::new(schema))
}
