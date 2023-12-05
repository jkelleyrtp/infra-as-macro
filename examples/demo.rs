use iac_macros::{Id, Postgres, Postgres16, PostgresOpts, Table};

// Declare our postgres instance
static DB: Postgres = Postgres16! {
    host: "env.host",
    password: "env.password",
    database: "todos",
    user: "localhost",
    port: 5432
};

// Declare a table
#[derive(Table)]
struct User {
    id: Id<Self>,
    name: String,
    email: String,
    friends: Vec<Id<Self>>,
}

// And now declare a relation between two tables
#[derive(Table)]
struct Post {
    id: Id<Self>,
    creator: Id<User>,
}

fn main() {
    let users = DB.query::<User>();
}
