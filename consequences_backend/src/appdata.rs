use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use r2d2_redis::RedisConnectionManager;
use std::env;
use dotenv::dotenv;

pub type DatabasePool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type RedisPool = r2d2::Pool<RedisConnectionManager>;

pub struct AppData {
    db_pool: DatabasePool,
    redis_pool: RedisPool,
}

impl AppData {

    pub fn new() -> AppData {
        dotenv().ok();
        let manager = ConnectionManager::<PgConnection>::new(env::var("DATABASE_URL").expect("DATABASE_URL must be set"));
        let db_pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Could not create pool.");
//        let manager = RedisConnectionManager::new(env::var("REDIS_URL").expect("REDIS_URL must be set")).unwrap();
        let manager = RedisConnectionManager::new("redis://localhost:6379").unwrap();
        let redis_pool = r2d2::Pool::builder()
            .build(manager)
            .unwrap();
        AppData {
            db_pool,
            redis_pool,
        }
    }

    pub fn database_pool(&self) -> &DatabasePool {
        &self.db_pool
    }

    pub fn redis_pool(&self) -> &RedisPool {
        &self.redis_pool
    }
}