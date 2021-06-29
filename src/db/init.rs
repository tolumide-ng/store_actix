use actix::prelude::{Actor, SyncContext};
use diesel::{pg::PgConnection, r2d2::{self, ConnectionManager, Pool}};