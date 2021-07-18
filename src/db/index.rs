use crate::index::Result;
use actix::prelude::{Actor, SyncContext};
use diesel::{
    pg::PgConnection,
    r2d2::{self, ConnectionManager, Pool, PooledConnection},
    Connection,
};

pub type Conn = PgConnection;
pub type PgPool = Pool<ConnectionManager<Conn>>;
pub type PooledConn = PooledConnection<ConnectionManager<Conn>>;

pub struct DbActor(pub PgPool);

// impl Actor for DbExecutor {
//     type Context = SyncContext<Self>;
// }

pub fn get_pool(db_url: String) -> Result<PgPool> {
    let manager = ConnectionManager::<Conn>::new(db_url);
    let pool = r2d2::Pool::builder()
        // .max_size(15)
        .build(manager)
        .expect("Failed to create pool");

    Ok(pool)
}

pub fn run_migrations(db_url: &str) {
    embed_migrations!();
    let connection = PgConnection::establish(db_url).expect("Error connecting to db");
    embedded_migrations::run_with_output(&connection, &mut std::io::stdout())
        .expect("Error running migrations");
}

impl Actor for DbActor {
    type Context = SyncContext<Self>;
}
