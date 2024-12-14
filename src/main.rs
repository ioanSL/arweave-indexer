mod arweave_indexer;
mod config;
mod database;
mod filter;
mod graphql;
mod tag;
mod transaction;

use arweave_indexer::ArweaveIndexerOptions;
use filter::create_filter;
use graphql::{GraphQLContext, Query, Schema};
use juniper::{EmptyMutation, EmptySubscription};
use warp::Filter;

use crate::arweave_indexer::ArweaveIndexer;

async fn run_graphql_service(schema: Schema, db: database::MongoDatabase) {
    let state = warp::any().map(move || GraphQLContext { db: db.clone() });
    let graphql_filter = juniper_warp::make_graphql_filter(schema, state.boxed());

    let routes = (warp::post().and(warp::path("graphql")).and(graphql_filter)).or(warp::get()
        .and(warp::path("graphql"))
        .and(juniper_warp::graphiql_filter(
            "/graphql",
            Some("/subscriptions"),
        )));

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}

#[tokio::main]
async fn main() {
    let config = config::Config::from_file("src/config/config.json");
    let db = database::MongoDatabase::new(&config.mongo_url).await;
    let indexer_options = ArweaveIndexerOptions {
        start_block: config.start_block,
        checkpoints: config.checkpoints,
        filter: Some(create_filter()),
    };
    let mut indexer = ArweaveIndexer::new(db.clone(), indexer_options).await;

    tokio::spawn(async move {
        indexer.start().await;
    });
    let schema = Schema::new(Query, EmptyMutation::new(), EmptySubscription::new());
    run_graphql_service(schema, db.clone()).await;
}
