# infra as macro

translate rust structs to terraform at compile time

## wait what

why would you do this??


well, pulumi is cool and all, but kinda lame since it requires actually *running* code to generate the right terraform files.

what if... we used rust macros... and const-compatible structs... to spit out the `.tf` files *AT COMPILE TIME*.

## what this demos

This repo doesn't currently do rust -> tf translation. TODO.

This repo demos a few things:
- how we can create *typesafe macros* that provide intellisense and goto-def
- how this can be automated
- how this can generate the .tf files, migrations, .sql, etc
- and the same types can be USED within your app. typesafely!

## how we intend it to be used

You'll want to define your infra. You can choose between sqlite, postgres, redis, kafka, clickhouse, mqtt, and s3. This should cover most basic use cases.

For a postgres 16 instance, we create a global static. We can do this in our app or in a workspace dependency - both work and get mapped to the same infra.

```rust
static DB: Postgres = Postgres16! {
    host: "env.host",
    password: "env.password",
    database: "todos",
    user: "localhost",
    port: 5432
};
```


Then, we can create a new table against this bit of infra. We can associate the table manually, or just let iac figure it out at runtime.

```rust
#[derive(Table)]
struct User {
    id: Id<Self>,
    name: String,
    email: String,
    friends: Vec<Id<Self>>,
    profile: UrlS3<PHOTOS>
}
```

We can even reference this entity in other tables:
```rust
// And now declare a relation between two tables
#[derive(Table)]
struct Post {
    id: Id<Self>,
    creator: Id<User>,
}
```

If we wanted, we could add s3 backing photo uploads
```rust
static PHOTOS: S3Bucket = S3! {
    name: "photos"
}
```

And then add it to our struct, typesafely.
```rust
#[derive(Table)]
struct User {
    // ....
    profile_photo: UrlS3<PHOTOS>
}
```

This will spit out the right `.sql` migrations in our deploy directory. IAC will properly stick together all the migrations and .tf files to get a sense of your infra.

In theory, we could create no-code tooling to visualize and customize the infra.

## using iac


Since the structs are readily available in our code, we can just query them directly.

```rust
fn main() {
    // query the db for a list
    let users = DB.list::<User>();

    // do some sql
    let users = DB.query(sql!(`select * from {User} where {User.friends.len()} < 5`));

    // handle a photo upload
    let new_photo: S3Url = S3.upload(bytes).unwrap();

    // and then insert those types directly
    DB.insert(User {
        id: unassigned_id(),
        name: "asd",

    })
}
```



## deploying

This is where iac-deploy comes in. You can embed this in any CLI you want. Our plan is to embed this into Dioxus via the `dx` cli.

iac-deploy swallows up all the metadata and builds a run-plan for you automatically.


## ideas

- so far we've demonstrated rust -> tf but in theory we could go the otherway to allow `tf -> rust` and bidrectional syncing so you can plug into existing infra.


## security

well, don't ship your infra to the frontend, obviously. the iac-macros crate will eliminate all variables on wasm32 unless you specify via cfg flags/features/env vars that you absolutely do want them embedded. You *might* want this if you're experimenting with wasm-based runners, however IAC does not support any wasm-based drivers.

## runtime declaration

Currently... we don't have the concept of choosing your runtime engine yet. We're considering doing this by letting you annotate various functions as a particular entrypoint.

For example, an EC2 instance could be setup with an attribute macro on main...
```rust
static APP: EC2 = EC2 {
    size: "t2-micro"
};

#[iac::runner(APP)]
#[tokio::main]
async fn main() {
    // do stuff
}
```

And say, an edge function could be setup using the "server_fn" attribute, similar to dioxus

```rust
#[server_fn]
async fn upload_photos() {

}
```

This approach would let you stitch together clusters with typesafety. I haven't really thought about going this far yet...

```rust
static CLUSTER: Cluster = Cluster! {
    proxy: WEST_VNET,
    services: [ KAFKA, REDIS, APP ]
}
```


## that's all folks

join the dioxus discord and hop into `#deploy` if this seems like something you'd want to chat about

https://discord.gg/XgGxMSkvUM
