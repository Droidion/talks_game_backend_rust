use juniper::{FieldResult, EmptyMutation};
use crate::models::Session;
use crate::auth::sign_in;
use rocket::{response::content, State};

struct Context;
impl juniper::Context for Context {}

struct Query;
#[juniper::object(
    Context = Context,
)]
impl Query {
    fn signin(context: &Context, login: String, password: String) -> FieldResult<Session> {
        let session = sign_in(login.as_str(), password.as_str())?;
        Ok(session)
    }
}

type Schema = juniper::RootNode<'static, Query, EmptyMutation<Context>>;


#[get("/")]
fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[rocket::get("/graphql?<request>")]
fn get_graphql_handler(
    context: State<Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

#[rocket::post("/graphql", data = "<request>")]
fn post_graphql_handler(
    context: State<Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    println!("asdasdasd");
    request.execute(&schema, &context)
}


pub fn start() {
    println!("Starting the server...");
    rocket::ignite()
        .manage(Context)
        .manage(Schema::new(Query, EmptyMutation::new()))
        .mount("/", routes![graphiql, get_graphql_handler, post_graphql_handler])
        .launch();
}
