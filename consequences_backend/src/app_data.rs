use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct AppData {
    pool: Pool,
}

impl AppData {

    pub fn new(db_pool: Pool) -> AppData {
        AppData {
            pool: db_pool,
        }
    }

    pub fn pool(&self) -> &Pool {
        &self.pool
    }
}