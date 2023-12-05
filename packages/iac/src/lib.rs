pub use macros::{Postgres16, Table};

#[derive(Clone, Copy)]
pub struct PostgresOpts {
    /// Host to connect to
    pub host: &'static str,

    /// Port to connect to
    pub port: u16,

    /// User to connect as
    pub user: &'static str,

    /// Password for the user
    pub password: &'static str,

    /// Name of the database to connect to
    pub database: &'static str,
}

pub struct Postgres {
    pub opts: PostgresOpts,
}

impl Postgres {
    pub async fn connect(&'static self) {
        println!("Connecting to postgres...");
    }

    pub fn connect_sync(&'static self) -> PostgresConnection {
        println!("Connecting to postgres...");
        PostgresConnection {
            opts: self.opts,
            item: self,
        }
    }

    pub fn put(&self, key: &str, value: &str) {
        println!("Putting {} into {}", value, key);
    }

    pub fn query<T: Table>(&self) -> Vec<T> {
        vec![]
    }
}

pub struct PostgresConnection {
    pub opts: PostgresOpts,
    item: &'static Postgres,
}

pub trait Table {
    type Id;
}

pub type Id<T> = <T as Table>::Id;
