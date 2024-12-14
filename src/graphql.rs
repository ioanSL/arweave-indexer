use juniper::{graphql_object, EmptyMutation, EmptySubscription, FieldResult, RootNode};

use crate::{database::MongoDatabase, transaction::Transaction};

pub struct Query;

#[graphql_object]
#[graphql(context = GraphQLContext)]
impl Query {
    async fn transaction(context: &GraphQLContext, id: String) -> FieldResult<Option<Transaction>> {
        let db = &context.db;
        let transaction = db.get_transaction(&id).await?;
        Ok(transaction)
    }
}

pub struct GraphQLContext {
    pub db: MongoDatabase,
}

impl juniper::Context for GraphQLContext {}

pub type Schema =
    RootNode<'static, Query, EmptyMutation<GraphQLContext>, EmptySubscription<GraphQLContext>>;
