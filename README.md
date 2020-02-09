# Talks Game Backend Rust implementation

## How to run

Have Rust nightly installed.

`$ cargo run`

## GraphQL

### Sign in

```
mutation signin($login: String!, $password: String!){
  signin(login: $login, password: $password) {
    token
    teamNumber
    teamType
    isCommander
    insertedAt
    updatedAt
  }
}

{
  "login": "supplier1",
  "password": "supplier1"
}
```

### Sign out

```
mutation signout($token: String!){
  signout(token: $token)
}

{
  "token": "fe0d4371-f08f-4e64-b496-5f3c2dcc8715"
}
```