use async_graphql::*;
use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_axum::GraphQL;
use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use hyper::{Method, Server};

struct QueryRoot;
struct MutationRoot;

#[derive(SimpleObject)]
struct Barby {
    name: String,
    age: i32,
    children: Vec<Child>,
}

#[derive(SimpleObject)]
struct Girl {
    girl_name: String,
    month: Month,
}

#[derive(SimpleObject)]
struct Boy {
    boy_name: String,
    month: Month,
}


#[derive(Union)]
enum Child {
    Boy(Boy),
    Girl(Girl),
}

#[derive(Union)]
enum Month {
    January(January),
    Febuary(February),
}

#[derive(SimpleObject)]
struct January {
    day: i32,
}

#[derive(SimpleObject)]
struct February {
    day: i32,
}

#[Object]
impl QueryRoot {
    async fn hello<'ctx>(&self, _ctx: &Context<'ctx>) -> Barby {
        let rv = Barby {
            name: "Yoda".to_string(),
            age: 900,
            children: vec![
                Child::Boy(Boy { boy_name: "Luke".to_string(), month: Month::January(January { day: 1 }) }),
                Child::Girl(Girl { girl_name: "Leia".to_string(), month: Month::Febuary(February { day: 25 }) }),
            ]
        };
        rv
    }
}

#[Object]
impl MutationRoot {
    async fn hellomooda(&self, something: String) -> String {
        format!("Hello world! {}", something).to_string()
    }
}

#[tokio::main]
async fn main() {
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(vec![1, 2, 3])
        .finish();

    println!("GraphiQL IDE: http://localhost:8000");

    let app = Router::new()
        .route("/", get(Html(GraphiQLSource::build().endpoint("/").finish())).post_service(GraphQL::new(schema)));

    Server::bind(&"127.0.0.1:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
