use crate::auth::{sign_in, sign_out};
use crate::models::Session;
use juniper::FieldResult;
use rocket::{response::content, State};

struct Context;
impl juniper::Context for Context {}

struct Query;
#[juniper::object(
Context = Context,
)]
impl Query {}

struct Mutation;
#[juniper::object(
Context = Context,
)]
impl Mutation {
    fn signin(context: &Context, login: String, password: String) -> FieldResult<Session> {
        let session = sign_in(login.as_str(), password.as_str())?;
        Ok(session)
    }
    fn signout(context: &Context, token: String) -> FieldResult<String> {
        sign_out(token.as_str())?;
        Ok("Ok".to_string())
    }
}

type Schema = juniper::RootNode<'static, Query, Mutation>;

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
    request.execute(&schema, &context)
}

pub fn start() {
    println!("Starting the server...");
    rocket::ignite()
        .manage(Context)
        .manage(Schema::new(Query, Mutation))
        .mount(
            "/",
            routes![graphiql, get_graphql_handler, post_graphql_handler],
        )
        .launch();
}
